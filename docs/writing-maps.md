# Writing the maps

How the conversion works and how to write or change a map, without guessing. The maps are plain
[FHIR Mapping Language](https://build.fhir.org/ig/HL7/mapping-language-ig/) (FML); nothing in this
repository is a private dialect. What is particular is the source side: CDA read into JSON by its
logical model, and C-CDA templates as profiles.

## 1. What runs, in order

`ccda-fhir to-fhir` (or `Converter::to_fhir`):

1. **Read.** `fml-xml` reads the XML into JSON of the CDA logical model (`hl7.cda.uv.core`), with
   the line and column of every element. What the model has no place for is reported, not dropped
   silently.
2. **Prepare** (`src/prepare.rs`). Narrative references (`<reference value="#id"/>`) get the text
   they point to, so a map reads `originalText.xmlText` and gets words, not `#id`. A Note
   Activity's text gets the referenced narrative as XHTML.
3. **Map.** `maps/cda2fhir/document.map` (group `ClinicalDocument`) runs, importing the others.
4. **Finish** (`src/prepare.rs`). Entries a map marked as left out are taken out and reported
   (see §5, "What cannot be carried over"); identical Practitioners, Organizations and the like are merged;
   Encounters with the same identifier are merged; a note's encounter named by its id becomes a
   reference to the Bundle entry; section narrative becomes XHTML.

`to-cda` is the mirror: CDA narrative from XHTML, `maps/fhir2cda/document.map` (group `Document`),
note paragraphs added to section narrative, then `fml-xml` writes XML in the model's element order.

Anything that is a mapping decision lives in the maps. The Rust steps only do what FML cannot say
well: whole-Bundle merges, reference rewriting, markup conversion.

## 2. The JSON a map sees

`fml-xml` makes CDA look like FHIR JSON of the logical model. The rules, with the XML they come
from:

| XML | JSON | In a map |
|---|---|---|
| `<code code="1" codeSystem="2.16…"/>` | `"code": {"code": "1", "codeSystem": "2.16…"}` | `src.code as c` then `c.code`, `c.codeSystem` |
| attributes | properties of the element | `id.root`, `id.extension` |
| `<title>Allergies</title>` | `"title": {"xmlText": "Allergies"}` | `title.xmlText as t` |
| `<value xsi:type="PQ" value="1" unit="mg"/>` | `"value": {"@type": "PQ", "value": 1, "unit": "mg"}` | `src.value : PQ as v` |
| `<effectiveTime value="20240102103000-0500"/>` | `"effectiveTime": {"value": "2024-01-02T10:30:00-05:00"}` | the FHIR form, at the precision written |
| `<name><given>Ada</given><family>X</family></name>` | `"name": [{"item": [{"given": {"xmlText": "Ada"}}, {"family": {"xmlText": "X"}}]}]` | `name.item as i` then `i.given`, `i.family` |
| `<section><text>…markup…</text>` | `"text": "<text>…</text>"` (a string) | copied as is, converted after the map |
| `sdtc:dischargeDispositionCode` | `"sdtcDischargeDispositionCode"` | the model's element name |

- **`"@type"` is `xsi:type`.** It is there when the document states it, when the element can have
  several types, or when the type differs from the declared one. A type filter (`: PQ`) reads it.
- **Mixed content** (names, addresses) is a list of one-member objects in `item`, in document
  order; plain text between parts is `{"xmlText": …}`.
- **Timestamps** are typed `DateTime` in FHIRPath. String functions need `toString()` first:
  `value.toString().matches('T.*[+-]')`, not `value.length()`.

To see the JSON of a real document, run `fml xml read document.xml --package hl7.cda.uv.core@2.0.3-sd`
(fml-rs CLI), or open the server's FML console, the **CDA** tab, and step through a trace.

## 3. Templates are profiles

The C-CDA templates are StructureDefinitions (`hl7.cda.us.ccda`) whose identifier is the template
id (`urn:hl7ii:2.16.840.1.113883.10.20.22.4.49:2015-08-01`). A map names one with `uses … as source`
and then:

- **Routes by it.** `entry.encounter : EncounterActivity as e` matches an `encounter` carrying that
  template id, in any version (the converter runs the engine with `with_template_versions(false)`).
- **Asks about it.** `hasTemplateIdOf('urn:oid:2.16.840.1.113883.10.20.22.2.22')` in a condition,
  for templates no map uses as a type: the "entries optional" sections, exclusions.
- **Counts it as known.** Every template a map `uses`, and every OID in `hasTemplateIdOf('urn:oid:…')`,
  is "recognised" in the coverage metric. Naming a template you do not convert would make the
  metric lie; naming one the general rules do convert (narrative-only sections, social history
  observations) makes it honest.

Writing CDA, the template is just values: `cda.templateId as t, t.root = '…', t.extension = '…'`,
with the root and extension the C-CDA 5.0.0 template states (look in the package, not in older
examples).

## 4. Where things go

| File | Holds |
|---|---|
| `maps/*/datatypes.map` | II ⇄ Identifier, CD ⇄ CodeableConcept, TS ⇄ dateTime, PQ ⇄ Quantity, AD, PN, TEL |
| `maps/*/participants.map` | authors, performers, organizations, locations, comments |
| `maps/*/document.map` | the header, the section routing by template (CDA) or LOINC code (FHIR) |
| `maps/*/<domain>.map` | one domain each way: `allergies`, `problems`, `medications`, `immunizations`, `results` (and vital signs), `procedures`, `encounters`, `social`, `notes`, `plans` (goals, plan of treatment), `status` (functional, mental), `family`, `devices` |
| `data/conceptmaps/` | C-CDA on FHIR ConceptMaps (used through `translate`) and OID ⇄ URI |

A new domain is a pair of files (`cda2fhir/x.map`, `fhir2cda/x.map`), an `imports` line and a
routing rule in each `document.map`, and a test in `tests/convert.rs`.

The source of truth for what goes where is the C-CDA on FHIR table of the domain
(`corpus/ccda-on-fhir/input/pagecontent/CF-*.md`, `FC-*.md`) and the template's StructureDefinition
for what CDA requires. Where the tables say nothing (FHIR → CDA for encounters, social history,
notes, results), the map is the inverse of the CDA → FHIR table, written to the 5.0.0 templates,
and its description says so.

## 5. FML and FHIRPath, the parts that bite

**A variable hides an element of the same name.** In a condition or `evaluate`, a bare name that is
a variable in scope is the variable, not the element of the focus (the mapping language lets a
local variable hide a constant of the same name; the engine and the checker resolve variables the
same way). This silently changes meaning:

```fml
// wrong: `value` in the condition is the variable, which always exists
src.value : PQ as value where (value.exists()) -> …
// right
src.value : PQ as pq where (pq.value.exists()) -> …
```

The same goes for target variables: do not call a target `code` and then test `code` in a condition
on a Coding; name it `cd`, `noteCode`, `fhirLocation`.

`ccda-fhir check` reports it: a bare name that is both a variable in scope and an element of the
focus is a warning (fml-check), so the slip is caught before it runs.

**Empty is not false.** `x.hasValue().not()` on an empty `x` is empty, so the rule does not fire.
Use `iif(…, false, true)` when you mean "not true", and `!=` only when both sides surely exist:

```fml
code where (iif(codeSystem = '2.16.840.1.113883.5.4', false, true)) -> …   // also when codeSystem is missing
```

**`first` is applied after `where`.** `src.translation first as t where (codeSystem = 'X')` is the
first translation in system X, not the first translation if it happens to be in X.

**Outer variables are in scope.** Inside `then { … }` a rule can read any source variable of the
enclosing rules (`%observation` in an expression, or the bare name as a source).

**One value for a single element.** Two rules that both write `effectiveTime` on the same target
write two; the writer reports it and the C-CDA check fails. Make their conditions exclusive.

**Data absent, not missing.** When US Core requires an element the document does not give,
write a data-absent-reason extension on it (`issued`, `class`, `type`, `Location.name`), as the
C-CDA on FHIR examples do, and count it in the report rather than inventing a value.

**What cannot be carried over** at all, because the profile would reject the resource, is
reported, not written: group `LeftOut` of `cda2fhir/participants.map` writes a marker entry with
the message and the resource it is about, and the Finish step takes it out and adds the message
to the report. An entry author with no organization is the case today: US Core Provenance needs
`onBehalfOf` for a Practitioner (provenance-1), so that author gets no Provenance.

**Translations that have no target** are left out with a warning (the engine runs with
`with_lenient_translations(true)`): a code nobody can map should not fail a whole document.

## 6. Working on a map

```sh
cargo run -- check                                   # type-check every map (CI does too)
cargo run -- --maps maps to-fhir doc.xml --deterministic -o out.json --report report.json
cargo run -- --maps maps to-fhir "Section Example.xml" --fragment   # a section or entry example
cargo run -- --maps maps to-cda out.json -o back.xml
```

- `--maps maps` runs the maps from the folder, so edits need no rebuild.
- `--deterministic` numbers ids from 1: the same input gives the same output, diffs are readable.
- `--fragment` wraps a section or entry example in a minimal document, as the corpus metrics do.
- The report (`--report`, or the OperationOutcome of `$ccda-to-fhir`) lists what was not carried
  over, with the CDA path and the line in the XML. Read it before reading the output.

To watch a map run, use the server's FML console: open the map, switch the input to **CDA**, load
the document, run with the trace on, and step; the XML element of each step is highlighted.

## 7. Measuring

`scripts/fetch-corpus.sh` once, then `scripts/metrics.sh` writes `reports/corpus.md`: sections and
entries recognised, the templates nothing maps, US Core validity, C-CDA conformance of what is
written back, both round trips, and agreement with the C-CDA on FHIR consensus outputs. Run it
before and after a change; a change that improves one domain and lowers another number shows up
there, not in the tests.
