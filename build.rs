//! Embeds the maps, ConceptMaps and model definitions, so the converter needs no files at run
//! time.

use std::fmt::Write;
use std::path::{Path, PathBuf};

fn files(dir: &Path, extension: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            files(&path, extension, out);
        } else if path.extension().is_some_and(|e| e == extension) {
            out.push(path);
        }
    }
}

fn table(name: &str, root: &Path, paths: &[PathBuf], code: &mut String) {
    writeln!(code, "pub static {name}: &[(&str, &str)] = &[").unwrap();
    for path in paths {
        let relative = path
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        writeln!(
            code,
            "    ({relative:?}, include_str!({:?})),",
            path.display().to_string()
        )
        .unwrap();
    }
    writeln!(code, "];").unwrap();
}

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let maps_dir = root.join("maps");
    let concept_dir = root.join("data/conceptmaps");
    let (mut maps, mut concept_maps) = (Vec::new(), Vec::new());
    files(&maps_dir, "map", &mut maps);
    files(&concept_dir, "json", &mut concept_maps);
    let mut code = String::new();
    table("MAPS", &maps_dir, &maps, &mut code);
    table("CONCEPT_MAPS", &concept_dir, &concept_maps, &mut code);
    let models = root.join("data/models.json");
    writeln!(
        code,
        "pub static MODELS: &str = include_str!({:?});",
        models.display().to_string()
    )
    .unwrap();
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("embedded.rs");
    std::fs::write(out, code).unwrap();
    println!("cargo:rerun-if-changed=maps");
    println!("cargo:rerun-if-changed=data");
}
