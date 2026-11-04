//! Conversions of HL7 C-CDA examples (tests/data, CC0) both ways.

use std::path::Path;
use std::sync::OnceLock;

use ccda_fhir::{CLINICAL_DOCUMENT, Converter, Options, metrics};
use serde_json::Value;
use tokio::sync::OnceCell;

fn data(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

async fn converter() -> &'static Converter {
    static CONVERTER: OnceCell<Converter> = OnceCell::const_new();
    CONVERTER
        .get_or_init(|| async { Converter::new().await.expect("the embedded maps load") })
        .await
}

fn options() -> Options {
    Options {
        deterministic_ids: true,
    }
}

fn resources<'a>(bundle: &'a Value, kind: &str) -> Vec<&'a Value> {
    bundle["entry"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|e| &e["resource"])
        .filter(|r| r["resourceType"] == kind)
        .collect()
}

#[test]
fn the_maps_check_clean() {
    static PROBLEMS: OnceLock<()> = OnceLock::new();
    PROBLEMS.get_or_init(|| {
        let problems = Converter::check_maps(&Converter::embedded_maps()).unwrap();
        assert!(problems.is_empty(), "{problems:#?}");
    });
}

#[tokio::test]
async fn a_ccd_becomes_a_document_bundle() {
    let converter = converter().await;
    let result = converter
        .to_fhir(&data("ccd-2.xml"), &options())
        .await
        .unwrap();
    let bundle = &result.bundle;
    assert_eq!(bundle["type"], "document");
    assert_eq!(
        bundle["entry"][0]["resource"]["resourceType"],
        "Composition"
    );
    let composition = &bundle["entry"][0]["resource"];
    assert_eq!(composition["type"]["coding"][0]["code"], "34133-9");
    assert_eq!(composition["date"], "2014-10-15T10:30:26-05:00");
    // Every reference in the Bundle points at an entry.
    let full_urls: Vec<&str> = bundle["entry"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|e| e["fullUrl"].as_str())
        .collect();
    let mut references = Vec::new();
    collect_references(bundle, &mut references);
    for reference in references {
        assert!(
            full_urls.contains(&reference.as_str()),
            "dangling reference {reference}"
        );
    }
    // The patient: US Core identifiers, name, gender, birth date, race and ethnicity.
    let patient = resources(bundle, "Patient")[0];
    assert_eq!(
        patient["identifier"][1]["system"],
        "http://hl7.org/fhir/sid/us-ssn"
    );
    assert_eq!(patient["gender"], "female");
    assert_eq!(patient["birthDate"], "1950-12-19");
    let race = patient["extension"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["url"] == "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race")
        .unwrap();
    assert_eq!(race["extension"][0]["valueCoding"]["code"], "2106-3");
    // No known allergies: a negated allergy observation with a class as its substance.
    let allergy = resources(bundle, "AllergyIntolerance")[0];
    assert_eq!(allergy["code"]["coding"][0]["code"], "716186003");
    assert_eq!(allergy["clinicalStatus"]["coding"][0]["code"], "active");
    // Section narrative is XHTML.
    let narrative = composition["section"][0]["text"]["div"].as_str().unwrap();
    assert!(
        narrative.starts_with("<div xmlns=\"http://www.w3.org/1999/xhtml\""),
        "{narrative}"
    );
}

fn collect_references(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                match child {
                    Value::String(reference) if key == "reference" => out.push(reference.clone()),
                    _ => collect_references(child, out),
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|i| collect_references(i, out)),
        _ => {}
    }
}

#[tokio::test]
async fn all_first_stage_domains_convert() {
    let converter = converter().await;
    let result = converter
        .to_fhir(&data("ccd-1.xml"), &options())
        .await
        .unwrap();
    let bundle = &result.bundle;
    for kind in [
        "AllergyIntolerance",
        "Condition",
        "MedicationRequest",
        "Immunization",
        "DiagnosticReport",
        "Observation",
        "Procedure",
    ] {
        assert!(!resources(bundle, kind).is_empty(), "no {kind}");
    }
    let condition = resources(bundle, "Condition")[0];
    assert_eq!(
        condition["category"][0]["coding"][0]["code"],
        "problem-list-item"
    );
    let request = resources(bundle, "MedicationRequest")[0];
    assert_eq!(
        request["medicationCodeableConcept"]["coding"][0]["system"],
        "http://www.nlm.nih.gov/research/umls/rxnorm"
    );
    assert_eq!(
        request["dosageInstruction"][0]["timing"]["repeat"]["periodUnit"],
        "h"
    );
    let panel = resources(bundle, "Observation")
        .into_iter()
        .find(|o| o["code"]["coding"][0]["code"] == "85353-1")
        .expect("a vital signs panel");
    assert!(!panel["hasMember"].as_array().unwrap().is_empty());
    let report = resources(bundle, "DiagnosticReport")[0];
    assert_eq!(report["category"][0]["coding"][0]["code"], "LAB");
    assert!(!report["result"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn back_to_cda_reads_as_the_model() {
    let converter = converter().await;
    for name in ["ccd-1.xml", "ccd-2.xml"] {
        let fhir = converter.to_fhir(&data(name), &options()).await.unwrap();
        let cda = converter.to_cda(&fhir.bundle, &options()).await.unwrap();
        let write_losses: Vec<_> = cda
            .report
            .losses
            .iter()
            .filter(|l| l.stage == ccda_fhir::Stage::Write)
            .collect();
        assert!(write_losses.is_empty(), "{name}: {write_losses:#?}");
        let document = fml_xml::read(&cda.xml, converter.model(), Some(CLINICAL_DOCUMENT)).unwrap();
        assert!(document.issues.is_empty(), "{name}: {:#?}", document.issues);
        assert_eq!(
            document.json["templateId"][0]["root"],
            "2.16.840.1.113883.10.20.22.1.1"
        );
        assert_eq!(
            document.json["recordTarget"][0]["patientRole"]["patient"]["administrativeGenderCode"]
                ["code"],
            "F"
        );
    }
}

/// The round trip keeps at least what it keeps today: a regression floor, not a goal.
#[tokio::test]
async fn round_trips_keep_their_values() {
    let converter = converter().await;
    for (name, floor) in [
        ("ccd-1.xml", 0.48),
        ("ccd-2.xml", 0.55),
        ("allergy-codeine-section.xml", 0.45),
    ] {
        let measured = metrics::measure(converter, Path::new(name), &data(name)).await;
        assert!(measured.error.is_none(), "{name}: {:?}", measured.error);
        let kept = measured.kept as f64 / measured.facts as f64;
        assert!(
            kept >= floor,
            "{name}: {:.3} of the values kept, below {floor}; missing {:#?}",
            kept,
            measured.missing
        );
    }
}

#[tokio::test]
async fn fhir_to_cda_to_fhir_keeps_the_resources() {
    let converter = converter().await;
    let first = converter
        .to_fhir(&data("ccd-1.xml"), &options())
        .await
        .unwrap();
    let cda = converter.to_cda(&first.bundle, &options()).await.unwrap();
    let second = converter.to_fhir(&cda.xml, &options()).await.unwrap();
    for kind in [
        "AllergyIntolerance",
        "Condition",
        "MedicationRequest",
        "Immunization",
        "DiagnosticReport",
        "Procedure",
    ] {
        assert_eq!(
            resources(&first.bundle, kind).len(),
            resources(&second.bundle, kind).len(),
            "{kind}"
        );
    }
    // The clinical codes come back.
    let codes = |bundle: &Value| {
        let mut out = std::collections::BTreeSet::new();
        for kind in [
            "AllergyIntolerance",
            "Condition",
            "MedicationRequest",
            "Immunization",
            "Procedure",
        ] {
            for resource in resources(bundle, kind) {
                out.extend(
                    metrics::fhir_facts(resource)
                        .into_iter()
                        .filter(|f| f.starts_with("coding")),
                );
            }
        }
        out
    };
    let (before, after) = (codes(&first.bundle), codes(&second.bundle));
    let missing: Vec<_> = before.difference(&after).collect();
    assert!(
        missing.len() * 10 <= before.len(),
        "{} of {} codings lost: {missing:#?}",
        missing.len(),
        before.len()
    );
}

#[tokio::test]
async fn a_composition_alone_converts() {
    let converter = converter().await;
    let composition = serde_json::json!({
        "resourceType": "Composition", "id": "c1", "status": "final",
        "type": { "coding": [{ "system": "http://loinc.org", "code": "34133-9" }] },
        "date": "2024-01-02", "title": "Summary", "author": [{ "display": "unknown" }]
    });
    let cda = converter.to_cda(&composition, &options()).await.unwrap();
    assert!(cda.xml.contains("<title>Summary</title>"));
    assert!(cda.xml.contains(r#"<effectiveTime value="20240102"/>"#));
}

#[tokio::test]
async fn not_a_document_is_an_error() {
    let converter = converter().await;
    assert!(
        converter
            .to_fhir("<observation xmlns=\"urn:hl7-org:v3\"/>", &options())
            .await
            .is_err()
    );
    assert!(
        converter
            .to_cda(
                &serde_json::json!({ "resourceType": "Patient" }),
                &options()
            )
            .await
            .is_err()
    );
}
