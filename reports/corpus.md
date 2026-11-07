# Corpus report

506 documents (307 section fragments in a minimal document); 8 not well-formed XML in the source, 0 failed; 19 other fragments skipped.

| Measure | Value |
|---|---|
| Sections recognised | 2951 of 3092 (95.4%) |
| Entries recognised | 3838 of 4048 (94.8%) |
| Round trip CDA → FHIR → CDA, significant values kept | 46953 of 63717 (73.7%) |
| Round trip FHIR → CDA → FHIR, significant values of the resources kept | 52402 of 53144 (98.6%) |
| Resources valid against US Core 6.1.0 | 11069 of 11099 (99.7%) |
| CDA written back conforming to C-CDA 5.0.0 templates | 459 of 498 documents (92.2%); 14962 of 15012 templated elements (99.7%) |
| Losses reported | 1024 |

## Resources

| Type | Count |
|---|---|
| AllergyIntolerance | 346 |
| Composition | 498 |
| Condition | 889 |
| Device | 96 |
| DiagnosticReport | 182 |
| DocumentReference | 15 |
| Encounter | 319 |
| FamilyMemberHistory | 84 |
| Goal | 73 |
| Immunization | 291 |
| Location | 235 |
| Medication | 435 |
| MedicationRequest | 537 |
| Observation | 2349 |
| Organization | 1243 |
| Patient | 493 |
| Practitioner | 1599 |
| PractitionerRole | 774 |
| Procedure | 261 |
| Provenance | 505 |
| ServiceRequest | 373 |

## US Core validation errors

| Error | Count |
|---|---|
| AllergyIntolerance "AllergyIntolerance": Required element 'code' is missing | 12 |
| MedicationRequest "MedicationRequest": Constraint 'us-core-21' failed: requester SHALL be present if intent is "order" | 5 |
| MedicationRequest "MedicationRequest": Required element 'medication' is missing | 5 |
| Procedure "Procedure": Required element 'code' is missing | 3 |
| Observation "Observation": Constraint 'vs-2' failed: If there is no component or hasMember element then either a value[x] or a data absent reason must be present. | 2 |
| Patient "Patient": Required element 'name' is missing | 2 |
| DiagnosticReport "DiagnosticReport": Required element 'code' is missing | 1 |
| Observation "Observation"."effective"."ofType(period)"."end": dateTime is not a valid calendar date: "2010-04-31" | 1 |
| Observation "Observation": Required element 'effective' is missing | 1 |
| Observation "Observation": Required element 'effectiveDateTime' is missing | 1 |
| Observation "Observation": Required element 'value' is missing | 1 |
| Procedure "Procedure": Constraint 'us-core-7' failed: Performed SHALL be present if the status is 'completed' or 'in-progress' | 1 |

## C-CDA conformance errors (CDA written back)

Checked: element cardinality, fixed and pattern values, error invariants of the templates applied by template id. Not checked: slices, terminology.

| Error | Count |
|---|---|
| USRealmHeader: component needs at least 1, has 0 | 16 |
| USRealmHeader: author needs at least 1, has 0 | 6 |
| USRealmHeader: custodian needs at least 1, has 0 | 6 |
| USRealmHeader: code needs at least 1, has 0 | 5 |
| USRealmHeader: effectiveTime needs at least 1, has 0 | 5 |
| USRealmHeader: recordTarget needs at least 1, has 0 | 5 |
| USRealmHeader: title needs at least 1, has 0 | 5 |
| CommentActivity: text.reference.value needs at least 1, has 0 | 4 |
| GoalObservation: id needs at least 1, has 0 | 3 |
| MedicationActivity: statusCode needs at least 1, has 0 | 3 |
| ProcedureActivityProcedure: code needs at least 1, has 0 | 3 |
| ProcedureActivityProcedure: statusCode needs at least 1, has 0 | 3 |
| ContinuityofCareDocumentCCD: component needs at least 1, has 0 | 2 |
| ContinuityofCareDocumentCCD: recordTarget.patientRole.patient.name needs at least 1, has 0 | 2 |
| ResultOrganizer: component needs at least 1, has 0 | 2 |
| USRealmHeader: recordTarget.patientRole.patient.name needs at least 1, has 0 | 2 |
| AllergiesAndIntolerancesSection: text needs at least 1, has 0 | 1 |
| AssessmentSection: text needs at least 1, has 0 | 1 |
| AuthorParticipation: assignedAuthor.id needs at least 1, has 0 | 1 |
| ContinuityofCareDocumentCCD: 1198-8456: Such assignedAuthors **SHALL** contain (exactly one [1..1] assignedPerson) or (exactly one [1..1] assignedAuthoringDevice and exactly one [1..1] representedOrganization) (CONF:1198-8456). | 1 |
| ContinuityofCareDocumentCCD: 4537-16790: There **SHALL** be exactly one assignedAuthor/assignedPerson or exactly one assignedAuthor/assignedAuthoringDevice (CONF:4537-16790). | 1 |
| ContinuityofCareDocumentCCD: author needs at least 1, has 0 | 1 |
| ContinuityofCareDocumentCCD: custodian needs at least 1, has 0 | 1 |
| ContinuityofCareDocumentCCD: custodian.assignedCustodian.representedCustodianOrganization.id needs at least 1, has 0 | 1 |
| ContinuityofCareDocumentCCD: custodian.assignedCustodian.representedCustodianOrganization.name needs at least 1, has 0 | 1 |
| FamilyHistoryOrganizer: component needs at least 1, has 0 | 1 |
| HistoryofPresentIllnessSection: text needs at least 1, has 0 | 1 |
| MedicationsSection: text needs at least 1, has 0 | 1 |
| PlanofTreatmentSection: text needs at least 1, has 0 | 1 |
| ProblemSection: text needs at least 1, has 0 | 1 |

Invariants that did not evaluate: 4537-5402 (174), author-details (456), no-a-p (2), no-cc-rfv (4), shall-use-one-binding (220), ts-value-before-document (228)

## Losses by stage

| Stage | Count |
|---|---|
| map | 817 |
| narrative | 21 |
| read | 186 |

## Values a round trip lost, by kind

| Kind | Documents |
|---|---|
| city | 183 |
| code | 2249 |
| country | 138 |
| family | 120 |
| given | 125 |
| id | 2962 |
| postalCode | 151 |
| prefix | 72 |
| quantity | 314 |
| state | 147 |
| streetAddressLine | 160 |
| suffix | 14 |
| time | 1856 |

## Templates not mapped

| Template | Occurrences |
|---|---|
| 2.16.840.1.113883.10.20.22.4.5 | 26 |
| 2.16.840.1.113883.10.20.22.4.5:2014-06-09 | 26 |
| 2.16.840.1.113883.10.20.1.2.3.4.5.6.7.8.9 | 20 |
| 2.16.840.1.113883.10.20.22.4.131 | 11 |
| 2.16.840.1.113883.10.20.22.4.136 | 11 |
| 2.16.840.1.113883.10.20.21.2.3:2015-08-01 | 10 |
| 2.16.840.1.113883.10.20.22.2.61 | 10 |
| 2.16.840.1.113883.10.20.22.4.144 | 9 |
| 2.16.840.1.113883.10.20.22.4.146 | 9 |
| 2.16.840.1.113883.10.20.24.3.97 | 8 |
| 2.16.840.1.113883.10.20.24.3.98 | 8 |
| 2.16.840.1.113883.10.20.22.2.500:2019-07-01 | 7 |
| 2.16.840.1.113883.10.20.22.4.12:2014-06-09 | 7 |
| 2.16.840.1.113883.10.20.22.4.13:2014-06-09 | 6 |
| 2.16.840.1.113883.10.20.2.10:2014-06-09 | 5 |
| 2.16.840.1.113883.10.20.21.2.3:2014-06-09 | 5 |
| 2.16.840.1.113883.10.20.22.4.108 | 5 |
| 2.16.840.1.113883.10.20.22.4.141 | 5 |
| 2.16.840.1.113883.3.3251.1.4 | 5 |
| 2.16.840.1.113883.3.3251.1.5 | 5 |
| 2.16.840.1.113883.10.20.1.2.3.4.5.6.7.8.9:2015-08-01 | 4 |
| 2.16.840.1.113883.10.20.21.2.3 | 4 |
| 2.16.840.1.113883.10.20.22.2.500:2022-06-01 | 4 |
| 2.16.840.1.113883.10.20.22.2.57 | 4 |
| 2.16.840.1.113883.10.20.22.4.124 | 4 |
| 2.16.840.1.113883.10.20.22.4.130 | 4 |
| 2.16.840.1.113883.10.20.6.2.12 | 4 |
| 2.16.840.1.113883.10.20.22.2.20:2014-06-09 | 3 |
| 2.16.840.1.113883.10.20.22.2.25:2014-06-09 | 3 |
| 2.16.840.1.113883.10.20.22.2.29:2014-06-09 | 3 |
| 2.16.840.1.113883.10.20.22.2.30:2014-06-09 | 3 |
| 2.16.840.1.113883.10.20.22.4.500:2019-07-01 | 3 |
| 2.16.840.1.113883.10.20.1.2.3.4.5.6.7.8.9:2014-06-09 | 2 |
| 2.16.840.1.113883.10.20.2.10:2015-08-01 | 2 |
| 2.16.840.1.113883.10.20.22.2.18:2014-06-09 | 2 |
| 2.16.840.1.113883.10.20.22.2.21:2014-06-09 | 2 |
| 2.16.840.1.113883.10.20.22.2.28:2014-06-09 | 2 |
| 2.16.840.1.113883.10.20.22.2.34:2014-06-09 | 2 |
| 2.16.840.1.113883.10.20.22.2.36:2015-08-01 | 2 |
| 2.16.840.1.113883.10.20.22.2.37:2014-06-09 | 2 |
| 2.16.840.1.113883.10.20.22.2.63 | 2 |
| 2.16.840.1.113883.10.20.22.4.114 | 2 |
| 2.16.840.1.113883.10.20.22.4.120 | 2 |
| 2.16.840.1.113883.10.20.22.4.131:2015-08-01 | 2 |
| 2.16.840.1.113883.10.20.22.4.500:2022-06-01 | 2 |
| 2.16.840.1.113883.10.20.22.4.51:2014-06-09 | 2 |
| 2.16.840.1.113883.10.20.22.4.60:2014-06-09 | 2 |
| 2.16.840.1.113883.10.20.22.4.65:2014-06-09 | 2 |
| 2.16.840.1.113883.10.20.24.2.2 | 2 |
| 2.16.840.1.113883.10.20.24.2.3 | 2 |
| 2.16.840.1.113883.10.20.6.1.1 | 2 |
| 2.16.840.1.113883.10.20.6.1.2 | 2 |
| 2.16.840.1.113883.10.20.6.2.6 | 2 |
| 1.3.6.1.4.1.19376.1.5.3.1.3.4.2 | 1 |
| 2.16.840.1.113883.10.20.21.2.45:2014-06-09 | 1 |
| 2.16.840.1.113883.10.20.22.2.18:2015-08-01 | 1 |
| 2.16.840.1.113883.10.20.22.2.20:2015-08-01 | 1 |
| 2.16.840.1.113883.10.20.22.2.21.1:2014-06-09 | 1 |
| 2.16.840.1.113883.10.20.22.2.21.1:2015-08-01 | 1 |
| 2.16.840.1.113883.10.20.22.2.28:2015-08-01 | 1 |

## Documents

| Document | Sections | Entries | Round trip | Losses |
|---|---|---|---|---|
| corpus/C-CDA-Examples/Allergies/Allergy to food egg/Allergy to food egg(C-CDA2.1).xml | 1/1 | 1/1 | 66.7% | 0 |
| corpus/C-CDA-Examples/Allergies/Allergy to latex/Allergy to specific substance latex(C-CDA2.1).xml | 1/1 | 1/1 | 61.9% | 2 |
| corpus/C-CDA-Examples/Allergies/Allergy to specific drug Codeine/Allergy to specific drug Codeine(C-CDA2.1).xml | 1/1 | 1/1 | 48.6% | 0 |
| corpus/C-CDA-Examples/Allergies/Allergy to specific drug Penicillin/Allergy to specific drug Penicillin(C-CDA2.1).xml | 1/1 | 1/1 | 66.7% | 0 |
| corpus/C-CDA-Examples/Allergies/Allergy to specific drug class Penicillins/Allergy to specific drug class Penicillins(C-CDA2.1).xml | 1/1 | 1/1 | 66.7% | 0 |
| corpus/C-CDA-Examples/Allergies/Free-text Allergy to clinical trial drug/Free-text Allergy to clinical trial drug(C-CDA2.1).xml | 1/1 | 1/1 | 63.6% | 0 |
| corpus/C-CDA-Examples/Allergies/No Known Allergies/No Known Allergies with Author Timestamp(C-CDA2.1).xml | 1/1 | 1/1 | 46.7% | 1 |
| corpus/C-CDA-Examples/Allergies/No Known Medication Allergies/No Known Medication Allergies(C-CDA2.1).xml | 1/1 | 1/1 | 53.3% | 0 |
| corpus/C-CDA-Examples/Allergies/No Section Information Allergies/No Information Allergies(C-CDA2.1).xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Allergies/Not Allergic to Peanuts/Not Allergic to Peanuts(C-CDAR2.1).xml | 1/1 | 1/1 | 58.8% | 0 |
| corpus/C-CDA-Examples/Allergies/Withdrawn Allergy to specific drug Epinephrine/Withdrawn_Allergy to specific drug Epinephrine(C-CDA2.1).xml | 1/1 | 1/1 | 42.4% | 0 |
| corpus/C-CDA-Examples/Care Team/Care Team Narrative Only/Care Team Narrative Only (C-CDAR2.1).xml | 0/1 | 0/0 | — | 0 |
| corpus/C-CDA-Examples/Care Team/Care Team Structured Entry/Care Team Organizer Example.xml | 0/1 | 0/1 | — | 0 |
| corpus/C-CDA-Examples/Documents/CCD/CCD 1/CCD.XML | 13/15 | 27/31 | 64.7% | 5 |
| corpus/C-CDA-Examples/Documents/CCD/CCD 2/CCD.xml | 7/7 | 5/7 | 71.7% | 11 |
| corpus/C-CDA-Examples/Documents/Care Plan/Care_Plan.xml | 2/4 | 2/7 | 66.0% | 2 |
| corpus/C-CDA-Examples/Documents/Consultation Note/Consultation_Note.xml | 11/13 | 18/21 | 73.8% | 18 |
| corpus/C-CDA-Examples/Documents/Diagnostic Imaging Report/Diagnostic_Imaging_Report.xml | 0/5 | 0/5 | 98.3% | 0 |
| corpus/C-CDA-Examples/Documents/Discharge Summary/Discharge_Summary.xml | 20/21 | 16/16 | 74.4% | 3 |
| corpus/C-CDA-Examples/Documents/History and Physical/History_and_Physical.xml | 15/17 | 22/24 | 64.7% | 8 |
| corpus/C-CDA-Examples/Documents/Operative Note/Operative_Note.xml | 10/16 | 9/10 | 94.7% | 0 |
| corpus/C-CDA-Examples/Documents/Procedure Note/Procedure_Note.xml | 9/16 | 8/10 | 94.7% | 2 |
| corpus/C-CDA-Examples/Documents/Progress Note/Progress_Note.xml | 11/12 | 13/15 | 69.7% | 7 |
| corpus/C-CDA-Examples/Documents/Referral Note/Referral_Note.xml | 15/19 | 23/31 | 59.8% | 15 |
| corpus/C-CDA-Examples/Documents/Transfer Summary/Transfer_Summary.xml | 21/27 | 38/48 | 67.4% | 20 |
| corpus/C-CDA-Examples/Encounters/Hospital Discharge Encounter with Billable Diagnoses/Hospital Discharge Encounter with Billable Diagnoses(C-CDA2.1).xml | 1/1 | 1/1 | 76.0% | 0 |
| corpus/C-CDA-Examples/Encounters/Inpatient Encounter Discharged to Rehab Location/Inpatient_Encounter_Discharged_to_Rehab_Location(C-CDA2.1).xml | 1/1 | 1/1 | 62.5% | 4 |
| corpus/C-CDA-Examples/Encounters/Multiple CPT E&M Codes/Multiple CPT E&M Codes Single Encounter(C-CDAR2.1).xml | 1/1 | 1/1 | 85.3% | 0 |
| corpus/C-CDA-Examples/Encounters/Outpatient Encounter Patient Disenrolled/Outpatient Encounter Patient Disenrolled(C-CDA2.1).xml | 1/1 | 1/1 | 82.4% | 0 |
| corpus/C-CDA-Examples/Encounters/Outpatient Encounter with Diagnoses/Outpatient Encounter with Diagnoses(C-CDA R2.1).xml | 1/1 | 1/1 | 95.5% | 0 |
| corpus/C-CDA-Examples/Family History/Family History Generic/Family History generic(C-CDA2.1).xml | 1/1 | 1/1 | 81.8% | 0 |
| corpus/C-CDA-Examples/Family History/Family History two individuals same relationship to the patient/Family History two individuals same relationship to the patient(C-CDA2.1).xml | 1/1 | 2/2 | 78.3% | 0 |
| corpus/C-CDA-Examples/Family History/Normal Family History Father deceased-Mother alive/Normal Family History Father deceased Mother alive(C-CDA2.1).xml | 1/1 | 2/2 | 76.9% | 0 |
| corpus/C-CDA-Examples/Functional Status/Functional Assessment - Glasgow Coma/Glasgow coma scale(C-CDAR2.1).xml | 1/1 | 1/1 | 85.0% | 0 |
| corpus/C-CDA-Examples/Functional Status/Functional Impairment/Functional Impairment Dependent on Walking Stick (C-CDAR2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Functional Status/No Functional Impairment/No Functional Impairment (C-CDAR2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/General/External Document Reference/External Document Reference QRDA Template (C-CDAR2.1).xml | 1/1 | 1/1 | 84.2% | 0 |
| corpus/C-CDA-Examples/General/External Document Reference/PROBLEMS_in_Empty_C-CDA_2.1 (C-CDAR2.1).xml | 19/20 | 10/10 | 84.3% | 0 |
| corpus/C-CDA-Examples/General/Narrative Reference - Act/Act text and originalText references (C-CDAR2.1).xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/General/Narrative Reference - Encounter/Encounter text and originalText references (C-CDAR2.1).xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/General/Narrative Reference - Observation/Observation text and originalText references (C-CDAR2.1).xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/General/Narrative Reference - Organizer/Organizer text and originalText references (C-CDAR2.1).xml | 0/1 | 0/1 | — | 0 |
| corpus/C-CDA-Examples/General/Narrative Reference - Procedure/Procedure text and originalText references (C-CDAR2.1).xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/General/Narrative Reference - SubstanceAdministration/SubstanceAdministration text and originalText references (C-CDAR2.1).xml | 0/1 | 0/1 | — | 0 |
| corpus/C-CDA-Examples/General/Narrative Reference - Supply/Supply text and originalText references (C-CDAR2.1).xml | 0/1 | 0/1 | — | 0 |
| corpus/C-CDA-Examples/General/No Section Information Problems/No Information Problems(C-CDA2.1).xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/General/Parent Document Replace Relationship/CCD Parent Document Replace (C-CDAR2.1).xml | 19/20 | 9/9 | 88.0% | 0 |
| corpus/C-CDA-Examples/General/Share With Patient or Proxy/Share with Patient or Proxy (C-CDAR2.1).xml | 1/1 | 1/1 | 25.0% | 0 |
| corpus/C-CDA-Examples/Goals/Goals Narrative Only/Goals Narrative Only(C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Goals/SDOH Goal/SDOH Goal(C-CDA2.1).xml | 1/1 | 1/1 | 70.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Admission Diagnosis Section (V3)_2.16.840.1.113883.10.20.22.2.43/Admission Diagnosis Section (V3) Example.xml | 1/1 | 0/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Advance Directives Section (entries required) (V3)_2.16.840.1.113883.10.20.22.2.21.1/Advance Directives Section (V3) Example.xml | 0/1 | 0/2 | — | 0 |
| corpus/C-CDA-Examples/Guide Examples/Allergies and Intolerances Section (entries required) (V3)_2.16.840.1.113883.10.20.22.2.6.1/Allergies and Intolerances Section (entries required) (V3) Example.xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Anesthesia Section (V2)_2.16.840.1.113883.10.20.22.2.25/Anesthesia Section (V2) Example.xml | 0/1 | 2/2 | — | 4 |
| corpus/C-CDA-Examples/Guide Examples/Assessment Section_2.16.840.1.113883.10.20.22.2.8/Assessment Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Assessment and Plan Section (V2)_2.16.840.1.113883.10.20.22.2.9/Assessment and Plan Section (V2) Example.xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Care Team Member Act (V2)_2.16.840.1.113883.10.20.22.4.500.1/Care Team Member Act Example.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/C-CDA-Examples/Guide Examples/Care Team Member Act_2.16.840.1.113883.10.20.22.4.500.1/Care Team Member Act.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/C-CDA-Examples/Guide Examples/Care Team Organizer (V2)_2.16.840.1.113883.10.20.22.4.500/Care Team Organizer Example.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/C-CDA-Examples/Guide Examples/Care Team Organizer_2.16.840.1.113883.10.20.22.4.500/Care Team Organizer Example 1.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/C-CDA-Examples/Guide Examples/Care Teams Section (V2)_2.16.840.1.113883.10.20.22.2.500/Care Teams Section Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Care Teams Section_2.16.840.1.113883.10.20.22.2.500/Care Teams Section.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Chief Complaint Section_1.3.6.1.4.1.19376.1.5.3.1.1.13.2.1/Chief Complaint Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Chief Complaint and Reason for Visit Section_2.16.840.1.113883.10.20.22.2.13/Chief Complaint and Reason for Visit Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Complications Section (V3)_2.16.840.1.113883.10.20.22.2.37/Complications Section (V3) Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Consultation Note (V3)_2.16.840.1.113883.10.20.22.1.4/Consultation Note structuredBody Example.xml | 11/12 | 0/0 | 100.0% | 12 |
| corpus/C-CDA-Examples/Guide Examples/Course of Care Section_2.16.840.1.113883.10.20.22.2.64/Course of Care Section Example.xml | 1/1 | 0/0 | 0.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/DICOM Object Catalog Section - DCM 121181_2.16.840.1.113883.10.20.6.1.1/DICOM Object Catalog Section - DCM 121181 Example.xml | 0/1 | 0/1 | — | 3 |
| corpus/C-CDA-Examples/Guide Examples/Discharge Diagnosis Section (V3)_2.16.840.1.113883.10.20.22.2.24/Discharge Diagnosis Section (V3) Example.xml | 1/1 | 0/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Discharge Medications Section (entries required) (V3)_2.16.840.1.113883.10.20.22.2.11.1/Discharge Medication Section (V3) (entries required) Example.xml | 1/1 | 0/1 | 100.0% | 2 |
| corpus/C-CDA-Examples/Guide Examples/Encounters Section (entries required) (V3)_2.16.840.1.113883.10.20.22.2.22.1/Encounters Section (entries required) (V3) Example.xml | 1/1 | 0/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Entry Reference_2.16.840.1.113883.10.20.22.4.122/Diagnosis Reference Example.xml | 0/0 | 0/0 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Entry Reference_2.16.840.1.113883.10.20.22.4.122/Entry Reference Example.xml | — | — | — | error: to FHIR: 37:26: not well-formed XML: an unknown namespace prefix 'xsi' at 37:26 |
| corpus/C-CDA-Examples/Guide Examples/Family History Section (V3)_2.16.840.1.113883.10.20.22.2.15/Family History Section (V3) Example.xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Findings Section (DIR)_2.16.840.1.113883.10.20.6.1.2/Findings Section (DIR) Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Functional Status Section (V2)_2.16.840.1.113883.10.20.22.2.14/Functional Status Section (V2) Example.xml | 1/1 | 4/5 | 100.0% | 5 |
| corpus/C-CDA-Examples/Guide Examples/General Status Section_2.16.840.1.113883.10.20.2.5/General Status Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Goal Observation (V2)_2.16.840.1.113883.10.20.22.4.121/Social Determinant of Health Text Goal Example.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Goals Section_2.16.840.1.113883.10.20.22.2.60/Goals Section Example.xml | 1/1 | 0/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Health Concerns Section (V2)_2.16.840.1.113883.10.20.22.2.58/Health Concerns Section Example.xml | 1/1 | 0/2 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Health Status Evaluations and Outcomes Section_2.16.840.1.113883.10.20.22.2.61/Health Status Evaluations and Outcomes Section Example.xml | 0/1 | 0/2 | — | 3 |
| corpus/C-CDA-Examples/Guide Examples/History of Present Illness Section_1.3.6.1.4.1.19376.1.5.3.1.3.4/History of Present Illness Section Example.xml | 0/1 | 0/0 | — | 0 |
| corpus/C-CDA-Examples/Guide Examples/Hospital Consultations Section_2.16.840.1.113883.10.20.22.2.42/Hospital Consultations Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Hospital Course Section_1.3.6.1.4.1.19376.1.5.3.1.3.5/Hospital Course Section Example.xml | 1/1 | 0/0 | 0.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Hospital Discharge Instructions Section_2.16.840.1.113883.10.20.22.2.41/Hospital Discharge Instructions Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Hospital Discharge Physical Section_1.3.6.1.4.1.19376.1.5.3.1.3.26/Hospital Discharge Physical Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Hospital Discharge Studies Summary Section_2.16.840.1.113883.10.20.22.2.16/Hospital Discharge Studies Summary Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Immunizations Section (entries required) (V3)_2.16.840.1.113883.10.20.22.2.2.1/Immunizations Section (entries required) (V3) Example.xml | 1/1 | 1/1 | 100.0% | 2 |
| corpus/C-CDA-Examples/Guide Examples/Instructions Section (V2)_2.16.840.1.113883.10.20.22.2.45/Instructions Section (V2) Example.xml | 0/1 | 1/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Interventions Section (V3)_2.16.840.1.113883.10.20.21.2.3/Interventions Section (V3) Example.xml | 0/1 | 0/1 | — | 0 |
| corpus/C-CDA-Examples/Guide Examples/Medical Equipment Section (V2)_2.16.840.1.113883.10.20.22.2.23/Medical Equipment Section (V2) Example.xml | 1/1 | 3/3 | 100.0% | 4 |
| corpus/C-CDA-Examples/Guide Examples/Medications Administered Section (V2)_2.16.840.1.113883.10.20.22.2.38/Medications Administered Section (V2) Example.xml | 0/1 | 1/1 | — | 2 |
| corpus/C-CDA-Examples/Guide Examples/Medications Section (entries required) (V2)_2.16.840.1.113883.10.20.22.2.1.1/Medications Section (entries required) (V2) Example.xml | 1/1 | 1/1 | 100.0% | 2 |
| corpus/C-CDA-Examples/Guide Examples/Mental Status Section (V2)_2.16.840.1.113883.10.20.22.2.56/Mental Status Section Example.xml | 1/1 | 2/3 | 50.0% | 3 |
| corpus/C-CDA-Examples/Guide Examples/Note Activity_2.16.840.1.113883.10.20.22.4.202/Note Activity as Standalone Entry.xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Note Activity_2.16.840.1.113883.10.20.22.4.202/Note Activity as entryRelationship to C-CDA Entry.xml | 1/1 | 1/1 | 45.5% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Note Activity_2.16.840.1.113883.10.20.22.4.202/RTF Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Notes Section_2.16.840.1.113883.10.20.22.2.65/Note Section Example.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Nutrition Section_2.16.840.1.113883.10.20.22.2.57/Nutrition Section Example.xml | 0/1 | 0/1 | — | 2 |
| corpus/C-CDA-Examples/Guide Examples/Objective Section_2.16.840.1.113883.10.20.21.2.1/Objective Section Example.xml | 1/1 | 0/0 | 0.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Operative Note Fluids Section_2.16.840.1.113883.10.20.7.12/Operative Note Fluids Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Operative Note Surgical Procedure Section_2.16.840.1.113883.10.20.7.14/Operative Note Surgical Procedure Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Past Medical History (V3)_2.16.840.1.113883.10.20.22.2.20/Past Medical History (V3) Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Payers Section (V3)_2.16.840.1.113883.10.20.22.2.18/Payers Section (V3) Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Physical Exam Section (V3)_2.16.840.1.113883.10.20.2.10/Physical Exam Section (V3) Example.xml | 0/1 | 0/0 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Plan of Treatment Section (V2)_2.16.840.1.113883.10.20.22.2.10/Plan of Treatment Section (V2) Example.xml | 1/1 | 1/2 | 100.0% | 2 |
| corpus/C-CDA-Examples/Guide Examples/Planned Procedure Section (V2)_2.16.840.1.113883.10.20.22.2.30/Planned Procedure Section (V2) Example.xml | 0/1 | 1/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Postoperative Diagnosis Section_2.16.840.1.113883.10.20.22.2.35/Postoperative Diagnosis Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Postprocedure Diagnosis (V3)_2.16.840.1.113883.10.20.22.4.51/Postprocedure Diagnosis (V3) Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Postprocedure Diagnosis Section (V3)_2.16.840.1.113883.10.20.22.2.36/Postprocedure Diagnosis Section (V3) Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Preoperative Diagnosis Section (V3)_2.16.840.1.113883.10.20.22.2.34/Preoperative Diagnosis Section (V3) Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Problem Section (entries required) (V3)_2.16.840.1.113883.10.20.22.2.5.1/No Known Problems Section Example.xml | 1/1 | 1/1 | 42.9% | 2 |
| corpus/C-CDA-Examples/Guide Examples/Problem Section (entries required) (V3)_2.16.840.1.113883.10.20.22.2.5.1/Problem Section (entries required) (V3) Example.xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Procedure Description Section_2.16.840.1.113883.10.20.22.2.27/Procedure Description Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Procedure Disposition Section_2.16.840.1.113883.10.20.18.2.12/Procedure Disposition Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Procedure Estimated Blood Loss Section_2.16.840.1.113883.10.20.18.2.9/Procedure Estimated Blood Loss Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Procedure Findings Section (V3)_2.16.840.1.113883.10.20.22.2.28/Procedure Findings Section (V3) Example.xml | 0/1 | 1/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Procedure Implants Section_2.16.840.1.113883.10.20.22.2.40/Procedure Implants Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Procedure Indications Section (V2)_2.16.840.1.113883.10.20.22.2.29/Procedure Indications Section (V2) Example.xml | 0/1 | 1/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Procedure Specimens Taken Section_2.16.840.1.113883.10.20.22.2.31/Procedure Specimens Taken Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Procedures Section (entries required) (V2)_2.16.840.1.113883.10.20.22.2.7.1/Procedures Section (entries required) (V2) Example.xml | 1/1 | 1/2 | 100.0% | 4 |
| corpus/C-CDA-Examples/Guide Examples/Reason for Referral Section (V2)_1.3.6.1.4.1.19376.1.5.3.1.3.1/Reason for Referral Section (V2) Example.xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Guide Examples/Reason for Visit Section_2.16.840.1.113883.10.20.22.2.12/Reason for Visit Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Results Section (entries required) (V3)_2.16.840.1.113883.10.20.22.2.3.1/Results Section (entries required) (V3) Example.xml | 1/1 | 1/1 | 100.0% | 2 |
| corpus/C-CDA-Examples/Guide Examples/Review of Systems Section_1.3.6.1.4.1.19376.1.5.3.1.3.18/Review of Systems Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Social History Section (V3)_2.16.840.1.113883.10.20.22.2.17/Social History Section (V3) Example.xml | 1/1 | 5/5 | 100.0% | 5 |
| corpus/C-CDA-Examples/Guide Examples/Subjective Section_2.16.840.1.113883.10.20.21.2.2/Subjective Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Surgical Drains Section_2.16.840.1.113883.10.20.7.13/Surgical Drains Section Example.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Guide Examples/Text Observation_2.16.840.1.113883.10.20.6.2.12/Text Observation Example.xml | 0/1 | 0/1 | — | 1 |
| corpus/C-CDA-Examples/Guide Examples/Transfer Summary (V2)_2.16.840.1.113883.10.20.22.1.13/Transfer Summary participant (Support) Example.xml | — | — | — | error: to FHIR: 4:11: not well-formed XML: an unknown namespace prefix 'xsi' at 4:11 |
| corpus/C-CDA-Examples/Guide Examples/US Realm Header (V3)_2.16.840.1.113883.10.20.22.1.1/Supporting Person participant Example.xml | 0/0 | 0/0 | — | 2 |
| corpus/C-CDA-Examples/Guide Examples/US Realm Header (V3)_2.16.840.1.113883.10.20.22.1.1/US Realm Header (V3) Example.xml | 0/0 | 0/0 | — | 12 |
| corpus/C-CDA-Examples/Guide Examples/US Realm Header for Patient Generated Document (V2)_2.16.840.1.113883.10.20.29.1/Patient Generated Document informationRecipient.xml | 0/0 | 0/0 | — | 2 |
| corpus/C-CDA-Examples/Guide Examples/Vital Signs Section (entries required) (V3)_2.16.840.1.113883.10.20.22.2.4.1/Vital Signs Section (entries required) (V3) Example.xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Header/Direct Address/Patient and Provider Organization Direct Address(C-CDAR2.1).xml | 0/0 | 0/0 | 88.5% | 6 |
| corpus/C-CDA-Examples/Health Concerns/Health Concerns Link to Problems Section/Health Concerns Link to Problems Section(C-CDA2.1).xml | — | — | — | error: to FHIR: 61:44: not well-formed XML: an unknown namespace prefix 'xsi' at 61:44 |
| corpus/C-CDA-Examples/Health Concerns/Health Concerns Link to Problems Section with linkHTML/Health Concerns Link to Problems Section with linkHTML(C-CDA2.1).xml | — | — | — | error: to FHIR: 61:44: not well-formed XML: an unknown namespace prefix 'xsi' at 61:44 |
| corpus/C-CDA-Examples/Health Concerns/Health Concerns Narrative Only/Health Concerns Narrative Only(C-CDA2.1).xml | 1/1 | 1/1 | 50.0% | 0 |
| corpus/C-CDA-Examples/Health Concerns/No Known Health Concerns/No Known Health Concerns(C-CDA2.1).xml | 1/1 | 1/1 | 6.7% | 0 |
| corpus/C-CDA-Examples/Immunizations/Immunization not given Patient refused/Immunization not given Patient refused(C-CDA2.1).xml | 1/1 | 1/1 | 83.3% | 0 |
| corpus/C-CDA-Examples/Immunizations/Influenza Vaccination/Influenza Vaccination(C-CDA2.1).xml | 1/1 | 1/1 | 94.4% | 0 |
| corpus/C-CDA-Examples/Immunizations/Influenza Vaccination - Patient Reported/Influenza Vaccination Patient Reported(C-CDA2.1).xml | 1/1 | 1/1 | 50.0% | 0 |
| corpus/C-CDA-Examples/Immunizations/Influenza Vaccination with NDC/Influenza Vaccination with NDC(C-CDA2.1).xml | 1/1 | 1/1 | 94.7% | 0 |
| corpus/C-CDA-Examples/Immunizations/No Section Information Immunizations/No Information Immunizations(C-CDA2.1).xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Immunizations/Unknown Immunization Status/Unknown Immunization Status(C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Interventions/Intervention Counseling/Intervention Counseling Procedure Activity Act(C-CDR2.1).xml | 0/1 | 0/1 | — | 0 |
| corpus/C-CDA-Examples/Medical Equipment/Implant UDI Organizer/Implant UDI Organizer (C-CDA R2.1).xml | 1/1 | 1/1 | 60.9% | 5 |
| corpus/C-CDA-Examples/Medical Equipment/Implant UDI Unknown/Implant UDI Unknown (C-CDA R2.1).xml | 1/1 | 1/1 | 54.2% | 0 |
| corpus/C-CDA-Examples/Medical Equipment/Implant Without Procedure/Implant Without Procedure(C-CDA2.1).xml | 1/1 | 3/3 | 88.2% | 0 |
| corpus/C-CDA-Examples/Medical Equipment/Multiple Implants/Multiple Implants (C-CDA R2.1).xml | 1/1 | 2/2 | 100.0% | 0 |
| corpus/C-CDA-Examples/Medical Equipment/No Implanted Devices/No Implanted Devices(C-CDA2.1).xml | 1/1 | 1/1 | 50.0% | 0 |
| corpus/C-CDA-Examples/Medical Equipment/Non-Medicinal Supply - Cane and Eyeglasses/Non-Medicinal Supply - Cane and Eyeglasses (C-CDAR2.1).xml | 1/1 | 2/2 | 10.0% | 0 |
| corpus/C-CDA-Examples/Medications/Drug Mixture/Drug Mixture (C-CDAR2.1).xml | 1/1 | 1/1 | 100.0% | 4 |
| corpus/C-CDA-Examples/Medications/Free Text Medication SIG/Free Text Medication SIG (C-CDAR2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Medications/Med Relative Dose IV Drug/Med Relative Dose IV Drug(C-CDA2.1).xml | 1/1 | 1/1 | 83.3% | 0 |
| corpus/C-CDA-Examples/Medications/Med at bedtime/Med at bedtime(C-CDAR2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Medications/Med every 4-6 hours/Med every 4-6 hours(C-CDA2.1).xml | 1/1 | 1/1 | 90.0% | 0 |
| corpus/C-CDA-Examples/Medications/Med oral QID with PRN/Med oral QID with PRN(C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Medications/Med oral liquid PRN/Med oral liquid PRN(C-CDA2.1).xml | 1/1 | 1/1 | 81.8% | 0 |
| corpus/C-CDA-Examples/Medications/Med oral with indications and instructions/Med oral with indications and instructions(C-CDA2.1).xml | 1/1 | 3/3 | 90.0% | 1 |
| corpus/C-CDA-Examples/Medications/Medication Refused/Medication Refused(C-CDAR2.1).xml | 1/1 | 1/1 | 81.0% | 0 |
| corpus/C-CDA-Examples/Medications/No Medications/No Medications(C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Medications/Single administration of medication/Single administration of medication at single point in time(C-CDA2.1).xml | 1/1 | 1/1 | 88.2% | 0 |
| corpus/C-CDA-Examples/Medications/Withdrawn Anitbiotics with varied dosing/Withdrawn Antibiotics with varied dosing(C-CDA2.1).xml | 1/1 | 0/1 | 5.9% | 0 |
| corpus/C-CDA-Examples/Medications/Withdrawn Patient reported oral medication/Withdrawn Patient reported oral medication no brand(C-CDA2.1).xml | 1/1 | 1/1 | 69.2% | 0 |
| corpus/C-CDA-Examples/Mental Status/Memory Impairment/Memory Impairment(C-CDAR2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Mental Status/No Cognitive Impairment/No Cognitive Impairment(C-CDAR2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Mental Status/Patient Health Questionnaire PHQ-9/Patient Health Questionnaire PHQ-9(C-CDAR2.1).xml | 1/1 | 1/1 | 80.6% | 0 |
| corpus/C-CDA-Examples/Notes/Discharge Note in Hospital Course/Discharge Note(C-CDAR2.1).xml | 1/1 | 1/1 | 71.4% | 2 |
| corpus/C-CDA-Examples/Notes/Note directly attached to a Procedure/Note directly attached to a Procedure(C-CDAR2.1).xml | 1/1 | 1/1 | 45.5% | 1 |
| corpus/C-CDA-Examples/Notes/RTF Note/RTF Note (C-CDA R2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Notes/Single Consultation Note/Single Consultation Note(C-CDA R2.1).xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/C-CDA-Examples/Plan of Treatment/Care Plan Goals and Instructions/Care Plan Goals and Instructions(C-CDAR2.1).xml | 1/1 | 2/2 | 38.5% | 0 |
| corpus/C-CDA-Examples/Plan of Treatment/No Planned Tests/No Planned Tests(C-CDAR2.1).xml | 1/1 | 2/2 | 71.4% | 0 |
| corpus/C-CDA-Examples/Plan of Treatment/Planned EKG/Planned EKG(C-CDAR2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Plan of Treatment/Planned Encounter - Referral/Planned Encounter - Reason for Referral(C-CDAR2.1).xml | — | — | — | error: to FHIR: 136:44: not well-formed XML: an unknown namespace prefix 'xsi' at 136:44 |
| corpus/C-CDA-Examples/Problems/Active Problem/Active Problem(C-CDA2.1).xml | 1/1 | 1/1 | 84.2% | 0 |
| corpus/C-CDA-Examples/Problems/Complete or Resolved Problem/Resolved Problem(C-CDA2.1).xml | 1/1 | 1/1 | 87.0% | 0 |
| corpus/C-CDA-Examples/Problems/No Known Problems/No Known Problems(C-CDA2.1).xml | 1/1 | 1/1 | 83.3% | 0 |
| corpus/C-CDA-Examples/Problems/Patient Does Not Have Diabetes/Patient Does Not Have Diabetes(C-CDA2.1).xml | 1/1 | 1/1 | 78.9% | 1 |
| corpus/C-CDA-Examples/Problems/Pregnancy with Week Gestation/Pregnancy with Week Gestation(C-CDA2.1).xml | 1/1 | 1/1 | 81.8% | 0 |
| corpus/C-CDA-Examples/Problems/Problem TargetSiteCode Qualifier/Problem TargetSiteCode Qualifier(C-CDAR2.1).xml | 1/1 | 1/1 | 54.5% | 0 |
| corpus/C-CDA-Examples/Problems/Problem Value, Translation, Qualifier examples/Problem List with different translation and qualifier usages(C-CDA2.1).xml | 1/1 | 1/1 | 72.0% | 0 |
| corpus/C-CDA-Examples/Problems/SDOH Problem/SDOH Problem(C-CDA2.1).xml | 1/1 | 1/1 | 86.4% | 2 |
| corpus/C-CDA-Examples/Procedures/Procedure Alternatives/Procedure Alternatives(C-CDA2.1).xml | 1/1 | 1/1 | 56.2% | 0 |
| corpus/C-CDA-Examples/Procedures/Procedure Refused/Procedures Section Procedure Procedure Refused(C-CDA2.1).xml | 1/1 | 1/1 | 58.3% | 0 |
| corpus/C-CDA-Examples/Procedures/Procedure Unsuccessful/Procedures Section Procedure Procedure Unsuccessful(C-CDA2.1).xml | 1/1 | 1/1 | 36.4% | 0 |
| corpus/C-CDA-Examples/Procedures/Procedures Section Act Entry/Procedures Section Act(C-CDA2.1).xml | 1/1 | 0/1 | 12.5% | 0 |
| corpus/C-CDA-Examples/Procedures/Procedures Section Observation Entry/Procedures Section Observation(C-CDA2.1).xml | 1/1 | 0/1 | 11.1% | 0 |
| corpus/C-CDA-Examples/Procedures/Procedures Section Procedure Entry/Procedures Section Procedure(C-CDA2.1).xml | 1/1 | 1/1 | 80.0% | 0 |
| corpus/C-CDA-Examples/Procedures/Procedures Section Procedure Entry - Colonoscopy/Procedures Section Procedure Colonoscopy (C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Quality/Quality Care Compliance in C-CDA/Quality Care Compliance in C-CDA (C-CDAR2.1).xml | 0/1 | 0/4 | — | 0 |
| corpus/C-CDA-Examples/Referrals - Planned and Completed/Close Referral with a Document/Consult Document Closing Referral(C-CDAR2.1).xml | 0/0 | 0/0 | 96.7% | 2 |
| corpus/C-CDA-Examples/Referrals - Planned and Completed/Referral Request and Close Referral with a Note/Intervention Planned Referral and Note Completeting Referral(C-CDAR2.1).xml | 0/0 | 0/0 | — | 1 |
| corpus/C-CDA-Examples/Results/Result COVID Positive/Results of COVID Test Positive Normal(C-CDA2.1).xml | 1/1 | 1/1 | 58.3% | 0 |
| corpus/C-CDA-Examples/Results/Result panel with coded values of negative-positive/Result panel with coded values of negative-positive(C-CDA2.1).xml | 1/1 | 1/1 | 52.4% | 0 |
| corpus/C-CDA-Examples/Results/Result with Multiple Reference Ranges/Lab with Multiple Reference Ranges(C-CDA2.1).xml | 1/1 | 1/1 | 60.0% | 0 |
| corpus/C-CDA-Examples/Results/Result with an unstructured string as value (urine color)/Result with an unstructured string as value urine color (C-CDA2.1).xml | 1/1 | 1/1 | 77.8% | 0 |
| corpus/C-CDA-Examples/Results/Result with greater than a specified value/Result with greater than a specified value(C-CDA2.1).xml | 1/1 | 1/1 | 62.5% | 2 |
| corpus/C-CDA-Examples/Results/Result with lab location/Result with lab location(C-CDAR2.1).xml | 1/1 | 1/1 | 68.8% | 0 |
| corpus/C-CDA-Examples/Results/Result with non-numeric physical quantity and unit/Result with non-numeric physical quantity and unit(C-CDA2.1).xml | 1/1 | 1/1 | 57.1% | 0 |
| corpus/C-CDA-Examples/Results/Results Radiology with Image Narrative/Chest X ray with Narrative Report(C-CDA2.1).xml | 1/1 | 1/1 | 94.1% | 0 |
| corpus/C-CDA-Examples/Results/Results Unit Non-UCUM/Results Unit Non-UCUM(C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Results/Results of Basic Metabolic Panel and Troponin/Basic Metabolic Panel with Troponin(C-CDA2.1).xml | 1/1 | 3/3 | 86.4% | 0 |
| corpus/C-CDA-Examples/Results/Results of CO2 Test/Results of CO2 Test Normal(C-CDA2.1).xml | 1/1 | 1/1 | 90.9% | 0 |
| corpus/C-CDA-Examples/Results/Results panel with pending component/Results panel with pending component(C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Results/Results with less than specific value/Results with less than specific value(C-CDA2.1).xml | 1/1 | 1/1 | 78.6% | 0 |
| corpus/C-CDA-Examples/Results/Results with translation unit/Results with translation unit(C-CDA2.1).xml | 1/1 | 1/1 | 62.5% | 0 |
| corpus/C-CDA-Examples/Social History/Current Smoking Status/Current Smoking Status(C-CDA2.1).xml | 1/1 | 2/2 | 84.2% | 0 |
| corpus/C-CDA-Examples/Social History/Electronic Cigarette/Electronic Cigarette Status(C-CDA2.1).xml | 1/1 | 1/1 | 83.3% | 1 |
| corpus/C-CDA-Examples/Social History/Former Smoking Status/Former Smoking Status(C-CDA2.1).xml | 1/1 | 2/2 | 80.0% | 0 |
| corpus/C-CDA-Examples/Social History/Never Smoking Status/Never Smoking Status(C-CDA2.1).xml | 1/1 | 1/1 | 80.0% | 0 |
| corpus/C-CDA-Examples/Social History/Not Pregnant/Not Pregnant (C-CDAR2.1).xml | 1/1 | 1/1 | 75.0% | 0 |
| corpus/C-CDA-Examples/Social History/Sexual Orientation Gender Identity/Sexual Orientation Gender Identity(C-CDA2.1).xml | 1/1 | 2/2 | 57.9% | 0 |
| corpus/C-CDA-Examples/Social History/Unknown Smoking Status/Unknown Smoking Status(C-CDA2.1).xml | 1/1 | 1/1 | 80.0% | 0 |
| corpus/C-CDA-Examples/Social History/Unknown if Pregnant/Unknown if Pregnant.xml | 1/1 | 1/1 | 75.0% | 0 |
| corpus/C-CDA-Examples/Social History/WE Care Assessment/WE Care Assessment(C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Unstructured/CDA reference PDF/Unstructured_Document_reference.xml | 0/0 | 0/0 | 95.1% | 0 |
| corpus/C-CDA-Examples/Unstructured/CDA with Embedded PDF 1/Unstructured_Document_embed.xml | 0/0 | 0/0 | 100.0% | 0 |
| corpus/C-CDA-Examples/Unstructured/CDA with Embedded PDF 2/CDA_with_Embedded_PDF.xml | 0/0 | 0/0 | 84.3% | 0 |
| corpus/C-CDA-Examples/Unstructured/CDA with Embedded Text plain/CDA_Embedded_Text_Plain_Surgical_Consult.xml | 0/0 | 0/0 | 94.9% | 0 |
| corpus/C-CDA-Examples/Vital Signs/Growth Charts Examples/Growth Charts Example.xml | 1/1 | 2/2 | 100.0% | 0 |
| corpus/C-CDA-Examples/Vital Signs/Heart Rate Rhythm/Heart Rate Rhythm(C-CDA2.1).xml | 1/1 | 1/1 | 69.2% | 0 |
| corpus/C-CDA-Examples/Vital Signs/Panel of Vital Signs (Oxygen Concentration Included)/Panel of Vital Signs (Oxygen Concentration Included) (C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Vital Signs/Panel of Vital Signs in Metric Units/Panel of Vital Signs in Metric Units(C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/C-CDA-Examples/Vital Signs/Panel of Vital Signs in Mixed Metric-Imperial Units/Panel of Vital Signs in Mixed Metric-Imperial Units(C-CDA2.1).xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/allergies-allergy-to-food-egg.xml | 1/1 | 1/1 | 66.7% | 0 |
| corpus/CDA-Examples/input/examples/allergies-allergy-to-latex.xml | 1/1 | 1/1 | 66.7% | 2 |
| corpus/CDA-Examples/input/examples/allergies-allergy-to-specific-drug-class-penicillins.xml | 1/1 | 1/1 | 66.7% | 0 |
| corpus/CDA-Examples/input/examples/allergies-allergy-to-specific-drug-codeine.xml | 1/1 | 1/1 | 48.6% | 0 |
| corpus/CDA-Examples/input/examples/allergies-allergy-to-specific-drug-penicillin.xml | 1/1 | 1/1 | 66.7% | 0 |
| corpus/CDA-Examples/input/examples/allergies-free-text-allergy-to-clinical-trial-drug.xml | 1/1 | 1/1 | 63.6% | 0 |
| corpus/CDA-Examples/input/examples/allergies-no-known-allergies.xml | 1/1 | 1/1 | 40.0% | 4 |
| corpus/CDA-Examples/input/examples/allergies-no-known-medication-allergies.xml | 1/1 | 1/1 | 53.3% | 0 |
| corpus/CDA-Examples/input/examples/allergies-no-section-information-allergies.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/allergies-not-allergic-to-peanuts.xml | 1/1 | 1/1 | 58.8% | 0 |
| corpus/CDA-Examples/input/examples/care-team-narrative-only.xml | 0/1 | 0/0 | — | 0 |
| corpus/CDA-Examples/input/examples/care-team-structured-entry.xml | 0/1 | 0/1 | — | 0 |
| corpus/CDA-Examples/input/examples/encounters-hospital-discharge-encounter-with-billable-diagnoses.xml | 1/1 | 1/1 | 76.0% | 0 |
| corpus/CDA-Examples/input/examples/encounters-inpatient-encounter-discharged-to-rehab-location.xml | 1/1 | 1/1 | 62.5% | 0 |
| corpus/CDA-Examples/input/examples/encounters-multiple-cpt-e-m-codes.xml | 1/1 | 1/1 | 85.3% | 0 |
| corpus/CDA-Examples/input/examples/encounters-outpatient-encounter-patient-disenrolled.xml | 1/1 | 1/1 | 82.4% | 0 |
| corpus/CDA-Examples/input/examples/encounters-outpatient-encounter-with-diagnoses.xml | 1/1 | 1/1 | 95.5% | 0 |
| corpus/CDA-Examples/input/examples/family-history-generic.xml | 1/1 | 1/1 | 81.8% | 0 |
| corpus/CDA-Examples/input/examples/family-history-normal-family-history-father-deceased-mother-aliv.xml | 1/1 | 2/2 | 76.9% | 0 |
| corpus/CDA-Examples/input/examples/family-history-two-individuals-same-relationship-to-the-patient.xml | 1/1 | 2/2 | 78.3% | 0 |
| corpus/CDA-Examples/input/examples/functional-status-disability-status-acs.xml | 1/1 | 1/1 | 13.6% | 0 |
| corpus/CDA-Examples/input/examples/functional-status-functional-assessment-glasgow-coma.xml | 1/1 | 1/1 | 85.0% | 0 |
| corpus/CDA-Examples/input/examples/functional-status-functional-impairment.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/functional-status-no-functional-impairment.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/general-external-document-reference.xml | 1/1 | 1/1 | 84.2% | 0 |
| corpus/CDA-Examples/input/examples/general-narrative-reference-act.xml | 0/1 | 0/1 | — | 0 |
| corpus/CDA-Examples/input/examples/general-narrative-reference-encounter.xml | 0/1 | 0/1 | — | 0 |
| corpus/CDA-Examples/input/examples/general-narrative-reference-observation.xml | 0/1 | 0/1 | — | 0 |
| corpus/CDA-Examples/input/examples/general-narrative-reference-organizer.xml | 0/1 | 0/1 | — | 0 |
| corpus/CDA-Examples/input/examples/general-narrative-reference-procedure.xml | 0/1 | 0/1 | — | 0 |
| corpus/CDA-Examples/input/examples/general-narrative-reference-substanceadministration.xml | 0/1 | 0/1 | — | 0 |
| corpus/CDA-Examples/input/examples/general-narrative-reference-supply.xml | 0/1 | 0/1 | — | 0 |
| corpus/CDA-Examples/input/examples/general-no-section-information-problems.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/general-parent-document-replace-relationship.xml | 19/20 | 9/9 | 88.0% | 0 |
| corpus/CDA-Examples/input/examples/general-share-with-patient-or-proxy.xml | 1/1 | 1/1 | 41.2% | 0 |
| corpus/CDA-Examples/input/examples/goals-narrative-only.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/goals-sdoh-goal.xml | 1/1 | 1/1 | 70.0% | 0 |
| corpus/CDA-Examples/input/examples/header-care-team-in-transition-of-care-documents.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/header-direct-address.xml | 0/1 | 0/0 | 88.5% | 4 |
| corpus/CDA-Examples/input/examples/header-masked-social-security-number.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/header-multiple-patient-identifiers.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/header-patient-aliases.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/header-patient-birth-name.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/header-patient-deceased.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/header-patient-demographic-information.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/header-patient-previous-name.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/header-patient-with-prior-addresses.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/header-person-name-formatting.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/health-concerns-link-to-problems-section-with-linkhtml.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/health-concerns-link-to-problems-section.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/health-concerns-narrative-only.xml | 1/1 | 1/1 | 50.0% | 0 |
| corpus/CDA-Examples/input/examples/health-concerns-no-known-health-concerns.xml | 1/1 | 1/1 | 6.7% | 0 |
| corpus/CDA-Examples/input/examples/immunizations-immunization-not-given-patient-refused.xml | 1/1 | 1/1 | 85.7% | 0 |
| corpus/CDA-Examples/input/examples/immunizations-influenza-vaccination-patient-reported.xml | 1/1 | 1/1 | 55.0% | 0 |
| corpus/CDA-Examples/input/examples/immunizations-influenza-vaccination-with-ndc.xml | 1/1 | 1/1 | 94.7% | 0 |
| corpus/CDA-Examples/input/examples/immunizations-influenza-vaccination.xml | 1/1 | 1/1 | 94.4% | 0 |
| corpus/CDA-Examples/input/examples/immunizations-no-section-information-immunizations.xml | 1/1 | 0/0 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/immunizations-unknown-immunization-status.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/interventions-intervention-counseling.xml | 0/1 | 0/1 | — | 0 |
| corpus/CDA-Examples/input/examples/medical-equipment-implant-udi-organizer.xml | 1/1 | 1/1 | 62.5% | 0 |
| corpus/CDA-Examples/input/examples/medical-equipment-implant-udi-unknown.xml | 1/1 | 1/1 | 56.0% | 0 |
| corpus/CDA-Examples/input/examples/medical-equipment-implant-without-procedure.xml | 1/1 | 3/3 | 88.2% | 0 |
| corpus/CDA-Examples/input/examples/medical-equipment-multiple-implants.xml | 1/1 | 2/2 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/medical-equipment-no-implanted-devices.xml | 1/1 | 1/1 | 50.0% | 0 |
| corpus/CDA-Examples/input/examples/medical-equipment-non-medicinal-supply-cane-and-eyeglasses.xml | 1/1 | 2/2 | 50.0% | 0 |
| corpus/CDA-Examples/input/examples/medications-drug-mixture.xml | 1/1 | 1/1 | 100.0% | 4 |
| corpus/CDA-Examples/input/examples/medications-free-text-medication-sig.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/medications-med-at-bedtime.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/medications-med-every-4-6-hours.xml | 1/1 | 1/1 | 90.0% | 0 |
| corpus/CDA-Examples/input/examples/medications-med-oral-qid-with-prn.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/medications-med-oral-with-indications-and-instructions.xml | 1/1 | 3/3 | 90.0% | 0 |
| corpus/CDA-Examples/input/examples/medications-med-relative-dose-iv-drug.xml | 1/1 | 1/1 | 83.3% | 0 |
| corpus/CDA-Examples/input/examples/medications-medication-refused.xml | 1/1 | 1/1 | 81.0% | 0 |
| corpus/CDA-Examples/input/examples/medications-no-medications.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/medications-single-administration-of-medication.xml | 1/1 | 1/1 | 88.2% | 0 |
| corpus/CDA-Examples/input/examples/mental-status-memory-impairment.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/mental-status-no-cognitive-impairment.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/mental-status-patient-health-questionnaire.xml | 1/1 | 1/1 | 80.6% | 0 |
| corpus/CDA-Examples/input/examples/notes-discharge-note-in-hospital-course.xml | 1/1 | 1/1 | 71.4% | 1 |
| corpus/CDA-Examples/input/examples/notes-note-directly-attached-to-a-procedure.xml | 1/1 | 1/1 | 45.5% | 0 |
| corpus/CDA-Examples/input/examples/notes-rtf-note.xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/CDA-Examples/input/examples/notes-single-consultation-note.xml | 1/1 | 1/1 | 100.0% | 1 |
| corpus/CDA-Examples/input/examples/plan-of-treatment-care-plan-goals-and-instructions.xml | 1/1 | 2/2 | 38.5% | 0 |
| corpus/CDA-Examples/input/examples/plan-of-treatment-no-planned-tests.xml | 1/1 | 2/2 | 71.4% | 0 |
| corpus/CDA-Examples/input/examples/plan-of-treatment-planned-ekg.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/plan-of-treatment-planned-encounter-referral.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/problems-active-problem.xml | 1/1 | 1/1 | 84.2% | 0 |
| corpus/CDA-Examples/input/examples/problems-complete-or-resolved-problem.xml | 1/1 | 1/1 | 87.0% | 0 |
| corpus/CDA-Examples/input/examples/problems-no-known-problems.xml | 1/1 | 1/1 | 83.3% | 0 |
| corpus/CDA-Examples/input/examples/problems-patient-does-not-have-diabetes.xml | 1/1 | 1/1 | 78.9% | 0 |
| corpus/CDA-Examples/input/examples/problems-pregnancy-with-week-gestation.xml | 1/1 | 1/1 | 81.8% | 0 |
| corpus/CDA-Examples/input/examples/problems-problem-targetsitecode-qualifier.xml | 1/1 | 1/1 | 54.5% | 0 |
| corpus/CDA-Examples/input/examples/problems-problem-value-translation-qualifier-examples.xml | 1/1 | 1/1 | 72.0% | 0 |
| corpus/CDA-Examples/input/examples/problems-sdoh-problem.xml | 1/1 | 1/1 | 90.9% | 0 |
| corpus/CDA-Examples/input/examples/procedures-procedure-alternatives.xml | 1/1 | 1/1 | 56.2% | 0 |
| corpus/CDA-Examples/input/examples/procedures-procedure-refused.xml | 1/1 | 1/1 | 58.3% | 0 |
| corpus/CDA-Examples/input/examples/procedures-procedure-unsuccessful.xml | 1/1 | 1/1 | 36.4% | 0 |
| corpus/CDA-Examples/input/examples/procedures-section-act-entry.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/procedures-section-observation-entry.xml | 1/1 | 1/1 | 87.5% | 0 |
| corpus/CDA-Examples/input/examples/procedures-section-procedure-entry-colonoscopy.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/procedures-section-procedure-entry.xml | 1/1 | 1/1 | 80.0% | 0 |
| corpus/CDA-Examples/input/examples/quality-care-compliance-in-c-cda.xml | 0/1 | 0/4 | — | 0 |
| corpus/CDA-Examples/input/examples/referrals-close-referral-with-a-document.xml | 5/5 | 0/0 | 96.8% | 1 |
| corpus/CDA-Examples/input/examples/referrals-referral-request-and-close-referral-with-a-note.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/results-covid-positive.xml | 1/1 | 1/1 | 58.3% | 0 |
| corpus/CDA-Examples/input/examples/results-of-basic-metabolic-panel-and-troponin.xml | 1/1 | 3/3 | 86.4% | 0 |
| corpus/CDA-Examples/input/examples/results-of-co2-test.xml | 1/1 | 1/1 | 90.9% | 0 |
| corpus/CDA-Examples/input/examples/results-panel-with-coded-values-of-negative-positive.xml | 1/1 | 1/1 | 52.4% | 0 |
| corpus/CDA-Examples/input/examples/results-panel-with-pending-component.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/results-radiology-with-image-narrative.xml | 1/1 | 1/1 | 94.1% | 0 |
| corpus/CDA-Examples/input/examples/results-unit-non-ucum.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/results-with-an-unstructured-string-as-value-urine-color.xml | 1/1 | 1/1 | 77.8% | 0 |
| corpus/CDA-Examples/input/examples/results-with-greater-than-a-specified-value.xml | 1/1 | 1/1 | 62.5% | 0 |
| corpus/CDA-Examples/input/examples/results-with-lab-location.xml | 1/1 | 1/1 | 68.8% | 0 |
| corpus/CDA-Examples/input/examples/results-with-less-than-specific-value.xml | 1/1 | 1/1 | 78.6% | 0 |
| corpus/CDA-Examples/input/examples/results-with-multiple-reference-ranges.xml | 1/1 | 1/1 | 60.0% | 0 |
| corpus/CDA-Examples/input/examples/results-with-non-numeric-physical-quantity-and-unit.xml | 1/1 | 1/1 | 57.1% | 0 |
| corpus/CDA-Examples/input/examples/results-with-translation-unit.xml | 1/1 | 1/1 | 62.5% | 0 |
| corpus/CDA-Examples/input/examples/social-history-birth-sex.xml | — | — | — | error: skipped: neither a document nor a section |
| corpus/CDA-Examples/input/examples/social-history-current-smoking-status.xml | 1/1 | 2/2 | 84.2% | 0 |
| corpus/CDA-Examples/input/examples/social-history-electronic-cigarette.xml | 1/1 | 1/1 | 83.3% | 1 |
| corpus/CDA-Examples/input/examples/social-history-former-smoking-status.xml | 1/1 | 2/2 | 80.0% | 0 |
| corpus/CDA-Examples/input/examples/social-history-never-smoking-status.xml | 1/1 | 1/1 | 80.0% | 0 |
| corpus/CDA-Examples/input/examples/social-history-not-pregnant.xml | 1/1 | 1/1 | 75.0% | 0 |
| corpus/CDA-Examples/input/examples/social-history-sexual-orientation-gender-identity.xml | 1/1 | 2/2 | 57.9% | 0 |
| corpus/CDA-Examples/input/examples/social-history-unknown-if-pregnant.xml | 1/1 | 1/1 | 75.0% | 0 |
| corpus/CDA-Examples/input/examples/social-history-unknown-smoking-status.xml | 1/1 | 1/1 | 80.0% | 0 |
| corpus/CDA-Examples/input/examples/social-history-we-care-assessment.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/unstructured-cda-reference-pdf.xml | 0/0 | 0/0 | 95.2% | 0 |
| corpus/CDA-Examples/input/examples/unstructured-cda-with-embedded-pdf-1.xml | 0/0 | 0/0 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/unstructured-cda-with-embedded-pdf-2.xml | 0/0 | 0/0 | 84.3% | 0 |
| corpus/CDA-Examples/input/examples/unstructured-cda-with-embedded-text-plain.xml | 0/0 | 0/0 | 94.9% | 0 |
| corpus/CDA-Examples/input/examples/vital-signs-growth-charts-examples.xml | 1/1 | 2/2 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/vital-signs-heart-rate-rhythm.xml | 1/1 | 1/1 | 66.7% | 0 |
| corpus/CDA-Examples/input/examples/vital-signs-panel-of-vital-signs-in-metric-units.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/vital-signs-panel-of-vital-signs-in-mixed-metric-imperial-units.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/CDA-Examples/input/examples/vital-signs-panel-of-vital-signs-oxygen-concentration-included.xml | 1/1 | 1/1 | 100.0% | 0 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_ccd_r11_sample1_v9.xml | 12/12 | 15/15 | 78.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_ccd_r21_sample1_v13.xml | 17/17 | 32/33 | 83.3% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_ccd_r21_sample2_v11.xml | 16/16 | 11/11 | 78.9% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_r11_sample1_v8.xml | 12/12 | 15/15 | 78.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_rn_r21_sample1_v13.xml | 17/17 | 32/33 | 83.3% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_rn_r21_sample2_v11.xml | 17/17 | 11/11 | 78.9% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ccd_r11_sample1_v9.xml | 15/15 | 19/19 | 69.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ccd_r21_sample1_v12.xml | 19/19 | 43/44 | 85.0% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ccd_r21_sample2_v12.xml | 20/20 | 15/15 | 84.1% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ds_r21_sample1_v12.xml | 21/21 | 43/44 | 84.9% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ds_r21_sample2_v12.xml | 22/22 | 15/15 | 83.8% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_r11_sample1_v8.xml | 15/15 | 19/19 | 69.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_rn_r21_sample1_v12.xml | 20/20 | 43/44 | 85.0% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_rn_r21_sample2_v12.xml | 21/21 | 15/15 | 84.1% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r11_sample1_recon_v10.xml | 15/15 | 21/21 | 80.4% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r11_sample1_v10.xml | 15/15 | 21/21 | 82.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r21_sample1_ccd_recon_v11.xml | 17/17 | 9/9 | 91.1% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r21_sample1_ccd_v11.xml | 17/17 | 6/6 | 94.2% | 3 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r21_sample1_rn_recon_v11.xml | 17/17 | 9/9 | 91.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r21_sample1_rn_v11.xml | 17/17 | 6/6 | 94.2% | 3 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r11_sample1_recon_v10.xml | 15/15 | 21/21 | 80.4% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r11_sample1_v10.xml | 15/15 | 21/21 | 82.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_ccd_recon_v11.xml | 17/17 | 9/9 | 91.1% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_ccd_v11.xml | — | — | — | error: to FHIR: 67:5: not well-formed XML: expected '>' not '<' at 67:5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_ds_recon_v11.xml | 22/22 | 11/11 | 92.0% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_ds_v11.xml | 22/22 | 7/7 | 93.7% | 2 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_rn_recon_v11.xml | 17/17 | 9/9 | 91.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_rn_v11.xml | 17/17 | 6/6 | 94.2% | 3 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Amb/170.315_b5_toc_amb_ccd_r11_sample1_v8.xml | 12/12 | 15/15 | 78.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Amb/170.315_b5_toc_amb_ccd_r21_sample1_v12.xml | 17/17 | 32/33 | 83.3% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Amb/170.315_b5_toc_amb_ccd_r21_sample2_v11.xml | 16/16 | 11/11 | 78.9% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Amb/170.315_b5_toc_amb_r11_sample1_v8.xml | 12/12 | 15/15 | 78.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Amb/170.315_b5_toc_amb_rn_r21_sample1_v12.xml | 17/17 | 32/33 | 83.3% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Amb/170.315_b5_toc_amb_rn_r21_sample2_v11.xml | 17/17 | 11/11 | 78.9% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Inp/170.315_b5_toc_inp_ccd_r11_sample1_v9.xml | 15/15 | 19/19 | 69.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Inp/170.315_b5_toc_inp_ccd_r21_sample1_v12.xml | 19/19 | 43/44 | 85.0% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Inp/170.315_b5_toc_inp_ccd_r21_sample2_v12.xml | 20/20 | 15/15 | 84.1% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Inp/170.315_b5_toc_inp_ds_r21_sample1_v12.xml | 21/21 | 43/44 | 84.9% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Inp/170.315_b5_toc_inp_ds_r21_sample2_v12.xml | 22/22 | 15/15 | 83.8% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Inp/170.315_b5_toc_inp_r11_sample1_v8.xml | 15/15 | 19/19 | 69.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Inp/170.315_b5_toc_inp_rn_r21_sample1_v12.xml | 20/20 | 43/44 | 85.0% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b5_CCDS_Inp/170.315_b5_toc_inp_rn_r21_sample2_v12.xml | 21/21 | 15/15 | 84.1% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b8_DS4P_Amb/170.315_b8_ds4p_amb_sample1_v8.xml | 16/17 | 30/32 | 85.5% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b8_DS4P_Amb/170.315_b8_ds4p_amb_sample2_v4.xml | 16/16 | 27/28 | 85.7% | 3 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b8_DS4P_Inp/170.315_b8_ds4p_inp_sample1_v7.xml | 19/20 | 43/45 | 85.0% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b8_DS4P_Inp/170.315_b8_ds4p_inp_sample2_v4.xml | 19/19 | 31/32 | 83.9% | 4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b9_CP_Amb/170.315_b9_cp_amb_sample1_v6.xml | 2/4 | 2/6 | 66.0% | 2 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/170.315_b9_CP_Inp/170.315_b9_cp_inp_sample1_v6.xml | 2/4 | 2/6 | 66.0% | 2 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_BadXml_r11_v2.xml | — | — | — | error: to FHIR: 59:4: not well-formed XML: invalid name token at 59:4 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_AllergyConcernElements_r11_v3.xml | 15/15 | 24/25 | 64.8% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_AllergyIntoleranceObservationElements_r11_v3.xml | 15/15 | 28/28 | 63.6% | 15 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_AllergySectionCode_r11_v2.xml | 15/15 | 24/24 | 61.3% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_DocCode_r11_v2.xml | 15/15 | 24/24 | 66.8% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_DocEffectiveTime_r11_v2.xml | 15/15 | 24/24 | 66.8% | 9 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_DocTypeId_r11_v2.xml | 15/15 | 24/24 | 66.8% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_EthnicityCodeSystem_r11_v2.xml | 15/15 | 24/24 | 66.6% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_EthnicityCode_r11_v2.xml | 15/15 | 24/24 | 66.8% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_GenderCodeSystem_r11_v2.xml | 15/15 | 24/24 | 66.6% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_GenderCode_r11_v2.xml | 15/15 | 24/24 | 66.6% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_MedicationActivityElements_r11_v3.xml | 15/15 | 29/30 | 49.8% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_MedicationInformationElements_r11_v3.xml | 15/15 | 27/27 | 57.5% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_MedicationSectionCode_r11_v2.xml | 15/15 | 24/24 | 62.6% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_MedicationSectionTemplateId_r11_v3.xml | 14/15 | 24/24 | 73.5% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_PatientFirstName_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_PatientLastName_r1_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_PatientMiddleName_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_PatientPreviousName_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_PatientSuffix_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_PreferredLanguageCodeSystem_r11_v2.xml | 15/15 | 24/24 | 66.7% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_ProblemConcernElements_r11_v3.xml | 15/15 | 25/26 | 63.7% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_ProblemObservationElements_r11_v3.xml | 15/15 | 31/31 | 60.4% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_ProblemSectionCode_r11_v2.xml | 15/15 | 24/24 | 64.0% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_ProblemSectionTemplateId_r11_v3.xml | 14/15 | 24/24 | 69.7% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Bad_RaceCodeSystem_r11_v2.xml | 15/15 | 24/24 | 66.6% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_CCDS_Sample1_r21_v4.xml | 19/19 | 28/28 | 27.0% | 3 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_CCDS_Sample2_r21_v4.xml | 10/19 | 28/28 | 93.3% | 3 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_CCDS_Sample3_r21_v4.xml | 19/19 | 6/16 | 50.2% | 1 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_CCDS_Sample4_r21_v4.xml | 19/19 | 28/28 | 84.9% | 5 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_CCDS_Sample5_r21_v4.xml | 19/19 | 28/28 | 85.4% | 3 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Missing_PatientLastName_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CCDS/NT_Missing_PatientName_r11_v2.xml | 15/15 | 24/24 | 66.7% | 8 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CarePlan/NT_CP_Sample1_r21_v5.xml | 18/18 | 25/25 | 85.7% | 3 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CarePlan/NT_CP_Sample2_r21_v4.xml | 2/4 | 2/7 | 48.9% | 2 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CarePlan/NT_CP_Sample3_r21_v4.xml | 0/4 | 2/7 | 97.2% | 2 |
| corpus/2015-certification-ccda-testdata/Receiver SUT Test Data/NegativeTesting_CarePlan/NT_CP_Sample4_r21_v4.xml | 2/4 | 0/7 | 50.4% | 1 |
| corpus/2015-certification-ccda-testdata/Sender SUT Test Data/Gold_Samples_For_Practice/170.315_b1_toc_gold_sample1_v6.xml | 19/19 | 31/32 | 84.9% | 4 |
| corpus/2015-certification-ccda-testdata/Sender SUT Test Data/Gold_Samples_For_Practice/170.315_b1_toc_gold_sample2_v4.xml | 20/20 | 15/15 | 83.2% | 0 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_ccd_r11_sample1_v9.xml | 12/12 | 15/15 | 78.5% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_ccd_r21_sample1_v13.xml | 17/17 | 32/33 | 83.3% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_ccd_r21_sample2_v11.xml | 16/16 | 11/11 | 78.9% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_r11_sample1_v8.xml | 12/12 | 15/15 | 78.5% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_rn_r21_sample1_v13.xml | 17/17 | 32/33 | 83.3% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Amb/170.315_b1_toc_amb_rn_r21_sample2_v11.xml | 17/17 | 11/11 | 78.9% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ccd_r11_sample1_v9.xml | 15/15 | 19/19 | 69.5% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ccd_r21_sample1_v12.xml | 19/19 | 43/44 | 85.0% | 5 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ccd_r21_sample2_v12.xml | 20/20 | 15/15 | 84.1% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ds_r21_sample1_v12.xml | 21/21 | 43/44 | 84.9% | 5 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_ds_r21_sample2_v12.xml | 22/22 | 15/15 | 83.8% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_r11_sample1_v8.xml | 15/15 | 19/19 | 69.5% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_rn_r21_sample1_v12.xml | 20/20 | 43/44 | 85.0% | 5 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b1_ToC_Inp/170.315_b1_toc_inp_rn_r21_sample2_v12.xml | 21/21 | 15/15 | 84.1% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r11_sample1_recon_v10.xml | 15/15 | 21/21 | 80.4% | 5 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r11_sample1_v10.xml | 15/15 | 21/21 | 82.5% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r21_sample1_ccd_recon_v12.xml | 17/17 | 10/10 | 89.8% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r21_sample1_ccd_v12.xml | 17/17 | 6/6 | 94.2% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r21_sample1_rn_recon_v12.xml | 17/17 | 10/10 | 90.2% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Amb/170.315_b2_ciri__r21_sample1_rn_v12.xml | 17/17 | 6/6 | 94.2% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r11_sample1_recon_v10.xml | 15/15 | 21/21 | 80.4% | 5 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r11_sample1_v10.xml | 15/15 | 21/21 | 82.5% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_ccd_recon_v12.xml | 17/17 | 10/10 | 89.9% | 3 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_ccd_v12.xml | 17/17 | 6/6 | 94.2% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_ds_recon_v12.xml | 22/22 | 12/12 | 90.9% | 3 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_ds_v12.xml | 22/22 | 7/7 | 93.8% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_rn_recon_v12.xml | 17/17 | 10/10 | 90.3% | 3 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b2_CIRI_Inp/170.315_b2_ciri__r21_sample1_rn_v12.xml | 17/17 | 6/6 | 94.2% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b8_DS4P_Amb/170.315_b8_ds4p_amb_sample1_v8.xml | 16/17 | 30/32 | 85.5% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b8_DS4P_Amb/170.315_b8_ds4p_amb_sample2_v4.xml | 16/16 | 27/28 | 85.7% | 3 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b8_DS4P_Inp/170.315_b8_ds4p_inp_sample1_v7.xml | 19/20 | 43/45 | 85.0% | 5 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b8_DS4P_Inp/170.315_b8_ds4p_inp_sample2_v4.xml | 19/19 | 31/32 | 83.9% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b9_CP_Amb/170.315_b9_cp_amb_sample1_v6.xml | 2/4 | 2/6 | 66.0% | 2 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/170.315_b9_CP_Inp/170.315_b9_cp_inp_sample1_v6.xml | 2/4 | 2/6 | 66.0% | 2 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_CarePlan/NT_CP_Sample1_r21_v5.xml | 18/18 | 25/25 | 85.7% | 3 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_CarePlan/NT_CP_Sample2_r21_v4.xml | 2/4 | 2/7 | 48.9% | 2 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_CarePlan/NT_CP_Sample3_r21_v4.xml | 0/4 | 2/7 | 97.2% | 2 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_CarePlan/NT_CP_Sample4_r21_v4.xml | 2/4 | 0/7 | 50.4% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_BadXml_r11_v2.xml | — | — | — | error: to FHIR: 59:4: not well-formed XML: invalid name token at 59:4 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_AllergyConcernElements_r11_v3.xml | 15/15 | 24/25 | 64.8% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_AllergyIntoleranceObservationElements_r11_v3.xml | 15/15 | 28/28 | 63.6% | 15 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_AllergySectionCode_r11_v2.xml | 15/15 | 24/24 | 61.3% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_DocCode_r11_v2.xml | 15/15 | 24/24 | 66.8% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_DocEffectiveTime_r11_v2.xml | 15/15 | 24/24 | 66.8% | 9 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_DocTypeId_r11_v2.xml | 15/15 | 24/24 | 66.8% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_EthnicityCodeSystem_r11_v2.xml | 15/15 | 24/24 | 66.6% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_EthnicityCode_r11_v2.xml | 15/15 | 24/24 | 66.8% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_GenderCodeSystem_r11_v2.xml | 15/15 | 24/24 | 66.6% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_GenderCode_r11_v2.xml | 15/15 | 24/24 | 66.6% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_MedicationActivityElements_r11_v3.xml | 15/15 | 29/30 | 49.8% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_MedicationInformationElements_r11_v3.xml | 15/15 | 27/27 | 57.5% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_MedicationSectionCode_r11_v2.xml | 15/15 | 24/24 | 62.6% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_MedicationSectionTemplateId_r11_v3.xml | 14/15 | 24/24 | 73.5% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_PatientFirstName_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_PatientLastName_r1_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_PatientMiddleName_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_PatientPreviousName_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_PatientSuffix_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_PreferredLanguageCodeSystem_r11_v2.xml | 15/15 | 24/24 | 66.7% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_ProblemConcernElements_r11_v3.xml | 15/15 | 25/26 | 63.7% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_ProblemObservationElements_r11_v3.xml | 15/15 | 31/31 | 60.4% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_ProblemSectionCode_r11_v2.xml | 15/15 | 24/24 | 64.0% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_ProblemSectionTemplateId_r11_v3.xml | 14/15 | 24/24 | 69.7% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Bad_RaceCodeSystem_r11_v2.xml | 15/15 | 24/24 | 66.6% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Missing_PatientLastName_r11_v2.xml | 15/15 | 24/24 | 67.0% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_Missing_PatientName_r11_v2.xml | 15/15 | 24/24 | 66.7% | 8 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_USCDI_Sample1_r21_v4.xml | 19/19 | 28/28 | 27.0% | 3 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_USCDI_Sample2_r21_v4.xml | 10/19 | 28/28 | 93.3% | 3 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_USCDI_Sample3_r21_v4.xml | 19/19 | 6/16 | 50.2% | 1 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_USCDI_Sample4_r21_v4.xml | 19/19 | 28/28 | 84.9% | 5 |
| corpus/2015-edition-cures-update-data/Cures Update Receiver SUT Test Data/NegativeTesting_USCDI/NT_USCDI_Sample5_r21_v4.xml | 19/19 | 28/28 | 85.4% | 3 |
| corpus/2015-edition-cures-update-data/Cures Update Sender SUT Test Data/Gold_Samples_For_Practice/170.315_b1_toc_cures_sample3.xml | 23/24 | 25/26 | 85.5% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Sender SUT Test Data/Gold_Samples_For_Practice/170.315_b1_toc_gold_sample1_v6.xml | 20/20 | 34/35 | 88.5% | 4 |
| corpus/2015-edition-cures-update-data/Cures Update Sender SUT Test Data/Gold_Samples_For_Practice/170.315_b1_toc_gold_sample2_v4.xml | 21/21 | 16/16 | 86.1% | 0 |
| corpus/2015-edition-cures-update-data/Security-tags-Summary-of-Care-receive-sample1.xml | 16/17 | 30/32 | 85.5% | 4 |

## Agreement with the C-CDA on FHIR consensus outputs

Values of the recommended resources (`mappings/examples/CDAtoFHIRExamples/Rec_Output`) our conversion of the shared input has: **78 of 89 (87.6%)**. The `_draft` outputs are drafts: where they differ from the mapping tables (SNOMED CT as `http://www.snomed.org/`, a note's category as a LOINC part, `34109-9` kept in its type) the tables are followed.

| Recommended | Type | Values | Ours | Missing |
|---|---|---|---|---|
| Rec_allergy.jsonc | AllergyIntolerance | 10 | 10 |  |
| Rec_diagnosticsReport_draft.json | DiagnosticReport | 6 | 5 | `identifier urn:oid:1.3.6.1.4.1.22812.3.2009316.3.3.3|3` |
| Rec_encounterDx_draft.jsonc | Condition | 4 | 3 | `coding http://www.snomed.org/|64109004` |
| Rec_encounter_draft.json | Encounter | 5 | 5 |  |
| Rec_immunization.jsonc | Immunization | 4 | 4 |  |
| Rec_medicationRequest.jsonc | MedicationRequest | 6 | 3 | `asNeededBoolean false`, `coding http://www.nlm.nih.gov/research/umls/rxnorm|582498`, `intent order` |
| Rec_note_draft.json | DocumentReference | 5 | 3 | `coding http://loinc.org|34109-9`, `coding http://loinc.org|LP72311-1` |
| Rec_patient.jsonc | Patient | 7 | 7 |  |
| Rec_problem.jsonc | Condition | 9 | 9 |  |
| Rec_procedure.jsonc | Procedure | 8 | 8 |  |
| Rec_result_obs_draft.json | Observation | 11 | 10 | `identifier urn:oid:1.3.6.1.4.1.22812.3.2009316.3.3.3|3` |
| Rec_social history_draft.json | Observation | 7 | 5 | `coding http://www.snomed.org/|449868002`, `identifier http://hl7.org/fhir/sid/us-npi|99999999` |
| Rec_vital sign_draft.json | Observation | 7 | 6 | `date 2014-05-21` |
