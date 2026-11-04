//! C-CDA conformance of CDA documents, by the templates' StructureDefinitions
//! (hl7.cda.us.ccda): each element that carries a template id is checked against the profile with
//! that identifier — element cardinality, fixed and pattern values, and the profile's invariants
//! (FHIRPath, `hasTemplateIdOf()` included).
//!
//! Not checked: slices (which entries a section must hold), terminology bindings, and invariants
//! that fail to evaluate (reported apart). It is a measurement for the corpus, not a replacement
//! for a full CDA validator.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Arc;

use octofhir_fhir_model::EmptyModelProvider;
use octofhir_fhirpath::evaluator::functions::{
    HasTemplateIdOfFunctionEvaluator, TemplateId, parse_template_id,
};
use octofhir_fhirpath::{
    Collection, EvaluationContext, FhirPathEngine, FhirPathValue, create_function_registry,
};
use serde_json::Value;

/// A rule of a profile, at a path below the element the template is on.
#[derive(Debug, Clone)]
enum Rule {
    Cardinality {
        path: Vec<String>,
        min: usize,
        max: Option<usize>,
    },
    Pattern {
        path: Vec<String>,
        value: Value,
    },
    Invariant {
        path: Vec<String>,
        key: String,
        expression: String,
        human: String,
    },
}

#[derive(Debug, Clone)]
struct Profile {
    name: String,
    templates: Vec<TemplateId>,
    rules: Vec<Rule>,
}

/// The outcome over a set of documents.
#[derive(Debug, Clone, Default)]
pub struct CdaValidity {
    pub documents: usize,
    pub valid: usize,
    /// Templated elements checked, and those with no error.
    pub elements: usize,
    pub valid_elements: usize,
    pub errors: BTreeMap<String, usize>,
    /// Invariants that could not be evaluated.
    pub unevaluated: BTreeMap<String, usize>,
}

impl CdaValidity {
    pub fn add(&mut self, other: &CdaValidity) {
        self.documents += other.documents;
        self.valid += other.valid;
        self.elements += other.elements;
        self.valid_elements += other.valid_elements;
        for (k, v) in &other.errors {
            *self.errors.entry(k.clone()).or_default() += v;
        }
        for (k, v) in &other.unevaluated {
            *self.unevaluated.entry(k.clone()).or_default() += v;
        }
    }
}

pub struct CdaValidator {
    profiles: Vec<Profile>,
    engine: FhirPathEngine,
}

impl CdaValidator {
    /// Loads the template profiles of a hl7.cda.us.ccda package folder.
    pub async fn new(package: &Path) -> Result<Self, String> {
        let folder = if package.join("package").is_dir() {
            package.join("package")
        } else {
            package.to_owned()
        };
        let mut profiles = Vec::new();
        let mut resolver: HashMap<String, Vec<TemplateId>> = HashMap::new();
        for entry in std::fs::read_dir(&folder).map_err(|e| format!("{}: {e}", folder.display()))? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_none_or(|e| e != "json") {
                continue;
            }
            let Ok(sd) =
                serde_json::from_str::<Value>(&std::fs::read_to_string(&path).unwrap_or_default())
            else {
                continue;
            };
            if sd["resourceType"] != "StructureDefinition" || sd["derivation"] != "constraint" {
                continue;
            }
            let templates: Vec<TemplateId> = sd["identifier"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|i| i["value"].as_str().and_then(parse_template_id))
                .collect();
            if templates.is_empty() {
                continue;
            }
            if let Some(url) = sd["url"].as_str() {
                resolver.insert(url.to_owned(), templates.clone());
            }
            profiles.push(Profile {
                name: sd["name"].as_str().unwrap_or_default().to_owned(),
                templates,
                rules: rules(&sd),
            });
        }
        let mut registry = create_function_registry();
        registry.register_pure_function(HasTemplateIdOfFunctionEvaluator::with_resolver(Arc::new(
            resolver,
        )));
        let engine = FhirPathEngine::new(Arc::new(registry), Arc::new(EmptyModelProvider))
            .await
            .map_err(|e| e.to_string())?;
        Ok(Self { profiles, engine })
    }

    pub fn profiles(&self) -> usize {
        self.profiles.len()
    }

    /// Checks a document (JSON of the CDA model).
    pub async fn validate(&self, document: &Value) -> CdaValidity {
        let mut validity = CdaValidity {
            documents: 1,
            ..CdaValidity::default()
        };
        let mut elements = Vec::new();
        collect_templated(document, &mut elements);
        let mut document_valid = true;
        for element in elements {
            let ids = template_ids(element);
            let mut element_valid = true;
            for profile in &self.profiles {
                let applies = profile.templates.iter().any(|(root, extension)| {
                    ids.iter().any(|(r, e)| {
                        r == root && (extension.is_none() || e.as_deref() == extension.as_deref())
                    })
                });
                if !applies {
                    continue;
                }
                for rule in &profile.rules {
                    if let Some(error) = self.check(element, rule, &mut validity).await {
                        element_valid = false;
                        *validity
                            .errors
                            .entry(format!("{}: {error}", profile.name))
                            .or_default() += 1;
                    }
                }
            }
            validity.elements += 1;
            if element_valid {
                validity.valid_elements += 1;
            } else {
                document_valid = false;
            }
        }
        if document_valid {
            validity.valid = 1;
        }
        validity
    }

    async fn check(
        &self,
        element: &Value,
        rule: &Rule,
        validity: &mut CdaValidity,
    ) -> Option<String> {
        match rule {
            Rule::Cardinality { path, min, max } => {
                let (parent, last) = path.split_at(path.len() - 1);
                for instance in values_at(element, parent) {
                    let count = values_at(instance, last).len();
                    if count < *min {
                        return Some(format!(
                            "{} needs at least {min}, has {count}",
                            path.join(".")
                        ));
                    }
                    if max.is_some_and(|max| count > max) {
                        return Some(format!(
                            "{} allows at most {}, has {count}",
                            path.join("."),
                            max.unwrap_or(0)
                        ));
                    }
                }
                None
            }
            Rule::Pattern { path, value } => {
                for found in values_at(element, path) {
                    if !matches_pattern(found, value) {
                        return Some(format!("{} must be {value}, is {found}", path.join(".")));
                    }
                }
                None
            }
            Rule::Invariant {
                path,
                key,
                expression,
                human,
            } => {
                for focus in values_at(element, path) {
                    let input = FhirPathValue::resource(focus.clone());
                    let context = EvaluationContext::new(
                        Collection::single(input),
                        Arc::new(EmptyModelProvider),
                        None,
                        None,
                        None,
                    );
                    match self.engine.evaluate(expression, &context).await {
                        Ok(result) => {
                            let ok = result.value.is_empty()
                                || result
                                    .value
                                    .iter()
                                    .all(|v| !matches!(v, FhirPathValue::Boolean(false, _, _)));
                            if !ok {
                                return Some(format!("{key}: {human}"));
                            }
                        }
                        Err(_) => {
                            *validity.unevaluated.entry(key.clone()).or_default() += 1;
                        }
                    }
                }
                None
            }
        }
    }
}

/// The rules of a profile's snapshot: elements outside slices, below the profile's root.
fn rules(sd: &Value) -> Vec<Rule> {
    let mut out = Vec::new();
    let Some(elements) = sd["snapshot"]["element"].as_array() else {
        return out;
    };
    let Some(root) = elements.first().and_then(|e| e["path"].as_str()) else {
        return out;
    };
    for element in elements {
        let id = element["id"].as_str().unwrap_or_default();
        if id.contains(':') {
            continue;
        }
        let Some(path) = element["path"].as_str() else {
            continue;
        };
        let relative: Vec<String> = if path == root {
            Vec::new()
        } else {
            match path.strip_prefix(root).and_then(|p| p.strip_prefix('.')) {
                Some(rest) => rest
                    .split('.')
                    .map(|s| s.trim_end_matches("[x]").to_owned())
                    .collect(),
                None => continue,
            }
        };
        // The XML's own name for the element is the model's; the JSON uses the element name.
        if !relative.is_empty() {
            let min = element["min"].as_u64().unwrap_or(0) as usize;
            let max = match element["max"].as_str() {
                Some("*") | None => None,
                Some(max) => max.parse().ok(),
            };
            if min > 0 || max.is_some_and(|m| m < 1) {
                out.push(Rule::Cardinality {
                    path: relative.clone(),
                    min,
                    max,
                });
            }
            if let Some((_, value)) = element.as_object().and_then(|o| {
                o.iter()
                    .find(|(k, _)| k.starts_with("pattern") || k.starts_with("fixed"))
            }) {
                out.push(Rule::Pattern {
                    path: relative.clone(),
                    value: value.clone(),
                });
            }
        }
        for constraint in element["constraint"].as_array().into_iter().flatten() {
            if constraint["severity"] != "error" {
                continue;
            }
            // The base model's own constraints come with every profile; checked once per profile
            // is enough, and they are not C-CDA's.
            let source = constraint["source"].as_str().unwrap_or_default();
            if source.starts_with("http://hl7.org/cda/stds/core")
                || source.starts_with("http://hl7.org/fhir/StructureDefinition")
            {
                continue;
            }
            if let (Some(key), Some(expression)) = (
                constraint["key"].as_str(),
                constraint["expression"].as_str(),
            ) {
                out.push(Rule::Invariant {
                    path: relative.clone(),
                    key: key.to_owned(),
                    expression: expression.to_owned(),
                    human: constraint["human"].as_str().unwrap_or_default().to_owned(),
                });
            }
        }
    }
    out
}

fn values_at<'a>(value: &'a Value, path: &[String]) -> Vec<&'a Value> {
    let mut current = vec![value];
    for segment in path {
        let mut next = Vec::new();
        for v in current {
            match v.get(segment) {
                Some(Value::Array(items)) => next.extend(items.iter()),
                Some(Value::Null) | None => {}
                Some(found) => next.push(found),
            }
        }
        current = next;
    }
    current
}

/// A FHIR pattern: every part present with the same value; a string pattern against a value read
/// as a number compares as text.
fn matches_pattern(value: &Value, pattern: &Value) -> bool {
    match (value, pattern) {
        (Value::Object(v), Value::Object(p)) => p
            .iter()
            .all(|(k, pv)| v.get(k).is_some_and(|vv| matches_pattern(vv, pv))),
        (Value::Array(items), p) => items.iter().any(|i| matches_pattern(i, p)),
        (Value::Number(n), Value::String(s)) => n.to_string() == *s,
        (v, p) => v == p,
    }
}

fn template_ids(element: &Value) -> Vec<(String, Option<String>)> {
    element["templateId"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|t| {
            Some((
                t["root"].as_str()?.to_owned(),
                t["extension"].as_str().map(str::to_owned),
            ))
        })
        .collect()
}

fn collect_templated<'a>(value: &'a Value, out: &mut Vec<&'a Value>) {
    match value {
        Value::Object(object) => {
            if object.get("templateId").is_some_and(Value::is_array) {
                out.push(value);
            }
            for child in object.values() {
                collect_templated(child, out);
            }
        }
        Value::Array(items) => items.iter().for_each(|i| collect_templated(i, out)),
        _ => {}
    }
}
