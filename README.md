# Carthage: RDF graph manipulation and parsing for Rust

*We're still a little salty about Rome.*

Carthage is a work-in-progress RDF graph parsing and manipulation library for
Rust, being built for use in [Attaca](https://github.com/attaca/attaca).

Progress:
- [x] N-Triples parser compatible with the W3C test suite.
- [x] N-Triples document is `fmt::Display`able.
- [x] In-memory store capable of flexible indexing.
- [ ] Convenience functions on in-memory triple store for working with RDF-formatted metadata.
- [ ] Configurable N-Triples formatter.
- [ ] Turtle parser compatible with the W3C test suite.
- [ ] Configurable Turtle formatter.

Eventual goals:
- Robust N-triples and Turtle parsers capable of passing the W3C test suites.
- N-triples and Turtle formatting capable of outputting "canonical forms" of graphs.
- Simple, in-memory RDF graph manipulation.
