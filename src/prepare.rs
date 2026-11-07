//! What happens around the maps: narrative references and markup, document Bundles, references.

use std::collections::HashMap;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde_json::{Map, Value, json};

use crate::Error;
use crate::narrative;
use crate::report::{Loss, Stage};

/// Gives every `ED` that only refers to the narrative (`<reference value="#id"/>`) the text it
/// refers to, as C-CDA on FHIR says such references are resolved (`CodeableConcept.text` is a
/// string). The reference stays.
pub(crate) fn resolve_narrative_references(document: &mut Value) {
    resolve_note_texts(document);
    let narratives = collect_narratives(document);
    resolve_in(document, &narratives);
}

/// The Note Activity template.
const NOTE_ACTIVITY: &str = "2.16.840.1.113883.10.20.22.4.202";

/// A Note Activity whose text refers to the narrative carries that part of the narrative as
/// XHTML (`mediaType` `application/xhtml+xml`): the note's content, which C-CDA on FHIR converts
/// to XHTML for `DocumentReference.content`, markup kept.
fn resolve_note_texts(document: &mut Value) {
    let mut markups: Vec<String> = Vec::new();
    for section in crate::report::sections(document) {
        if let Some(markup) = section["text"].as_str() {
            markups.push(markup.to_owned());
        }
    }
    fn walk(value: &mut Value, markups: &[String]) {
        match value {
            Value::Object(object) => {
                let is_note = object
                    .get("templateId")
                    .and_then(Value::as_array)
                    .is_some_and(|ids| ids.iter().any(|t| t["root"] == NOTE_ACTIVITY));
                if is_note && let Some(text) = object.get_mut("text") {
                    let id = text["reference"]["value"]
                        .as_str()
                        .and_then(|v| v.strip_prefix('#'))
                        .map(str::to_owned);
                    if let Some(id) = id
                        && text.get("mediaType").is_none()
                        && let Some(xhtml) = markups
                            .iter()
                            .find_map(|markup| narrative::xhtml_of_id(markup, &id))
                    {
                        text["mediaType"] = json!("application/xhtml+xml");
                        text["xmlText"] = json!(xhtml);
                    }
                }
                for child in object.values_mut() {
                    walk(child, markups);
                }
            }
            Value::Array(items) => items.iter_mut().for_each(|i| walk(i, markups)),
            _ => {}
        }
    }
    walk(document, &markups);
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

/// A Note Activity's text refers to the section narrative (`reference/@value`, required by the
/// template) while its content is inline: the narrative gets a paragraph with that ID and the
/// note's text (the text of markup, plain text as it is; other kinds of content are named).
pub(crate) fn link_note_narratives(document: &mut Value) {
    fn link_section(section: &mut Value) {
        let mut paragraphs = String::new();
        let existing = section["text"].as_str().unwrap_or_default().to_owned();
        for entry in section["entry"].as_array().into_iter().flatten() {
            let act = &entry["act"];
            let is_note = act["templateId"]
                .as_array()
                .is_some_and(|ids| ids.iter().any(|t| t["root"] == NOTE_ACTIVITY));
            if !is_note {
                continue;
            }
            let Some(id) = act["text"]["reference"]["value"]
                .as_str()
                .and_then(|v| v.strip_prefix('#'))
            else {
                continue;
            };
            if existing.contains(&format!("ID=\"{id}\"")) {
                continue;
            }
            paragraphs.push_str(&format!(
                "<paragraph ID=\"{id}\">{}</paragraph>",
                narrative::escape(&note_text(&act["text"]))
            ));
        }
        if !paragraphs.is_empty() {
            section["text"] = json!(match existing.rfind("</text>") {
                Some(end) => format!("{}{paragraphs}{}", &existing[..end], &existing[end..]),
                None => format!("<text>{paragraphs}</text>"),
            });
        }
        if let Some(components) = section.get_mut("component").and_then(Value::as_array_mut) {
            for component in components {
                if let Some(nested) = component.get_mut("section") {
                    link_section(nested);
                }
            }
        }
    }
    if let Some(components) = document
        .get_mut("component")
        .and_then(|c| c.get_mut("structuredBody"))
        .and_then(|b| b.get_mut("component"))
        .and_then(Value::as_array_mut)
    {
        for component in components {
            if let Some(s) = component.get_mut("section") {
                link_section(s);
            }
        }
    }
}

/// What a note's inline content says, as text.
fn note_text(text: &Value) -> String {
    let media = text["mediaType"].as_str().unwrap_or("text/plain");
    let raw = text["xmlText"].as_str().unwrap_or_default();
    let content = if text["representation"] == "B64" {
        let compact: String = raw.split_whitespace().collect();
        match STANDARD
            .decode(compact)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
        {
            Some(decoded) => decoded,
            None => return format!("[{media} content]"),
        }
    } else {
        raw.to_owned()
    };
    if media.contains("html") || media.contains("xml") {
        narrative::plain_text(&content).unwrap_or(content)
    } else if media.starts_with("text/plain") {
        content.trim().to_owned()
    } else {
        format!("[{media} content]")
    }
}

/// The extension a map marks a Bundle entry with to say what it could not carry over (group
/// `LeftOut` of the participants map): its value is the message, the entry's `target` the
/// resource it is about.
const CONVERSION_LOSS: &str = "http://octofhir.com/ccda-fhir/StructureDefinition/conversion-loss";

/// Takes the entries the maps marked with [`CONVERSION_LOSS`] out of the Bundle and reports
/// each one, naming the resource it is about.
pub(crate) fn take_conversion_losses(bundle: &mut Value) -> Vec<Loss> {
    let Some(entries) = bundle["entry"].as_array_mut() else {
        return Vec::new();
    };
    let mut messages = Vec::new();
    entries.retain(|entry| {
        let Some(message) = entry["resource"]["extension"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|e| e["url"] == CONVERSION_LOSS)
            .and_then(|e| e["valueString"].as_str())
        else {
            return true;
        };
        let target = entry["resource"]["target"][0]["reference"]
            .as_str()
            .unwrap_or_default();
        messages.push((target.to_owned(), message.to_owned()));
        false
    });
    let kinds: HashMap<&str, &str> = entries
        .iter()
        .filter_map(|e| {
            Some((
                e["fullUrl"].as_str()?,
                e["resource"]["resourceType"].as_str()?,
            ))
        })
        .collect();
    messages
        .into_iter()
        .map(|(target, message)| {
            let about = match kinds.get(target.as_str()) {
                Some(kind) => format!("{kind} {target}"),
                None => target,
            };
            Loss::new(Stage::Map, "", format!("{about}: {message}"))
        })
        .collect()
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

/// Resource types that are one thing whatever the place they are written in: the header's
/// encompassingEncounter and an Encounter Activity with the same id are one encounter; a
/// performer named only by its id is the practitioner described elsewhere with that id.
const ONE_PER_IDENTIFIER: &[&str] = &["Encounter", "Practitioner"];

/// Merges resources of the [`ONE_PER_IDENTIFIER`] types that share an identifier into the first
/// one: what the first lacks comes from the others, repeating elements are joined, and the
/// references point at the one kept.
pub(crate) fn merge_by_identifier(bundle: &mut Value) {
    let Some(entries) = bundle["entry"].as_array() else {
        return;
    };
    let mut kept_by_identifier: HashMap<String, usize> = HashMap::new();
    let mut merges: Vec<(usize, usize)> = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        let resource = &entry["resource"];
        if !resource["resourceType"]
            .as_str()
            .is_some_and(|kind| ONE_PER_IDENTIFIER.contains(&kind))
        {
            continue;
        }
        let keys: Vec<String> = resource["identifier"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|i| {
                let value = i["value"].as_str()?;
                // A bare OID (a root without an extension) names an assigning authority, which
                // examples share between people: not an identity.
                if i["system"] == "urn:ietf:rfc:3986" && value.starts_with("urn:oid:") {
                    return None;
                }
                Some(format!(
                    "{}|{}|{value}",
                    resource["resourceType"].as_str().unwrap_or_default(),
                    i["system"].as_str().unwrap_or_default()
                ))
            })
            .collect();
        match keys.iter().find_map(|k| kept_by_identifier.get(k)) {
            Some(&kept) => merges.push((kept, index)),
            None => {
                for key in keys {
                    kept_by_identifier.insert(key, index);
                }
            }
        }
    }
    if merges.is_empty() {
        return;
    }
    let mut replaced: HashMap<String, String> = HashMap::new();
    let mut removed = Vec::new();
    let Some(entries) = bundle["entry"].as_array_mut() else {
        return;
    };
    for (kept, other) in merges {
        let (Some(kept_url), Some(other_url)) = (
            entries[kept]["fullUrl"].as_str().map(str::to_owned),
            entries[other]["fullUrl"].as_str().map(str::to_owned),
        ) else {
            continue;
        };
        let other_resource = entries[other]["resource"].clone();
        merge_into(&mut entries[kept]["resource"], &other_resource);
        replaced.insert(other_url, kept_url);
        removed.push(other);
    }
    let mut index = 0;
    entries.retain(|_| {
        index += 1;
        !removed.contains(&(index - 1))
    });
    for entry in entries.iter_mut() {
        repoint(&mut entry["resource"], &replaced);
    }
}

/// A note's encounter is named by its id (`Reference.identifier`): when the Bundle has an
/// Encounter with that identifier, the reference points at its entry instead.
pub(crate) fn resolve_encounter_identifiers(bundle: &mut Value) {
    let Some(entries) = bundle["entry"].as_array() else {
        return;
    };
    let mut by_identifier: HashMap<(String, String), String> = HashMap::new();
    for entry in entries {
        let resource = &entry["resource"];
        if resource["resourceType"] != "Encounter" {
            continue;
        }
        let Some(full) = entry["fullUrl"].as_str() else {
            continue;
        };
        for identifier in resource["identifier"].as_array().into_iter().flatten() {
            if let Some(value) = identifier["value"].as_str() {
                let system = identifier["system"].as_str().unwrap_or_default();
                by_identifier
                    .entry((system.to_owned(), value.to_owned()))
                    .or_insert_with(|| full.to_owned());
            }
        }
    }
    let Some(entries) = bundle["entry"].as_array_mut() else {
        return;
    };
    for entry in entries {
        let Some(resource) = entry.get_mut("resource") else {
            continue;
        };
        if resource["resourceType"] != "DocumentReference" {
            continue;
        }
        let Some(references) = resource
            .get_mut("context")
            .and_then(|c| c.get_mut("encounter"))
            .and_then(Value::as_array_mut)
        else {
            continue;
        };
        for reference in references {
            let key = (
                reference["identifier"]["system"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                reference["identifier"]["value"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            );
            if let Some(full) = by_identifier.get(&key) {
                *reference = json!({ "reference": full });
            }
        }
    }
}

/// Adds to `kept` what it lacks of `other`: missing elements, items of repeating elements it
/// does not have yet; a value that is only data-absent gives way to a real one.
fn merge_into(kept: &mut Value, other: &Value) {
    let (Some(kept), Some(other)) = (kept.as_object_mut(), other.as_object()) else {
        return;
    };
    for (key, value) in other {
        if key == "id" {
            continue;
        }
        match (kept.get_mut(key), value) {
            (None, _) => {
                kept.insert(key.clone(), value.clone());
            }
            (Some(current), _) if is_absent(current) && !is_absent(value) => {
                *current = value.clone();
            }
            (Some(Value::Array(items)), Value::Array(more)) => {
                for item in more {
                    if !is_absent(item) && !items.contains(item) {
                        items.push(item.clone());
                    }
                }
            }
            _ => {}
        }
    }
}

/// A value that only says it is absent: a data-absent-reason extension and nothing else (or a
/// list of such).
fn is_absent(value: &Value) -> bool {
    const DATA_ABSENT: &str = "http://hl7.org/fhir/StructureDefinition/data-absent-reason";
    match value {
        Value::Array(items) => !items.is_empty() && items.iter().all(is_absent),
        Value::Object(object) => {
            object.len() == 1
                && object
                    .get("extension")
                    .and_then(Value::as_array)
                    .is_some_and(|e| !e.is_empty() && e.iter().all(|e| e["url"] == DATA_ABSENT))
        }
        _ => false,
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
