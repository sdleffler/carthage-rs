extern crate lalrpop;
extern crate string_cache_codegen;

use std::{env, path::Path};

const ATOMS: &'static [&'static str] = &[
    "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString",
    "http://www.w3.org/2001/XMLSchema#string",
];

fn main() {
    lalrpop::process_root().unwrap();
    string_cache_codegen::AtomType::new("rdf::RdfAtom", "rdf_atom!")
        .atoms(ATOMS)
        .write_to_file(&Path::new(&env::var("OUT_DIR").unwrap()).join("rdf_atom.rs"))
        .unwrap();
}
