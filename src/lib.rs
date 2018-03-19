#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate string_cache;
extern crate url;

#[macro_use]
mod rdf {
    include!(concat!(env!("OUT_DIR"), "/rdf_atom.rs"));
}

pub mod document;
pub mod ntriples;
pub mod traits;
