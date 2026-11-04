# Sources

Everything in `data/` comes from HL7 packages licensed CC0-1.0, fetched by `scripts/vendor.py`.

| Package | Version | Used for |
|---|---|---|
| hl7.cda.uv.core | 2.0.3-sd | `models.json`: the CDA logical model |
| hl7.cda.us.ccda | 5.0.0 | `models.json`: C-CDA templates |
| hl7.fhir.us.ccda | 2.0.0 | `conceptmaps/`: 42 ConceptMaps of C-CDA on FHIR, unchanged |
| hl7.terminology.r4 | 7.5.0 | `conceptmaps/ConceptMap-oid-to-uri.json`, `ConceptMap-uri-to-oid.json`: 1287 OID ↔ URI pairs |
