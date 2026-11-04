//! What happens around the maps: narrative references and markup, document Bundles, references.

use std::collections::HashMap;

use serde_json::{Map, Value, json};

use crate::Error;
use crate::narrative;
use crate::report::{Loss, Stage};

/// Gives every `ED` that only refers to the narrative (`<reference value="#id"/>`) the text it
/// refers to, as C-CDA on FHIR says such references are resolved (`CodeableConcept.text` is a
/// string). The reference stays.
pub(crate) fn resolve_narrative_references(document: &mut Value) {
    let narratives = collect_narratives(document);
    resolve_in(document, &narratives);
}

/// Every `ID` in every section narrative, with its text.
fn collect_narratives(document: &Value) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for section in crate::report::sections(document) {
        let Some(markup) = section["text"].as_str() else {
            continue;
        };
        for (id, text) in narrative::texts_by_id(markup) {
            out.entry(id).or_insert(text);
        }
    }
    out
}

fn resolve_in(value: &mut Value, narratives: &HashMap<String, String>) {
    match value {
        Value::Object(object) => {
            let target = object
                .get("reference")
                .and_then(|r| r.get("value"))
                .and_then(Value::as_str)
                .and_then(|v| v.strip_prefix('#'))
                .map(str::to_owned);
            if let Some(id) = target
                && !object.contains_key("xmlText")
                && let Some(text) = narratives.get(&id)
            {
                object.insert("xmlText".to_owned(), Value::String(text.clone()));
            }
            for child in object.values_mut() {
                resolve_in(child, narratives);
            }
        }
        Value::Array(items) => {
            for item in items {
                resolve_in(item, narratives);
            }
        }
        _ => {}
    }
}

/// Converts the CDA narrative the maps copy into `Narrative.div` to XHTML.
pub(crate) fn narratives_to_xhtml(bundle: &mut Value) -> Vec<Loss> {
    let mut losses = Vec::new();
    walk_divs(bundle, "Bundle", &mut |path, div| {
        if !div.starts_with("<text") {
            return None;
        }
        match narrative::cda_to_xhtml(div) {
            Ok((xhtml, lost)) => {
                losses.extend(
                    lost.into_iter()
                        .map(|l| Loss::new(Stage::Narrative, path, l.message)),
                );
                Some(xhtml)
            }
            Err(message) => {
                losses.push(Loss::new(Stage::Narrative, path, message));
                None
            }
        }
    });
    losses
}

/// Converts the XHTML of Composition sections to CDA narrative, for the maps to copy into
/// `section/text`.
pub(crate) fn narratives_to_cda(bundle: &mut Value) -> Vec<Loss> {
    let mut losses = Vec::new();
    let Some(entries) = bundle["entry"].as_array_mut() else {
        return losses;
    };
    for entry in entries {
        if entry["resource"]["resourceType"] != "Composition" {
            continue;
        }
        walk_divs(&mut entry["resource"], "Composition", &mut |path, div| {
            if !div.trim_start().starts_with("<div") {
                return None;
            }
            match narrative::xhtml_to_cda(div) {
                Ok((cda, lost)) => {
                    losses.extend(
                        lost.into_iter()
                            .map(|l| Loss::new(Stage::Narrative, path, l.message)),
                    );
                    Some(cda)
                }
                Err(message) => {
                    losses.push(Loss::new(Stage::Narrative, path, message));
                    None
                }
            }
        });
    }
    losses
}

/// Calls `convert` with every `div` string; a returned value replaces it.
fn walk_divs(value: &mut Value, path: &str, convert: &mut dyn FnMut(&str, &str) -> Option<String>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object.iter_mut() {
                let child_path = format!("{path}.{key}");
                if key == "div"
                    && let Value::String(div) = child
                {
                    if let Some(converted) = convert(&child_path, div) {
                        *div = converted;
                    }
                    continue;
                }
                walk_divs(child, &child_path, convert);
            }
        }
        Value::Array(items) => {
            for (index, item) in items.iter_mut().enumerate() {
                walk_divs(item, &format!("{path}[{index}]"), convert);
            }
        }
        _ => {}
    }
}

/// A document Bundle from a Bundle, or from a Composition alone.
pub(crate) fn as_document_bundle(input: &Value) -> Result<Value, Error> {
    match input["resourceType"].as_str() {
        Some("Bundle") => {
            let first = &input["entry"][0]["resource"];
            if first["resourceType"] != "Composition" {
                return Err(Error::new(
                    "the Bundle's first entry must be a Composition (a FHIR document)",
                ));
            }
            Ok(input.clone())
        }
        Some("Composition") => {
            let id = input["id"].as_str().unwrap_or("composition");
            Ok(json!({
                "resourceType": "Bundle",
                "type": "document",
                "entry": [{ "fullUrl": format!("Composition/{id}"), "resource": input }],
            }))
        }
        _ => Err(Error::new("expected a document Bundle or a Composition")),
    }
}

/// Points every reference inside the Bundle at the `fullUrl` of the entry it names, so maps can
/// follow references by comparing them with `fullUrl`: `Patient/1` becomes the entry's
/// `urn:uuid:…` or absolute URL. Entries without a `fullUrl` get `Type/id`.
pub(crate) fn resolve_references(bundle: &mut Value) {
    let mut by_reference: HashMap<String, String> = HashMap::new();
    if let Some(entries) = bundle["entry"].as_array_mut() {
        for entry in entries.iter_mut() {
            let resource = &entry["resource"];
            let local = match (resource["resourceType"].as_str(), resource["id"].as_str()) {
                (Some(kind), Some(id)) => Some(format!("{kind}/{id}")),
                _ => None,
            };
            let full = match entry["fullUrl"].as_str() {
                Some(full) => full.to_owned(),
                None => match &local {
                    Some(local) => {
                        entry["fullUrl"] = Value::String(local.clone());
                        local.clone()
                    }
                    None => continue,
                },
            };
            by_reference.insert(full.clone(), full.clone());
            if let Some(local) = local {
                by_reference.entry(local).or_insert(full);
            }
        }
    }
    fn rewrite(value: &mut Value, by_reference: &HashMap<String, String>) {
        match value {
            Value::Object(object) => {
                rewrite_object(object, by_reference);
            }
            Value::Array(items) => {
                for item in items {
                    rewrite(item, by_reference);
                }
            }
            _ => {}
        }
    }
    fn rewrite_object(object: &mut Map<String, Value>, by_reference: &HashMap<String, String>) {
        for (key, child) in object.iter_mut() {
            match child {
                Value::String(reference) if key == "reference" => {
                    if let Some(full) = by_reference.get(reference.as_str()) {
                        *reference = full.clone();
                    }
                }
                _ => rewrite(child, by_reference),
            }
        }
    }
    if let Some(entries) = bundle["entry"].as_array_mut() {
        for entry in entries {
            rewrite(&mut entry["resource"], &by_reference);
        }
    }
}

/// Resource types that describe who or what, written once per mention by the maps: identical
/// ones are merged.
const SHARED_TYPES: &[&str] = &[
    "Practitioner",
    "PractitionerRole",
    "Organization",
    "Device",
    "Location",
    "Medication",
];

/// Merges identical Practitioners, Organizations and the like (equal apart from their id) into
/// one entry and points the references at it. Merging repeats until nothing changes: a merged
/// Organization can make two PractitionerRoles equal.
pub(crate) fn deduplicate(bundle: &mut Value) {
    loop {
        let Some(entries) = bundle["entry"].as_array() else {
            return;
        };
        let mut first_by_content: HashMap<String, String> = HashMap::new();
        let mut replaced: HashMap<String, String> = HashMap::new();
        for entry in entries {
            let resource = &entry["resource"];
            let Some(kind) = resource["resourceType"].as_str() else {
                continue;
            };
            if !SHARED_TYPES.contains(&kind) {
                continue;
            }
            let Some(full) = entry["fullUrl"].as_str() else {
                continue;
            };
            let mut content = resource.clone();
            if let Some(object) = content.as_object_mut() {
                object.remove("id");
            }
            let key = content.to_string();
            match first_by_content.get(&key) {
                Some(kept) => {
                    replaced.insert(full.to_owned(), kept.clone());
                }
                None => {
                    first_by_content.insert(key, full.to_owned());
                }
            }
        }
        if replaced.is_empty() {
            return;
        }
        if let Some(entries) = bundle["entry"].as_array_mut() {
            entries.retain(|e| {
                !e["fullUrl"]
                    .as_str()
                    .is_some_and(|f| replaced.contains_key(f))
            });
            for entry in entries.iter_mut() {
                repoint(&mut entry["resource"], &replaced);
            }
        }
    }
}

fn repoint(value: &mut Value, replaced: &HashMap<String, String>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object.iter_mut() {
                match child {
                    Value::String(reference) if key == "reference" => {
                        if let Some(kept) = replaced.get(reference.as_str()) {
                            *reference = kept.clone();
                        }
                    }
                    _ => repoint(child, replaced),
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                repoint(item, replaced);
            }
        }
        _ => {}
    }
}
