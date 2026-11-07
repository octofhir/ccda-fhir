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
    // With more than a code to say (the product's id, form, manufacturer), a Medication.
    let medication = request["medicationReference"]["reference"]
        .as_str()
        .and_then(|reference| {
            bundle["entry"]
                .as_array()
                .unwrap()
                .iter()
                .find(|e| e["fullUrl"] == reference)
        })
        .map(|e| e["resource"]["code"].clone())
        .unwrap_or_else(|| request["medicationCodeableConcept"].clone());
    assert_eq!(
        medication["coding"][0]["system"],
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

fn fragment(name: &str) -> String {
    metrics::as_document(&data(name)).expect("a section example")
}

#[tokio::test]
async fn encounters_convert_both_ways() {
    let converter = converter().await;
    let fhir = converter
        .to_fhir(&fragment("encounter-discharge-section.xml"), &options())
        .await
        .unwrap();
    let encounter = resources(&fhir.bundle, "Encounter")[0];
    assert_eq!(encounter["status"], "finished");
    assert_eq!(encounter["class"]["code"], "IMP");
    assert_eq!(encounter["type"][0]["coding"][0]["code"], "99238");
    assert_eq!(
        encounter["hospitalization"]["dischargeDisposition"]["coding"][0]["code"],
        "04"
    );
    // Discharge diagnoses: encounter-diagnosis Conditions with their role.
    let diagnoses = encounter["diagnosis"].as_array().unwrap();
    assert_eq!(diagnoses.len(), 2);
    assert_eq!(diagnoses[0]["use"]["coding"][0]["code"], "DD");
    for condition in resources(&fhir.bundle, "Condition") {
        assert_eq!(
            condition["category"][0]["coding"][0]["code"],
            "encounter-diagnosis"
        );
    }
    let location = resources(&fhir.bundle, "Location")[0];
    assert_eq!(location["name"], "Good Day Hospital");

    let cda = converter.to_cda(&fhir.bundle, &options()).await.unwrap();
    assert!(cda.xml.contains(
        r#"<templateId root="2.16.840.1.113883.10.20.22.4.49" extension="2015-08-01"/>"#
    ));
    assert!(cda.xml.contains(
        r#"<templateId root="2.16.840.1.113883.10.20.22.4.33" extension="2015-08-01"/>"#
    ));
    assert!(cda.xml.contains(r#"<participant typeCode="LOC">"#));
    let back = converter.to_fhir(&cda.xml, &options()).await.unwrap();
    for kind in ["Encounter", "Condition", "Location"] {
        assert_eq!(
            resources(&fhir.bundle, kind).len(),
            resources(&back.bundle, kind).len(),
            "{kind}"
        );
    }
}

#[tokio::test]
async fn the_document_encounter_is_one_with_its_encounter_activity() {
    let converter = converter().await;
    let fhir = converter
        .to_fhir(&data("encompassing-encounter.xml"), &options())
        .await
        .unwrap();
    let encounters = resources(&fhir.bundle, "Encounter");
    assert_eq!(encounters.len(), 1, "{encounters:#?}");
    let encounter = encounters[0];
    // From the header: the facility; from the entry: the type.
    assert!(encounter["location"].is_array());
    assert_eq!(encounter["type"][0]["coding"][0]["code"], "99213");
    let full_url = fhir.bundle["entry"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["resource"]["resourceType"] == "Encounter")
        .unwrap()["fullUrl"]
        .clone();
    let composition = &fhir.bundle["entry"][0]["resource"];
    assert_eq!(composition["encounter"]["reference"], full_url);
    // The note's encounter is the document's.
    let note = resources(&fhir.bundle, "DocumentReference")[0];
    assert_eq!(note["context"]["encounter"][0]["reference"], full_url);
    assert_eq!(note["type"]["coding"][0]["code"], "11506-3");
    assert_eq!(
        note["content"][0]["attachment"]["contentType"],
        "application/xhtml+xml"
    );

    let cda = converter.to_cda(&fhir.bundle, &options()).await.unwrap();
    assert!(cda.xml.contains("<encompassingEncounter>"));
    assert!(
        cda.xml
            .contains(r#"<templateId root="2.16.840.1.113883.10.20.22.4.202""#)
    );
    let document = fml_xml::read(&cda.xml, converter.model(), Some(CLINICAL_DOCUMENT)).unwrap();
    assert!(document.issues.is_empty(), "{:#?}", document.issues);
}

#[tokio::test]
async fn social_history_is_observations_and_patient_extensions() {
    let converter = converter().await;
    let fhir = converter
        .to_fhir(&fragment("social-sogi-section.xml"), &options())
        .await
        .unwrap();
    let observation = resources(&fhir.bundle, "Observation")[0];
    assert_eq!(
        observation["meta"]["profile"][0],
        "http://hl7.org/fhir/us/core/StructureDefinition/us-core-observation-sexual-orientation"
    );
    assert_eq!(
        observation["category"][0]["coding"][0]["code"],
        "social-history"
    );
    let patient = resources(&fhir.bundle, "Patient")[0];
    let gender_identity = patient["extension"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| {
            e["url"] == "http://hl7.org/fhir/us/core/StructureDefinition/us-core-genderIdentity"
        })
        .expect("gender identity on the Patient");
    assert_eq!(
        gender_identity["valueCodeableConcept"]["coding"][0]["code"],
        "407377005"
    );
    assert_eq!(resources(&fhir.bundle, "Observation").len(), 1);

    let cda = converter.to_cda(&fhir.bundle, &options()).await.unwrap();
    assert!(cda.xml.contains(
        r#"<templateId root="2.16.840.1.113883.10.20.34.3.45" extension="2023-05-01"/>"#
    ));
    assert!(cda.xml.contains(
        r#"<templateId root="2.16.840.1.113883.10.20.22.4.501" extension="2023-05-01"/>"#
    ));
}

#[tokio::test]
async fn not_pregnant_is_a_value() {
    let converter = converter().await;
    let fhir = converter
        .to_fhir(&fragment("social-not-pregnant-section.xml"), &options())
        .await
        .unwrap();
    let observation = resources(&fhir.bundle, "Observation")[0];
    assert_eq!(observation["code"]["coding"][0]["code"], "82810-3");
    assert_eq!(
        observation["valueCodeableConcept"]["coding"][0]["code"],
        "60001007"
    );
    assert!(observation["effectiveDateTime"].is_string());
}

#[tokio::test]
async fn a_note_round_trips_with_its_narrative() {
    let converter = converter().await;
    let fhir = converter
        .to_fhir(&fragment("note-consultation-section.xml"), &options())
        .await
        .unwrap();
    let note = resources(&fhir.bundle, "DocumentReference")[0];
    assert_eq!(note["category"][0]["coding"][0]["code"], "clinical-note");
    let cda = converter.to_cda(&fhir.bundle, &options()).await.unwrap();
    // The Note Activity refers to a paragraph of the narrative that holds its text.
    let reference = cda
        .xml
        .split("<reference value=\"#")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("a narrative reference");
    assert!(
        cda.xml
            .contains(&format!("<paragraph ID=\"{reference}\">Dr. Specialist"))
    );
    let back = converter.to_fhir(&cda.xml, &options()).await.unwrap();
    let again = resources(&back.bundle, "DocumentReference")[0];
    assert_eq!(again["type"], note["type"]);
    assert_eq!(
        again["content"][0]["attachment"]["data"],
        note["content"][0]["attachment"]["data"]
    );
}

#[tokio::test]
async fn goals_and_plans_convert_both_ways() {
    let converter = converter().await;
    let fhir = converter
        .to_fhir(&fragment("plan-goals-section.xml"), &options())
        .await
        .unwrap();
    let goal = resources(&fhir.bundle, "Goal")[0];
    assert_eq!(goal["lifecycleStatus"], "active");
    assert_eq!(goal["description"]["coding"][0]["code"], "289169006");
    let requests = resources(&fhir.bundle, "ServiceRequest");
    assert_eq!(requests.len(), 2);
    assert!(
        requests
            .iter()
            .all(|r| r["intent"] == "plan" && r["status"] == "active")
    );

    let planned = converter
        .to_fhir(&fragment("plan-procedures-section.xml"), &options())
        .await
        .unwrap();
    let order = resources(&planned.bundle, "ServiceRequest")
        .into_iter()
        .find(|r| r["code"]["coding"][0]["code"] == "6025007")
        .expect("the planned appendectomy");
    assert_eq!(order["intent"], "order");

    for bundle in [&fhir.bundle, &planned.bundle] {
        let cda = converter.to_cda(bundle, &options()).await.unwrap();
        let back = converter.to_fhir(&cda.xml, &options()).await.unwrap();
        for kind in ["Goal", "ServiceRequest"] {
            assert_eq!(
                resources(bundle, kind).len(),
                resources(&back.bundle, kind).len(),
                "{kind}"
            );
        }
    }
}

/// CDA → FHIR, then FHIR → CDA → FHIR keeps the number of resources of each kind.
async fn both_ways(converter: &Converter, name: &str, kinds: &[&str]) -> Value {
    let fhir = converter
        .to_fhir(&fragment(name), &options())
        .await
        .unwrap();
    let cda = converter.to_cda(&fhir.bundle, &options()).await.unwrap();
    let document = fml_xml::read(&cda.xml, converter.model(), Some(CLINICAL_DOCUMENT)).unwrap();
    assert!(document.issues.is_empty(), "{name}: {:#?}", document.issues);
    let back = converter.to_fhir(&cda.xml, &options()).await.unwrap();
    for kind in kinds {
        assert_eq!(
            resources(&fhir.bundle, kind).len(),
            resources(&back.bundle, kind).len(),
            "{name}: {kind}"
        );
    }
    fhir.bundle
}

fn has_category(resource: &Value, code: &str) -> bool {
    resource["category"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|c| c["coding"][0]["code"] == code)
}

#[tokio::test]
async fn functional_and_mental_status() {
    let converter = converter().await;
    let bundle = both_ways(converter, "mental-phq9-section.xml", &["Observation"]).await;
    let score = resources(&bundle, "Observation")
        .into_iter()
        .find(|o| o["code"]["coding"][0]["code"] == "44249-1")
        .expect("the PHQ-9 score");
    assert!(has_category(score, "cognitive-status") && has_category(score, "survey"));
    assert_eq!(score["valueInteger"], 12);
    assert!(!score["hasMember"].as_array().unwrap().is_empty());

    let bundle = both_ways(
        converter,
        "functional-walking-stick-section.xml",
        &["Observation"],
    )
    .await;
    let observation = resources(&bundle, "Observation")[0];
    assert!(has_category(observation, "functional-status"));
    assert_eq!(
        observation["valueCodeableConcept"]["coding"][0]["code"],
        "105504002"
    );
}

#[tokio::test]
async fn family_history_is_family_member_history() {
    let converter = converter().await;
    let bundle = both_ways(
        converter,
        "family-history-section.xml",
        &["FamilyMemberHistory"],
    )
    .await;
    let father = resources(&bundle, "FamilyMemberHistory")
        .into_iter()
        .find(|m| m["relationship"]["coding"][0]["code"] == "FTH")
        .expect("the father");
    assert_eq!(father["sex"]["coding"][0]["code"], "male");
    assert!(father["deceasedDate"].is_string());
    assert!(!father["condition"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn implants_are_devices_with_their_udi() {
    let converter = converter().await;
    let bundle = both_ways(
        converter,
        "implant-udi-section.xml",
        &["Procedure", "Device"],
    )
    .await;
    let device = resources(&bundle, "Device")[0];
    assert_eq!(
        device["udiCarrier"][0]["deviceIdentifier"],
        "00848486001048"
    );
    assert_eq!(device["lotNumber"], "ABC999");
    let procedure = resources(&bundle, "Procedure")[0];
    assert!(procedure["focalDevice"][0]["manipulated"]["reference"].is_string());
}

#[tokio::test]
async fn health_concerns_round_trip() {
    let converter = converter().await;
    let section = r#"<section xmlns="urn:hl7-org:v3" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <templateId root="2.16.840.1.113883.10.20.22.2.58" extension="2015-08-01"/>
  <code code="75310-3" codeSystem="2.16.840.1.113883.6.1"/>
  <title>Health Concerns</title>
  <text><paragraph>Shortness of breath</paragraph></text>
  <entry>
    <act classCode="ACT" moodCode="EVN">
      <templateId root="2.16.840.1.113883.10.20.22.4.132" extension="2015-08-01"/>
      <id root="b2c3b5ba-1f6d-4a40-9a4e-1f8c1a0e2a11"/>
      <code code="75310-3" codeSystem="2.16.840.1.113883.6.1"/>
      <statusCode code="active"/>
      <entryRelationship typeCode="REFR">
        <observation classCode="OBS" moodCode="EVN">
          <templateId root="2.16.840.1.113883.10.20.22.4.4" extension="2015-08-01"/>
          <id root="c3d4e5f6-1f6d-4a40-9a4e-1f8c1a0e2a12"/>
          <code code="75315-2" codeSystem="2.16.840.1.113883.6.1"/>
          <statusCode code="completed"/>
          <effectiveTime><low value="20240102"/></effectiveTime>
          <value xsi:type="CD" code="267036007" codeSystem="2.16.840.1.113883.6.96" displayName="Dyspnea"/>
        </observation>
      </entryRelationship>
    </act>
  </entry>
</section>"#;
    let document = metrics::as_document(section).expect("a section");
    let fhir = converter.to_fhir(&document, &options()).await.unwrap();
    let conditions = resources(&fhir.bundle, "Condition");
    assert_eq!(conditions.len(), 1);
    assert_eq!(
        conditions[0]["category"][0]["coding"][0]["system"],
        "http://hl7.org/fhir/us/core/CodeSystem/condition-category"
    );
    assert_eq!(
        conditions[0]["category"][0]["coding"][0]["code"],
        "health-concern"
    );
    assert_eq!(conditions[0]["code"]["coding"][0]["code"], "267036007");

    let cda = converter.to_cda(&fhir.bundle, &options()).await.unwrap();
    assert!(cda.xml.contains(
        r#"<templateId root="2.16.840.1.113883.10.20.22.4.132" extension="2022-06-01"/>"#
    ));
    let back = converter.to_fhir(&cda.xml, &options()).await.unwrap();
    let again = resources(&back.bundle, "Condition");
    assert_eq!(again.len(), 1);
    assert_eq!(
        again[0]["category"][0]["coding"][0]["code"],
        "health-concern"
    );
}

#[tokio::test]
async fn a_problems_supporting_observation_is_its_evidence() {
    let converter = converter().await;
    let section = r#"<section xmlns="urn:hl7-org:v3" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <templateId root="2.16.840.1.113883.10.20.22.2.5.1" extension="2015-08-01"/>
  <code code="11450-4" codeSystem="2.16.840.1.113883.6.1"/>
  <title>Problems</title>
  <text><paragraph>Food insecurity</paragraph></text>
  <entry>
    <act classCode="ACT" moodCode="EVN">
      <templateId root="2.16.840.1.113883.10.20.22.4.3" extension="2015-08-01"/>
      <id root="d1e2f3a4-1f6d-4a40-9a4e-1f8c1a0e2a13"/>
      <code code="CONC" codeSystem="2.16.840.1.113883.5.6"/>
      <statusCode code="active"/>
      <effectiveTime><low value="20240102"/></effectiveTime>
      <entryRelationship typeCode="SUBJ">
        <observation classCode="OBS" moodCode="EVN">
          <templateId root="2.16.840.1.113883.10.20.22.4.4" extension="2015-08-01"/>
          <id root="e2f3a4b5-1f6d-4a40-9a4e-1f8c1a0e2a14"/>
          <code code="55607006" codeSystem="2.16.840.1.113883.6.96"/>
          <statusCode code="completed"/>
          <effectiveTime><low value="20240102"/></effectiveTime>
          <value xsi:type="CD" code="733423003" codeSystem="2.16.840.1.113883.6.96" displayName="Food insecurity"/>
          <entryRelationship typeCode="SPRT">
            <observation classCode="OBS" moodCode="EVN">
              <templateId root="2.16.840.1.113883.10.20.22.4.69" extension="2022-06-01"/>
              <id root="f3a4b5c6-1f6d-4a40-9a4e-1f8c1a0e2a15"/>
              <code code="88124-3" codeSystem="2.16.840.1.113883.6.1" displayName="Food insecurity risk"/>
              <statusCode code="completed"/>
              <effectiveTime value="20240102"/>
              <value xsi:type="CD" code="LA19952-3" codeSystem="2.16.840.1.113883.6.1" displayName="At risk"/>
            </observation>
          </entryRelationship>
        </observation>
      </entryRelationship>
    </act>
  </entry>
</section>"#;
    let document = metrics::as_document(section).expect("a section");
    let fhir = converter.to_fhir(&document, &options()).await.unwrap();
    let condition = resources(&fhir.bundle, "Condition")[0];
    let evidence = condition["evidence"][0]["detail"][0]["reference"]
        .as_str()
        .unwrap();
    let screening = fhir.bundle["entry"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["fullUrl"] == evidence)
        .map(|e| &e["resource"])
        .expect("the evidence in the Bundle");
    assert_eq!(screening["code"]["coding"][0]["code"], "88124-3");
    assert!(has_category(screening, "survey"));

    let cda = converter.to_cda(&fhir.bundle, &options()).await.unwrap();
    let back = converter.to_fhir(&cda.xml, &options()).await.unwrap();
    let again = resources(&back.bundle, "Condition")[0];
    assert_eq!(again["evidence"].as_array().map(Vec::len), Some(1));
}

/// The entry of `bundle` a reference names.
fn resolve<'a>(bundle: &'a Value, reference: &Value) -> &'a Value {
    let reference = reference["reference"].as_str().expect("a reference");
    bundle["entry"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["fullUrl"] == reference)
        .map(|e| &e["resource"])
        .unwrap_or_else(|| panic!("{reference} is not in the Bundle"))
}

#[tokio::test]
async fn entry_authors_are_provenance() {
    let converter = converter().await;
    // The codeine allergy with three more authors on the observation: one representing an
    // organization; one with neither an organization nor a role code (US Core cannot have it as
    // an agent: provenance-1); one with an organization whose time is only a date.
    let authors = r#"</author>
<author>
  <templateId root="2.16.840.1.113883.10.20.22.4.119"/>
  <time value="20140104090000-0500"/>
  <assignedAuthor>
    <id extension="12345" root="2.16.840.1.113883.4.6"/>
    <addr><streetAddressLine>1002 Healthcare Dr</streetAddressLine><city>Portland</city><state>OR</state><postalCode>97266</postalCode></addr>
    <telecom use="WP" value="tel:555-555-1003"/>
    <assignedPerson><name><given>Ada</given><family>Clinic</family></name></assignedPerson>
    <representedOrganization>
      <id root="2.16.840.1.113883.19.5.9999.1393"/>
      <name>Community Health and Hospitals</name>
      <telecom use="WP" value="tel:555-555-5000"/>
      <addr><streetAddressLine>1001 Village Avenue</streetAddressLine><city>Portland</city><state>OR</state><postalCode>99123</postalCode></addr>
    </representedOrganization>
  </assignedAuthor>
</author>
<author>
  <templateId root="2.16.840.1.113883.10.20.22.4.119"/>
  <time value="201401050930-0500"/>
  <assignedAuthor>
    <id extension="67890" root="2.16.840.1.113883.4.6"/>
    <assignedPerson><name><given>Bo</given><family>Solo</family></name></assignedPerson>
  </assignedAuthor>
</author>
<author>
  <templateId root="2.16.840.1.113883.10.20.22.4.119"/>
  <time value="20140106"/>
  <assignedAuthor>
    <id extension="24680" root="2.16.840.1.113883.4.6"/>
    <addr><streetAddressLine>1002 Healthcare Dr</streetAddressLine><city>Portland</city><state>OR</state><postalCode>97266</postalCode></addr>
    <telecom use="WP" value="tel:555-555-1004"/>
    <assignedPerson><name><given>Cy</given><family>Later</family></name></assignedPerson>
    <representedOrganization>
      <id root="2.16.840.1.113883.19.5.9999.1393"/>
      <name>Community Health and Hospitals</name>
      <telecom use="WP" value="tel:555-555-5000"/>
      <addr><streetAddressLine>1001 Village Avenue</streetAddressLine><city>Portland</city><state>OR</state><postalCode>99123</postalCode></addr>
    </representedOrganization>
  </assignedAuthor>
</author>"#;
    let mut document = fragment("allergy-codeine-section.xml");
    // The observation's author is the last one (the concern act's comes before it).
    let at = document.rfind("</author>").unwrap();
    document.replace_range(at..at + "</author>".len(), authors);
    let fhir = converter.to_fhir(&document, &options()).await.unwrap();
    let bundle = &fhir.bundle;
    let allergy_url = bundle["entry"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["resource"]["resourceType"] == "AllergyIntolerance")
        .and_then(|e| e["fullUrl"].as_str())
        .unwrap();

    // One Provenance per author US Core can take, in document order.
    let provenances = resources(bundle, "Provenance");
    assert_eq!(provenances.len(), 3);
    for provenance in &provenances {
        assert_eq!(
            provenance["meta"]["profile"][0],
            "http://hl7.org/fhir/us/core/StructureDefinition/us-core-provenance"
        );
        assert_eq!(provenance["target"][0]["reference"], allergy_url);
        let agent = &provenance["agent"][0];
        assert_eq!(
            agent["type"]["coding"][0]["system"],
            "http://terminology.hl7.org/CodeSystem/provenance-participant-type"
        );
        assert_eq!(agent["type"]["coding"][0]["code"], "author");
        // A role (code or organization): never a bare Practitioner (provenance-1).
        assert_eq!(
            resolve(bundle, &agent["who"])["resourceType"],
            "PractitionerRole"
        );
    }
    // Henry Seven, in a role with a code and no organization: no onBehalfOf.
    assert_eq!(provenances[0]["recorded"], "2014-01-03T10:25:00-05:00");
    assert!(provenances[0]["agent"][0]["onBehalfOf"].is_null());
    // Ada Clinic, on behalf of her organization (the role's own).
    let ada = &provenances[1]["agent"][0];
    assert_eq!(provenances[1]["recorded"], "2014-01-04T09:00:00-05:00");
    let organization = resolve(bundle, &ada["onBehalfOf"]);
    assert_eq!(organization["name"], "Community Health and Hospitals");
    assert_eq!(
        resolve(bundle, &ada["who"])["organization"]["reference"],
        ada["onBehalfOf"]["reference"]
    );
    // Cy Later's time is a date: recorded is data-absent, not padded.
    assert!(provenances[2]["recorded"].is_null());
    assert_eq!(
        provenances[2]["_recorded"]["extension"][0]["valueCode"],
        "unknown"
    );
    // Bo Solo has no Provenance, and the report says so.
    let left_out: Vec<&str> = fhir
        .report
        .losses
        .iter()
        .map(|l| l.message.as_str())
        .filter(|m| m.contains("provenance-1"))
        .collect();
    assert_eq!(left_out.len(), 1, "{left_out:?}");
    assert!(left_out[0].starts_with(&format!(
        "AllergyIntolerance {allergy_url}: author 2.16.840.1.113883.4.6^67890"
    )));
    // The recorder is still the last author.
    assert_eq!(
        resolve(
            bundle,
            &resources(bundle, "AllergyIntolerance")[0]["recorder"]
        )["resourceType"],
        "PractitionerRole"
    );

    // Back to CDA: the recorder (Cy Later) and the other Provenance authors, each once.
    let cda = converter.to_cda(bundle, &options()).await.unwrap();
    for (id, times) in [("99999999", 1), ("12345", 1), ("24680", 1), ("67890", 0)] {
        let authored = format!(r#"<id root="2.16.840.1.113883.4.6" extension="{id}"/>"#);
        assert_eq!(cda.xml.matches(&authored).count(), times, "author {id}");
    }
    assert!(cda.xml.contains(r#"<time value="20140104090000-0500"/>"#));
    assert!(
        cda.xml
            .contains("<name>Community Health and Hospitals</name>")
    );
    let back = converter.to_fhir(&cda.xml, &options()).await.unwrap();
    assert_eq!(resources(&back.bundle, "Provenance").len(), 3);
}
