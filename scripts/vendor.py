#!/usr/bin/env python3
"""Vendors the definitions and terminology maps the converter needs into data/.

Downloads FHIR packages from the package registry (packages.fhir.org) and writes:

- data/models.json: the StructureDefinitions of the CDA logical model (hl7.cda.uv.core) and
  the C-CDA templates (hl7.cda.us.ccda), trimmed to what the XML layer, the type checker and
  the engine read (snapshots without slices, XML extensions, template identifiers).
- data/conceptmaps/*.json: the ConceptMaps of C-CDA on FHIR (hl7.fhir.us.ccda), as published,
  and two generated ConceptMaps between code system and identifier system OIDs and URIs, from
  the NamingSystems and CodeSystems of HL7 Terminology (hl7.terminology.r4).
- data/SOURCES.md: what came from where, with versions and licenses.

All sources are CC0-1.0. Run from the repository root: python3 scripts/vendor.py
"""

import io
import json
import os
import sys
import tarfile
import urllib.request

PACKAGES = {
    "hl7.cda.uv.core": "2.0.3-sd",
    "hl7.cda.us.ccda": "5.0.0",
    "hl7.fhir.us.ccda": "2.0.0",
    "hl7.terminology.r4": "7.5.0",
}
XML_EXTENSIONS = {"xml-name", "xml-namespace", "xml-choice-group", "elementdefinition-defaulttype",
                  "elementdefinition-date-format"}
SD_EXTENSIONS = {"xml-name", "xml-namespace", "type-profile-style"}
CANONICAL = "http://octofhir.com/ccda-fhir"


def fetch(name, version):
    """The JSON resources of a package, by file name."""
    url = f"https://packages.fhir.org/{name}/{version}"
    with urllib.request.urlopen(url) as response:
        data = response.read()
    files = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        for member in archive.getmembers():
            if not member.isfile() or not member.name.startswith("package/"):
                continue
            path = member.name[len("package/"):]
            if "/" in path or not path.endswith(".json"):
                continue
            files[path] = json.load(archive.extractfile(member))
    return files


def tail(url):
    return url.rsplit("/", 1)[-1]


def trim_element(element):
    out = {k: element[k] for k in ("id", "path", "min", "max", "representation", "contentReference")
           if k in element}
    if "type" in element:
        out["type"] = [{"code": t["code"]} for t in element["type"]]
    extensions = [e for e in element.get("extension", []) if tail(e["url"]) in XML_EXTENSIONS]
    if extensions:
        out["extension"] = extensions
    return out


def trim_structure(sd):
    out = {k: sd[k] for k in ("resourceType", "url", "version", "name", "status", "kind", "abstract", "type",
                              "baseDefinition", "derivation", "identifier") if k in sd}
    extensions = [e for e in sd.get("extension", []) if tail(e["url"]) in SD_EXTENSIONS]
    if extensions:
        out["extension"] = extensions
    out["snapshot"] = {"element": [trim_element(e) for e in sd["snapshot"]["element"]
                                   if ":" not in e.get("id", "")]}
    return out


def oid_uri_pairs(terminology):
    """OID → URI from HL7 Terminology: a NamingSystem's preferred URI first, else the URL of the
    CodeSystem that names the OID. An OID several sources give different URIs for is left out
    unless a NamingSystem settles it."""
    from_naming = {}
    from_codesystems = {}
    for name, resource in sorted(terminology.items()):
        kind = resource.get("resourceType")
        if kind == "NamingSystem" and resource.get("status") != "retired":
            ids = resource.get("uniqueId", [])
            oids = [u["value"] for u in ids if u.get("type") == "oid"]
            uris = [u for u in ids if u.get("type") == "uri"]
            preferred = [u["value"] for u in uris if u.get("preferred")] or [u["value"] for u in uris]
            for oid in oids:
                if preferred:
                    from_naming.setdefault(oid, set()).add(preferred[0])
        elif kind == "CodeSystem" and resource.get("url"):
            identifiers = resource.get("identifier", [])
            if isinstance(identifiers, dict):
                identifiers = [identifiers]
            for identifier in identifiers:
                value = identifier.get("value", "")
                if value.startswith("urn:oid:"):
                    from_codesystems.setdefault(value[8:], set()).add(resource["url"])
    pairs = {}
    for oid, uris in from_naming.items():
        if len(uris) == 1:
            pairs[oid] = next(iter(uris))
    for oid, uris in from_codesystems.items():
        if oid not in pairs and oid not in from_naming and len(uris) == 1:
            pairs[oid] = next(iter(uris))
    return dict(sorted(pairs.items()))


def concept_map(name, title, description, source, target, pairs):
    return {
        "resourceType": "ConceptMap",
        "id": name,
        "url": f"{CANONICAL}/ConceptMap/{name}",
        "name": "".join(part.capitalize() for part in name.split("-")),
        "title": title,
        "status": "active",
        "description": description,
        "group": [{
            "source": source,
            "target": target,
            "element": [{"code": a, "target": [{"code": b, "equivalence": "equivalent"}]}
                        for a, b in pairs],
            "unmapped": {"mode": "provided"},
        }],
    }


def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    data = os.path.join(root, "data")
    os.makedirs(os.path.join(data, "conceptmaps"), exist_ok=True)
    packages = {name: fetch(name, version) for name, version in PACKAGES.items()}

    structures = []
    for name in ("hl7.cda.uv.core", "hl7.cda.us.ccda"):
        for file, resource in sorted(packages[name].items()):
            if resource.get("resourceType") == "StructureDefinition" and "snapshot" in resource:
                structures.append(trim_structure(resource))
    with open(os.path.join(data, "models.json"), "w") as out:
        json.dump(structures, out, separators=(",", ":"), ensure_ascii=False)

    folder = os.path.join(data, "conceptmaps")
    for file in os.listdir(folder):
        os.remove(os.path.join(folder, file))
    published = 0
    for file, resource in sorted(packages["hl7.fhir.us.ccda"].items()):
        if resource.get("resourceType") == "ConceptMap":
            resource.pop("text", None)
            with open(os.path.join(folder, file), "w") as out:
                json.dump(resource, out, indent=2, ensure_ascii=False)
                out.write("\n")
            published += 1

    pairs = oid_uri_pairs(packages["hl7.terminology.r4"])
    to_uri = concept_map(
        "oid-to-uri", "OID to URI",
        "Code system and identifier system OIDs to the URIs FHIR uses, from HL7 Terminology "
        "NamingSystems and CodeSystems. An OID it does not know translates to itself.",
        "urn:ietf:rfc:3986", "urn:ietf:rfc:3986", list(pairs.items()))
    reverse = {}
    for oid, uri in pairs.items():
        reverse.setdefault(uri, oid)
    to_oid = concept_map(
        "uri-to-oid", "URI to OID",
        "The URIs FHIR uses for code systems and identifier systems to their OIDs, from HL7 "
        "Terminology. A URI it does not know translates to itself.",
        "urn:ietf:rfc:3986", "urn:ietf:rfc:3986", sorted(reverse.items()))
    for resource in (to_uri, to_oid):
        with open(os.path.join(folder, f"ConceptMap-{resource['id']}.json"), "w") as out:
            json.dump(resource, out, indent=2, ensure_ascii=False)
            out.write("\n")

    with open(os.path.join(data, "SOURCES.md"), "w") as out:
        out.write("# Sources\n\nEverything in `data/` comes from HL7 packages licensed CC0-1.0, "
                  "fetched by `scripts/vendor.py`.\n\n| Package | Version | Used for |\n|---|---|---|\n")
        out.write(f"| hl7.cda.uv.core | {PACKAGES['hl7.cda.uv.core']} | `models.json`: the CDA "
                  "logical model |\n")
        out.write(f"| hl7.cda.us.ccda | {PACKAGES['hl7.cda.us.ccda']} | `models.json`: C-CDA "
                  "templates |\n")
        out.write(f"| hl7.fhir.us.ccda | {PACKAGES['hl7.fhir.us.ccda']} | `conceptmaps/`: "
                  f"{published} ConceptMaps of C-CDA on FHIR, unchanged |\n")
        out.write(f"| hl7.terminology.r4 | {PACKAGES['hl7.terminology.r4']} | "
                  f"`conceptmaps/ConceptMap-oid-to-uri.json`, `ConceptMap-uri-to-oid.json`: "
                  f"{len(pairs)} OID ↔ URI pairs |\n")
    print(f"{len(structures)} structures, {published} ConceptMaps, {len(pairs)} OID ↔ URI pairs",
          file=sys.stderr)


if __name__ == "__main__":
    main()
