#![feature(const_fn, option_filter)]

#[macro_use]
extern crate bitflags;
extern crate bitwise;
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

#[macro_use]
pub mod document;
pub mod index;
pub mod ntriples;
pub mod view;
pub mod xsd;
