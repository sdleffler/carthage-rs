extern crate lalrpop;
extern crate string_cache_codegen;

use std::{env, path::Path};

const ATOMS: &'static [&'static str] = &[
    // builtin RDF language-tagged string type
    "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString",

    // Primitive XML Schema types
    "http://www.w3.org/2001/XMLSchema#string",
    "http://www.w3.org/2001/XMLSchema#byte",
    "http://www.w3.org/2001/XMLSchema#short",
    "http://www.w3.org/2001/XMLSchema#int",
    "http://www.w3.org/2001/XMLSchema#long",
    "http://www.w3.org/2001/XMLSchema#unsignedByte",
    "http://www.w3.org/2001/XMLSchema#unsignedShort",
    "http://www.w3.org/2001/XMLSchema#unsignedInt",
    "http://www.w3.org/2001/XMLSchema#unsignedLong",
];

fn main() {
    lalrpop::process_root().unwrap();
    string_cache_codegen::AtomType::new("rdf::RdfAtom", "rdf_atom!")
        .atoms(ATOMS)
        .write_to_file(&Path::new(&env::var("OUT_DIR").unwrap()).join("rdf_atom.rs"))
        .unwrap();
}
