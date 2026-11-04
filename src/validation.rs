//! US Core validation of converted resources, with the fhirschema validator: the R4 core
//! definitions and the profiles of a US Core package, FHIRPath invariants included.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Arc;

use octofhir_fhirpath::{FhirPathEngine, create_function_registry};
use octofhir_fhirschema::embedded::{FhirVersion, get_schemas};
use octofhir_fhirschema::types::StructureDefinition;
use octofhir_fhirschema::{EmbeddedSchemaProvider, FhirSchema, FhirValidator, translate};
use serde_json::Value;

/// Resources checked against the profiles they claim (`meta.profile`), or their base type.
pub struct Validator {
    validator: FhirValidator,
    profiles: usize,
}

/// The outcome of validating a set of resources.
#[derive(Debug, Clone, Default)]
pub struct Validity {
    pub resources: usize,
    pub valid: usize,
    /// Error messages, by how often they occur.
    pub errors: BTreeMap<String, usize>,
}

impl Validator {
    /// Loads the StructureDefinitions of a package folder (US Core 6.1.0's `package` folder).
    pub async fn new(package: &Path) -> Result<Self, String> {
        let mut schemas: HashMap<String, Arc<FhirSchema>> = HashMap::new();
        for (name, schema) in get_schemas(FhirVersion::R4) {
            let schema = Arc::new(schema.clone());
            schemas.insert(name.clone(), schema.clone());
            schemas.insert(
                format!("http://hl7.org/fhir/StructureDefinition/{name}"),
                schema,
            );
        }
        let folder = if package.join("package").is_dir() {
            package.join("package")
        } else {
            package.to_owned()
        };
        let mut profiles = 0;
        for entry in std::fs::read_dir(&folder).map_err(|e| format!("{}: {e}", folder.display()))? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_none_or(|e| e != "json") {
                continue;
            }
            let Ok(json) =
                serde_json::from_str::<Value>(&std::fs::read_to_string(&path).unwrap_or_default())
            else {
                continue;
            };
            if json["resourceType"] != "StructureDefinition" {
                continue;
            }
            let Some(url) = json["url"].as_str().map(str::to_owned) else {
                continue;
            };
            let Ok(definition) = serde_json::from_value::<StructureDefinition>(json) else {
                continue;
            };
            if let Ok(schema) = translate(definition, None) {
                schemas.insert(url, Arc::new(schema));
                profiles += 1;
            }
        }
        let model = Arc::new(EmbeddedSchemaProvider::new(
            octofhir_fhirschema::ModelFhirVersion::R4,
        ));
        let engine = FhirPathEngine::new(Arc::new(create_function_registry()), model)
            .await
            .map_err(|e| e.to_string())?;
        Ok(Self {
            validator: FhirValidator::from_arc_schemas(schemas, Some(Arc::new(engine))),
            profiles,
        })
    }

    /// How many profiles the package brought.
    pub fn profiles(&self) -> usize {
        self.profiles
    }

    /// Validates the resources of a Bundle (the Bundle itself and the Composition aside).
    pub async fn validate_bundle(&self, bundle: &Value) -> Validity {
        let mut validity = Validity::default();
        for entry in bundle["entry"].as_array().into_iter().flatten() {
            let resource = &entry["resource"];
            let Some(kind) = resource["resourceType"].as_str() else {
                continue;
            };
            if kind == "Composition" {
                continue;
            }
            let profiles: Vec<String> = match resource["meta"]["profile"].as_array() {
                Some(profiles) => profiles
                    .iter()
                    .filter_map(|p| p.as_str().map(str::to_owned))
                    .collect(),
                None => vec![kind.to_owned()],
            };
            let result = self.validator.validate(resource, profiles).await;
            validity.resources += 1;
            if result.valid {
                validity.valid += 1;
            }
            for error in &result.errors {
                let message = error
                    .message
                    .clone()
                    .unwrap_or_else(|| error.error_type.clone());
                let path = error
                    .path
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(".");
                *validity
                    .errors
                    .entry(format!("{kind} {path}: {message}"))
                    .or_default() += 1;
            }
        }
        validity
    }
}

impl Validity {
    pub fn add(&mut self, other: &Validity) {
        self.resources += other.resources;
        self.valid += other.valid;
        for (message, count) in &other.errors {
            *self.errors.entry(message.clone()).or_default() += count;
        }
    }
}
