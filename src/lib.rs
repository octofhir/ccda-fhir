//! C-CDA ⇄ FHIR R4, both ways, on FHIR Mapping Language maps.
//!
//! The conversion is declarative: StructureMaps in FML (`maps/`) over the HL7 CDA logical model
//! (hl7.cda.uv.core) and the C-CDA templates (hl7.cda.us.ccda), run by the fml-rs engine. XML is
//! read into the model and written back by `fml-xml`. Terminology goes through the ConceptMaps
//! of C-CDA on FHIR and an OID ⇄ URI map from HL7 Terminology. Everything is embedded: a
//! [`Converter`] needs no files and no network.
//!
//! ```no_run
//! # async fn example(xml: &str) -> Result<(), ccda_fhir::Error> {
//! let converter = ccda_fhir::Converter::new().await?;
//! let fhir = converter.to_fhir(xml, &ccda_fhir::Options::default()).await?;
//! println!("{}", fhir.bundle);
//! for loss in &fhir.report.losses {
//!     eprintln!("{loss}");
//! }
//! let cda = converter.to_cda(&fhir.bundle, &ccda_fhir::Options::default()).await?;
//! println!("{}", cda.xml);
//! # Ok(()) }
//! ```
//!
//! What a conversion cannot carry over is reported, never dropped silently: elements and
//! attributes outside the model, templates the maps do not know, values a map cannot place,
//! narrative markup without a counterpart.

pub mod cda_validation;
mod embedded {
    include!(concat!(env!("OUT_DIR"), "/embedded.rs"));
}
pub mod metrics;
pub mod narrative;
mod prepare;
mod report;
pub mod validation;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use fml_check::{CoreDefinitions, Definitions, PackageDefinitions, Release, TemplateId};
use fml_engine::{ConceptMapSet, Engine, FhirPathExpressions, MapSet, RandomIds, SequentialIds};
use fml_model::model::StructureMap;
use serde_json::Value;

pub use report::{Coverage, Loss, Report, SectionCoverage, Stage};

/// The type of a CDA document's root.
pub const CLINICAL_DOCUMENT: &str =
    "http://hl7.org/cda/stds/core/StructureDefinition/ClinicalDocument";
/// The map from a CDA document to a FHIR document Bundle.
pub const TO_FHIR: &str = "http://octofhir.com/ccda-fhir/StructureMap/ccda-to-fhir";
/// The map from a FHIR document Bundle to a CDA document.
pub const TO_CDA: &str = "http://octofhir.com/ccda-fhir/StructureMap/fhir-to-ccda";

/// Something that stops a conversion: input that is not a document, a map that fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub message: String,
}

impl Error {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

/// How to convert.
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// Resource ids and UUIDs numbered from 1 instead of random: the same input gives the same
    /// output (tests, round trips, diffs).
    pub deterministic_ids: bool,
}

/// A CDA document converted to FHIR.
#[derive(Debug, Clone)]
pub struct ToFhir {
    /// A FHIR document Bundle: the Composition first, then the resources it refers to.
    pub bundle: Value,
    pub report: Report,
}

/// FHIR converted to a CDA document.
#[derive(Debug, Clone)]
pub struct ToCda {
    pub xml: String,
    /// The document as JSON of the CDA model, as the map made it.
    pub document: Value,
    pub report: Report,
}

/// Maps, definitions and terminology, ready to convert documents.
pub struct Converter {
    definitions: PackageDefinitions<CoreDefinitions>,
    model: fml_xml::Model,
    maps: MapSet,
    by_url: HashMap<String, StructureMap>,
    concept_maps: ConceptMapSet,
    expressions: FhirPathExpressions,
    /// Template roots the maps route on: what coverage counts as recognised.
    supported: HashSet<String>,
}

impl Converter {
    /// A converter with the embedded maps.
    pub async fn new() -> Result<Self, Error> {
        let maps = embedded::MAPS
            .iter()
            .map(|(name, text)| ((*name).to_owned(), (*text).to_owned()))
            .collect();
        Self::with_maps(maps).await
    }

    /// A converter with other maps (`name`, FML text) — for working on the maps without
    /// rebuilding.
    pub async fn with_maps(sources: Vec<(String, String)>) -> Result<Self, Error> {
        let mut definitions = PackageDefinitions::new(CoreDefinitions::new(Release::R4));
        let mut model = fml_xml::Model::new();
        let resources: Vec<Value> = serde_json::from_str(embedded::MODELS)
            .map_err(|e| Error::new(format!("embedded models: {e}")))?;
        for resource in &resources {
            definitions
                .add(resource)
                .map_err(|e| Error::new(format!("embedded models: {e}")))?;
            model.add(resource);
        }
        let mut maps = MapSet::new();
        let mut by_url = HashMap::new();
        let mut supported = HashSet::new();
        for (name, text) in &sources {
            let compiled = fml_model::compile(text);
            if compiled.has_errors() {
                let messages: Vec<String> = compiled
                    .diagnostics
                    .iter()
                    .map(|d| d.message.clone())
                    .collect();
                return Err(Error::new(format!("{name}: {}", messages.join("; "))));
            }
            for structure in &compiled.map.structure {
                for template in definitions.templates(&structure.url) {
                    supported.insert(template.root);
                }
            }
            // Templates the map routes on by OID (`hasTemplateIdOf('urn:oid:…')`).
            for (start, _) in text.match_indices("hasTemplateIdOf('urn:oid:") {
                let rest = &text[start + "hasTemplateIdOf('urn:oid:".len()..];
                if let Some(end) = rest.find('\'') {
                    supported.insert(rest[..end].to_owned());
                }
            }
            if let Some(url) = &compiled.map.url {
                by_url.insert(url.clone(), compiled.map.clone());
            }
            maps.add(compiled.map);
        }
        let mut concept_maps = ConceptMapSet::new();
        for (name, text) in embedded::CONCEPT_MAPS {
            let json: Value =
                serde_json::from_str(text).map_err(|e| Error::new(format!("{name}: {e}")))?;
            concept_maps.add(json);
        }
        let templates: HashMap<String, Vec<TemplateId>> = definitions.template_profiles().clone();
        let expressions = FhirPathExpressions::with_templates(Release::R4, templates)
            .await
            .map_err(Error::new)?;
        Ok(Self {
            definitions,
            model,
            maps,
            by_url,
            concept_maps,
            expressions,
            supported,
        })
    }

    /// The definitions the maps are checked and run against: FHIR R4, the CDA model and the
    /// C-CDA templates.
    pub fn definitions(&self) -> &dyn Definitions {
        &self.definitions
    }

    /// The XML form of the CDA model.
    pub fn model(&self) -> &fml_xml::Model {
        &self.model
    }

    /// Template roots the maps recognise.
    pub fn supported_templates(&self) -> &HashSet<String> {
        &self.supported
    }

    async fn run(
        &self,
        url: &str,
        source: &Value,
        options: &Options,
    ) -> Result<fml_engine::Transformed, Error> {
        let map = self
            .by_url
            .get(url)
            .ok_or_else(|| Error::new(format!("no map {url}")))?;
        let sequential = SequentialIds::default();
        let random = RandomIds;
        let ids: &dyn fml_engine::IdGenerator = if options.deterministic_ids {
            &sequential
        } else {
            &random
        };
        Engine::new(&self.definitions, &self.expressions)
            .with_maps(&self.maps)
            .with_concept_maps(&self.concept_maps)
            .with_ids(ids)
            .with_template_versions(false)
            .with_lenient_translations(true)
            .transform(map, source)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }

    /// A CDA document (XML) as a FHIR document Bundle.
    pub async fn to_fhir(&self, xml: &str, options: &Options) -> Result<ToFhir, Error> {
        let document = fml_xml::read(xml, &self.model, Some(CLINICAL_DOCUMENT))
            .map_err(|e| Error::new(e.to_string()))?;
        let mut report = Report::default();
        for issue in &document.issues {
            report.losses.push(Loss::from_xml(Stage::Read, issue));
        }
        let mut json = document.json;
        prepare::resolve_narrative_references(&mut json);
        report.coverage = report::coverage(&json, &self.supported);
        let transformed = self.run(TO_FHIR, &json, options).await?;
        for warning in &transformed.warnings {
            report
                .losses
                .push(Loss::new(Stage::Map, "", warning.clone()));
        }
        let mut bundle = transformed.target;
        prepare::deduplicate(&mut bundle);
        for loss in prepare::narratives_to_xhtml(&mut bundle) {
            report.losses.push(loss);
        }
        report::count_resources(&bundle, &mut report.coverage);
        Ok(ToFhir { bundle, report })
    }

    /// A FHIR document Bundle (or a Composition alone) as a CDA document.
    pub async fn to_cda(&self, input: &Value, options: &Options) -> Result<ToCda, Error> {
        let mut bundle = prepare::as_document_bundle(input)?;
        let mut report = Report::default();
        prepare::resolve_references(&mut bundle);
        for loss in prepare::narratives_to_cda(&mut bundle) {
            report.losses.push(loss);
        }
        let transformed = self.run(TO_CDA, &bundle, options).await?;
        for warning in &transformed.warnings {
            report
                .losses
                .push(Loss::new(Stage::Map, "", warning.clone()));
        }
        let document = transformed.target;
        let written = fml_xml::write(&document, &self.model, CLINICAL_DOCUMENT)
            .map_err(|e| Error::new(e.to_string()))?;
        for issue in &written.issues {
            report.losses.push(Loss::from_xml(Stage::Write, issue));
        }
        Ok(ToCda {
            xml: written.xml,
            document,
            report,
        })
    }

    /// Reads CDA XML into JSON of the CDA model: the root is the type `root` names (a URL or a
    /// type name), or the type whose root element name matches.
    pub fn read_cda(&self, xml: &str, root: Option<&str>) -> Result<fml_xml::Document, Error> {
        fml_xml::read(xml, &self.model, root).map_err(|e| Error::new(e.to_string()))
    }

    /// Runs any map over JSON of the CDA model (or FHIR) with the converter's definitions, maps
    /// (a map can call their groups) and ConceptMaps, with the debug trace when asked: a
    /// workbench for writing maps.
    pub async fn run_map(
        &self,
        map: &StructureMap,
        source: &Value,
        trace: bool,
    ) -> Result<fml_engine::Transformed, fml_engine::Error> {
        let ids = RandomIds;
        Engine::new(&self.definitions, &self.expressions)
            .with_maps(&self.maps)
            .with_concept_maps(&self.concept_maps)
            .with_ids(&ids)
            .with_template_versions(false)
            .with_lenient_translations(true)
            .with_trace(trace)
            .transform(map, source)
            .await
    }

    /// Checks every map against the definitions: element paths, types, groups.
    pub fn check_maps(
        sources: &[(String, String)],
    ) -> Result<BTreeMap<String, Vec<String>>, Error> {
        let mut definitions = PackageDefinitions::new(CoreDefinitions::new(Release::R4));
        let resources: Vec<Value> = serde_json::from_str(embedded::MODELS)
            .map_err(|e| Error::new(format!("embedded models: {e}")))?;
        for resource in &resources {
            definitions
                .add(resource)
                .map_err(|e| Error::new(format!("embedded models: {e}")))?;
        }
        let mut problems = BTreeMap::new();
        for (name, text) in sources {
            let parse = fml_syntax::parse(text);
            let index = fml_syntax::LineIndex::new(text);
            let mut messages = Vec::new();
            let diagnostics = if parse.has_errors() {
                parse.diagnostics().to_vec()
            } else {
                fml_check::check(&parse.tree(), &definitions)
            };
            for diagnostic in diagnostics {
                let at = index.line_col(text, diagnostic.range.start());
                messages.push(format!(
                    "{}:{}: {}",
                    at.line + 1,
                    at.col_utf16 + 1,
                    diagnostic.message
                ));
            }
            if !messages.is_empty() {
                problems.insert(name.clone(), messages);
            }
        }
        Ok(problems)
    }

    /// The embedded maps, by file name.
    pub fn embedded_maps() -> Vec<(String, String)> {
        embedded::MAPS
            .iter()
            .map(|(name, text)| ((*name).to_owned(), (*text).to_owned()))
            .collect()
    }
}

/// Shared converters cost one load; this is a convenience for callers that keep one.
pub type SharedConverter = Arc<Converter>;
