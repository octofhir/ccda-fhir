//! The `ccda-fhir` command.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use ccda_fhir::{Converter, Options};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ccda-fhir",
    version,
    about = "C-CDA ⇄ FHIR R4 on FHIR Mapping Language maps"
)]
struct Cli {
    /// Use the maps in this folder instead of the built-in ones (for working on the maps).
    #[arg(long, global = true)]
    maps: Option<PathBuf>,
    /// Number resource ids and UUIDs from 1, so the same input gives the same output.
    #[arg(long, global = true)]
    deterministic: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Convert a C-CDA document (XML) to a FHIR document Bundle.
    ToFhir {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Write the report (losses, coverage) here as JSON.
        #[arg(long)]
        report: Option<PathBuf>,
    },
    /// Convert a FHIR document Bundle (or a Composition) to a C-CDA document.
    ToCda {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long)]
        report: Option<PathBuf>,
    },
    /// Check the maps against FHIR R4, the CDA model and the C-CDA templates.
    Check,
    /// Measure a corpus of C-CDA documents: coverage, round trips, losses.
    Coverage {
        /// Files or folders of XML documents.
        #[arg(required = true)]
        inputs: Vec<PathBuf>,
        /// Write the report (Markdown) here.
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Also write the measurements as JSON here.
        #[arg(long)]
        json: Option<PathBuf>,
        /// Also compare with the consensus outputs of C-CDA on FHIR: the
        /// `mappings/examples/CDAtoFHIRExamples` folder of HL7/ccda-on-fhir.
        #[arg(long)]
        consensus: Option<PathBuf>,
        /// Validate the resources against the US Core package in this folder (6.1.0).
        #[arg(long)]
        us_core: Option<PathBuf>,
        /// Check the CDA written back against the C-CDA templates in this folder
        /// (hl7.cda.us.ccda 5.0.0).
        #[arg(long)]
        ccda: Option<PathBuf>,
    },
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("error: cannot start the runtime: {error}");
            return ExitCode::from(2);
        }
    };
    match runtime.block_on(run(cli)) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

fn map_sources(folder: Option<&Path>) -> Result<Vec<(String, String)>> {
    let Some(folder) = folder else {
        return Ok(Converter::embedded_maps());
    };
    let mut out = Vec::new();
    collect(folder, folder, &mut out)?;
    out.sort();
    Ok(out)
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect(root, &path, out)?;
        } else if path.extension().is_some_and(|e| e == "map") {
            let name = path
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/");
            out.push((name, std::fs::read_to_string(&path)?));
        }
    }
    Ok(())
}

fn write(output: Option<&Path>, text: &str) -> Result<()> {
    match output {
        Some(path) => std::fs::write(path, text)?,
        None => std::io::stdout().write_all(text.as_bytes())?,
    }
    Ok(())
}

async fn run(cli: Cli) -> Result<ExitCode> {
    let sources = map_sources(cli.maps.as_deref())?;
    let options = Options {
        deterministic_ids: cli.deterministic,
    };
    match cli.command {
        Command::Check => {
            let problems = Converter::check_maps(&sources)?;
            for (file, messages) in &problems {
                for message in messages {
                    println!("{file}:{message}");
                }
            }
            eprintln!(
                "checked {} map(s): {} with problems",
                sources.len(),
                problems.len()
            );
            Ok(if problems.is_empty() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
        Command::ToFhir {
            input,
            output,
            report,
        } => {
            let converter = Converter::with_maps(sources).await?;
            let xml = std::fs::read_to_string(&input)?;
            let result = converter.to_fhir(&xml, &options).await?;
            for loss in &result.report.losses {
                eprintln!("warning: {}: {loss}", input.display());
            }
            for (template, count) in &result.report.coverage.unknown_templates {
                eprintln!(
                    "warning: {}: template {template} not mapped ({count}×)",
                    input.display()
                );
            }
            if let Some(path) = report {
                std::fs::write(
                    path,
                    serde_json::to_string_pretty(&result.report.to_json())? + "\n",
                )?;
            }
            write(
                output.as_deref(),
                &(serde_json::to_string_pretty(&result.bundle)? + "\n"),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Command::ToCda {
            input,
            output,
            report,
        } => {
            let converter = Converter::with_maps(sources).await?;
            let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&input)?)?;
            let result = converter.to_cda(&json, &options).await?;
            for loss in &result.report.losses {
                eprintln!("warning: {}: {loss}", input.display());
            }
            if let Some(path) = report {
                std::fs::write(
                    path,
                    serde_json::to_string_pretty(&result.report.to_json())? + "\n",
                )?;
            }
            write(output.as_deref(), &result.xml)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Coverage {
            inputs,
            output,
            json,
            consensus,
            us_core,
            ccda,
        } => {
            let converter = Converter::with_maps(sources).await?;
            let validator = match &us_core {
                Some(folder) => Some(ccda_fhir::validation::Validator::new(folder).await?),
                None => None,
            };
            let ccda_validator = match &ccda {
                Some(folder) => Some(ccda_fhir::cda_validation::CdaValidator::new(folder).await?),
                None => None,
            };
            let validators = ccda_fhir::metrics::Validators {
                us_core: validator.as_ref(),
                ccda: ccda_validator.as_ref(),
            };
            let files = ccda_fhir::metrics::corpus_files(&inputs);
            let mut all = Vec::with_capacity(files.len());
            for (index, file) in files.iter().enumerate() {
                let Ok(xml) = std::fs::read_to_string(file) else {
                    continue;
                };
                // Not CDA (IG configuration, stylesheets): skipped.
                if !xml.contains("urn:hl7-org:v3")
                    && !xml.contains("<section")
                    && !xml.contains("<ClinicalDocument")
                {
                    continue;
                }
                eprint!("\r{}/{} ", index + 1, files.len());
                all.push(
                    ccda_fhir::metrics::measure_with(&converter, file, &xml, &validators).await,
                );
            }
            eprintln!();
            let summary = ccda_fhir::metrics::summarize(&all);
            let root = (inputs.len() == 1).then(|| inputs[0].as_path());
            let mut report = ccda_fhir::metrics::markdown(&summary, &all, root);
            if let Some(examples) = consensus {
                let results = ccda_fhir::metrics::consensus(&converter, &examples).await?;
                report.push('\n');
                report.push_str(&ccda_fhir::metrics::consensus_markdown(&results));
            }
            write(output.as_deref(), &report)?;
            if let Some(path) = json {
                std::fs::write(
                    path,
                    serde_json::to_string_pretty(&ccda_fhir::metrics::to_json(&summary, &all))?
                        + "\n",
                )?;
            }
            Ok(ExitCode::SUCCESS)
        }
    }
}
