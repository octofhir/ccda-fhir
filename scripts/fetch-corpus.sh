#!/usr/bin/env bash
# Fetches the corpus the metrics run on into ./corpus (not committed): CC0 example documents at
# pinned commits, and the packages the output is validated against.
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p corpus/packages
fetch() {
  local name=$1 url=$2 commit=$3
  if [ ! -d "corpus/$name/.git" ]; then
    git init -q "corpus/$name"
    git -C "corpus/$name" remote add origin "$url"
  fi
  if [ "$(git -C "corpus/$name" rev-parse -q --verify HEAD 2>/dev/null)" != "$commit" ]; then
    git -C "corpus/$name" fetch -q --depth 1 origin "$commit"
    git -C "corpus/$name" checkout -q FETCH_HEAD
  fi
}
fetch C-CDA-Examples https://github.com/HL7/C-CDA-Examples ad5007abd912a45bbd04ce96e871c70954c2b2c2
fetch CDA-Examples https://github.com/HL7/CDA-Examples 76619b21ccd7f42567f77fa6219bfab473f9dfed
fetch 2015-certification-ccda-testdata https://github.com/onc-healthit/2015-certification-ccda-testdata 4ac99c52ac99c998055c72c3120c52b10a29ebe7
fetch 2015-edition-cures-update-data https://github.com/onc-healthit/2015-edition-cures-update-data 651712be3a58fbeffa0f9abbf14863ec786912a6
fetch ccda-on-fhir https://github.com/HL7/ccda-on-fhir 2ecc324bb9225a76f321b2df01e6515dec9bcc3a
package() {
  local name=$1 version=$2
  if [ ! -d "corpus/packages/$name" ]; then
    mkdir -p "corpus/packages/$name"
    curl -sfL "https://packages.fhir.org/$name/$version" | tar xz -C "corpus/packages/$name"
  fi
}
package hl7.fhir.us.core 6.1.0
package hl7.cda.us.ccda 5.0.0
