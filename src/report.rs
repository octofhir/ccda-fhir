//! What a conversion carried over and what it did not.

use std::collections::{BTreeMap, HashSet};

use serde_json::{Value, json};

/// Where in the conversion something was lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Stage {
    /// Reading XML into the CDA model.
    Read,
    /// Running the maps.
    Map,
    /// Converting narrative between CDA and XHTML.
    Narrative,
    /// Writing the CDA model as XML.
    Write,
}

impl Stage {
    pub fn name(self) -> &'static str {
        match self {
            Stage::Read => "read",
            Stage::Map => "map",
            Stage::Narrative => "narrative",
            Stage::Write => "write",
        }
    }
}

/// Content that did not make it into the output, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loss {
    pub stage: Stage,
    /// Where in the input: a path in the CDA model (`ClinicalDocument.component…`) or in the
    /// FHIR input.
    pub path: String,
    /// Line and column in the XML input.
    pub position: Option<fml_xml::Position>,
    pub message: String,
}

impl Loss {
    pub fn new(stage: Stage, path: &str, message: String) -> Self {
        Self {
            stage,
            path: path.to_owned(),
            position: None,
            message,
        }
    }

    pub(crate) fn from_xml(stage: Stage, issue: &fml_xml::Issue) -> Self {
        Self {
            stage,
            path: issue.path.clone(),
            position: issue.position,
            message: issue.message.clone(),
        }
    }

    pub fn to_json(&self) -> Value {
        let mut out = json!({ "stage": self.stage.name(), "message": self.message });
        if !self.path.is_empty() {
            out["path"] = json!(self.path);
        }
        if let Some(position) = self.position {
            out["line"] = json!(position.line);
            out["column"] = json!(position.column);
        }
        out
    }
}

impl std::fmt::Display for Loss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stage.name())?;
        if let Some(position) = self.position {
            write!(f, " {position}")?;
        }
        if !self.path.is_empty() {
            write!(f, " {}", self.path)?;
        }
        write!(f, ": {}", self.message)
    }
}

/// A section of the document: what it is and how much of it became resources.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SectionCoverage {
    /// The section's template ids (`root` or `root:extension`).
    pub templates: Vec<String>,
    pub title: Option<String>,
    /// One of its templates is one the maps know.
    pub recognised: bool,
    pub entries: usize,
    /// Entries with a template the maps know.
    pub recognised_entries: usize,
    /// Resources the converted section refers to.
    pub resources: usize,
}

/// How much of a document the maps recognised and converted. A section or entry is recognised
/// by any of its template ids, whatever their version.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Coverage {
    pub sections: Vec<SectionCoverage>,
    /// Templates of sections and entries nothing recognised, with how often they occur.
    pub unknown_templates: BTreeMap<String, usize>,
}

impl Coverage {
    pub fn entries(&self) -> usize {
        self.sections.iter().map(|s| s.entries).sum()
    }

    pub fn recognised_entries(&self) -> usize {
        self.sections.iter().map(|s| s.recognised_entries).sum()
    }

    pub fn resources(&self) -> usize {
        self.sections.iter().map(|s| s.resources).sum()
    }

    pub fn to_json(&self) -> Value {
        json!({
            "sections": self.sections.iter().map(|s| json!({
                "templates": s.templates,
                "title": s.title,
                "recognised": s.recognised,
                "entries": s.entries,
                "recognisedEntries": s.recognised_entries,
                "resources": s.resources,
            })).collect::<Vec<_>>(),
            "unknownTemplates": self.unknown_templates,
        })
    }
}

/// The losses and coverage of one conversion.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    pub losses: Vec<Loss>,
    pub coverage: Coverage,
}

impl Report {
    pub fn to_json(&self) -> Value {
        json!({
            "losses": self.losses.iter().map(Loss::to_json).collect::<Vec<_>>(),
            "coverage": self.coverage.to_json(),
        })
    }

    /// The report as an OperationOutcome: one `warning` issue per loss, with the CDA path in
    /// `expression` and the XML position in `location` (`line N, column M`).
    pub fn to_operation_outcome(&self) -> Value {
        let mut issues: Vec<Value> = self
            .losses
            .iter()
            .map(|loss| {
                let mut issue = json!({
                    "severity": "warning",
                    "code": if loss.stage == Stage::Read { "not-supported" } else { "incomplete" },
                    "diagnostics": format!("{}: {}", loss.stage.name(), loss.message),
                });
                if !loss.path.is_empty() {
                    issue["expression"] = json!([loss.path]);
                }
                if let Some(position) = loss.position {
                    issue["location"] = json!([format!(
                        "line {}, column {}",
                        position.line, position.column
                    )]);
                }
                issue
            })
            .collect();
        for (template, count) in &self.coverage.unknown_templates {
            issues.push(json!({
                "severity": "warning",
                "code": "not-supported",
                "diagnostics": format!("template {template} is not mapped ({count} time(s)); its content is not in the result"),
            }));
        }
        if issues.is_empty() {
            issues.push(json!({ "severity": "information", "code": "informational", "diagnostics": "converted without losses" }));
        }
        json!({ "resourceType": "OperationOutcome", "issue": issues })
    }
}

/// The template ids of a CDA element (`root` or `root:extension`).
fn templates(element: &Value) -> Vec<String> {
    element["templateId"]
        .as_array()
        .map(|ids| {
            ids.iter()
                .filter_map(|id| {
                    let root = id["root"].as_str()?;
                    Some(match id["extension"].as_str() {
                        Some(extension) => format!("{root}:{extension}"),
                        None => root.to_owned(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn recognised(templates: &[String], supported: &HashSet<String>) -> bool {
    templates
        .iter()
        .any(|t| supported.contains(t.split(':').next().unwrap_or(t)))
}

/// The sections of a document in document order, nested ones after their parent.
pub(crate) fn sections(document: &Value) -> Vec<&Value> {
    fn walk<'a>(section: &'a Value, out: &mut Vec<&'a Value>) {
        out.push(section);
        for component in section["component"].as_array().into_iter().flatten() {
            if component["section"].is_object() {
                walk(&component["section"], out);
            }
        }
    }
    let mut out = Vec::new();
    for component in document["component"]["structuredBody"]["component"]
        .as_array()
        .into_iter()
        .flatten()
    {
        if component["section"].is_object() {
            walk(&component["section"], &mut out);
        }
    }
    out
}

/// The clinical statement of an entry (`act`, `observation`, …).
pub(crate) fn statement(entry: &Value) -> Option<&Value> {
    entry.as_object()?.iter().find_map(|(key, value)| {
        matches!(
            key.as_str(),
            "act"
                | "encounter"
                | "observation"
                | "observationMedia"
                | "organizer"
                | "procedure"
                | "regionOfInterest"
                | "substanceAdministration"
                | "supply"
        )
        .then_some(value)
    })
}

pub(crate) fn coverage(document: &Value, supported: &HashSet<String>) -> Coverage {
    let mut coverage = Coverage::default();
    for section in sections(document) {
        let section_templates = templates(section);
        let mut item = SectionCoverage {
            recognised: recognised(&section_templates, supported),
            title: section["title"]["xmlText"]
                .as_str()
                .map(|t| t.trim().to_owned()),
            ..SectionCoverage::default()
        };
        if !item.recognised {
            for template in &section_templates {
                *coverage
                    .unknown_templates
                    .entry(template.clone())
                    .or_default() += 1;
            }
        }
        item.templates = section_templates;
        for entry in section["entry"].as_array().into_iter().flatten() {
            item.entries += 1;
            let Some(statement) = statement(entry) else {
                continue;
            };
            let entry_templates = templates(statement);
            if recognised(&entry_templates, supported) {
                item.recognised_entries += 1;
            } else {
                for template in &entry_templates {
                    *coverage
                        .unknown_templates
                        .entry(template.clone())
                        .or_default() += 1;
                }
            }
        }
        coverage.sections.push(item);
    }
    coverage
}

/// Counts the resources each converted section refers to; the maps keep one Composition section
/// per CDA section, in order.
pub(crate) fn count_resources(bundle: &Value, coverage: &mut Coverage) {
    let Some(composition) = bundle["entry"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|e| &e["resource"])
        .find(|r| r["resourceType"] == "Composition")
    else {
        return;
    };
    fn walk<'a>(section: &'a Value, out: &mut Vec<&'a Value>) {
        out.push(section);
        for nested in section["section"].as_array().into_iter().flatten() {
            walk(nested, out);
        }
    }
    let mut flat = Vec::new();
    for section in composition["section"].as_array().into_iter().flatten() {
        walk(section, &mut flat);
    }
    for (item, section) in coverage.sections.iter_mut().zip(flat) {
        item.resources = section["entry"].as_array().map_or(0, Vec::len);
    }
}
