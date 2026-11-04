#!/usr/bin/env bash
# Measures the corpus (scripts/fetch-corpus.sh) and writes reports/corpus.md and
# reports/corpus.json: coverage, round trip, US Core and C-CDA validity, consensus agreement.
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p reports
cargo run --quiet -- coverage \
  corpus/C-CDA-Examples corpus/CDA-Examples \
  corpus/2015-certification-ccda-testdata corpus/2015-edition-cures-update-data \
  --us-core corpus/packages/hl7.fhir.us.core \
  --ccda corpus/packages/hl7.cda.us.ccda \
  --consensus corpus/ccda-on-fhir/mappings/examples/CDAtoFHIRExamples \
  -o reports/corpus.md --json reports/corpus.json "$@"
