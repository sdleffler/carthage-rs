use std::hash::Hash;

use document::{Iri, Literal};

/// A reference to a node from a document view, which behaves like an IRI.
pub trait IriLike<'refr>: Ord + Hash + Clone {
    fn to_iri(&self) -> Iri;
}

/// A reference to a node from a document view, which behaves like a blank node.
///
/// Unlike the `IriLike` and `LiteralLike` traits, `BlankLike` does not support a `to_blank` method
/// equivalent to the `to_iri` and `to_literal` methods. This is due to technical limitations on
/// how blank nodes actually work, so to speak: blank nodes do not have a canonical representation
/// in any given format, while IRIs and literals have substructure (IRIs have canonical string
/// representations while literals have substructure which can be converted to canonical string
/// representations.)
pub trait BlankLike<'refr>: Ord + Hash + Clone {}

/// A reference to a node from a document view, which behaves like a literal.
///
/// Graph views are designed to be capable of no-copy abstractions over structs. As such, it
/// should be possible to directly attempt downcasting a `LiteralLike` type into a reference to a
/// Rust value.
pub trait LiteralLike<'refr>: Ord + Hash + Clone {
    fn to_literal(&self) -> Literal;

    fn downcast_ref<T: 'static>(&self) -> Option<&T>;
    fn downcast_str(&self) -> Option<&str>;
}

/// A "subject-like" value, which is either an IRI-like or blank-like value from an underlying
/// document view.
pub enum SubjectLike<'refr, D: ?Sized + GraphView<'refr>> {
    Iri(D::Iri),
    Blank(D::Blank),
}

/// A "predicate-like" value, consisting of an IRI-like value from an underlying document view.
pub struct PredicateLike<'refr, D: ?Sized + GraphView<'refr>>(pub D::Iri);

/// An "object-like" value, consisting of an IRI-like, blank-like, or literal-like value from an
/// underlying document view.
pub enum ObjectLike<'refr, D: ?Sized + GraphView<'refr>> {
    Iri(D::Iri),
    Blank(D::Blank),
    Literal(D::Literal),
}

/// A "quad-like" value, consisting of a subject-like value, predicate-like value, and
/// object-like value, from an underlying document view.
pub struct QuadLike<'refr, D: ?Sized + GraphView<'refr>> {
    pub subject: SubjectLike<'refr, D>,
    pub predicate: PredicateLike<'refr, D>,
    pub object: ObjectLike<'refr, D>,
    pub context: Option<SubjectLike<'refr, D>>,
}

/// A "term-like" value for searching an underlying document view.
pub struct TermLike<'refr, D: ?Sized + GraphView<'refr>> {
    pub subject: Option<SubjectLike<'refr, D>>,
    pub predicate: Option<PredicateLike<'refr, D>>,
    pub object: Option<ObjectLike<'refr, D>>,
    pub context: Option<Option<SubjectLike<'refr, D>>>,
}

/// Trait for types which can be used as a "document view", allowing them to be generically treated
/// as an RDF graph.
pub trait GraphView<'refr> {
    type Iri: IriLike<'refr>;
    type Blank: BlankLike<'refr>;
    type Literal: LiteralLike<'refr>;

    fn get_iri(&self, iri: &Iri) -> Option<Self::Iri>;

    type Quads: IntoIterator<Item = QuadLike<'refr, Self>>;
    fn quads(&self) -> Self::Quads;

    type Search: IntoIterator<Item = QuadLike<'refr, Self>>;
    fn search(&self, term: TermLike<'refr, Self>) -> Self::Search;
}

/// Trait for a type family, dispatched on lifetimes, for extracting a document view type from an
/// underlying `GraphLike` type.
pub trait GraphLike<'refr> {
    type View: GraphView<'refr>;

    fn as_view(&'refr self) -> Self::View;
}

#[cfg(test)]
mod tests {
    use super::*;

    const BAR_IRI: &'static str = "http://purl.org/sleffy/carthage-rs/examples/foo#bar";
    const BAZ_IRI: &'static str = "http://purl.org/sleffy/carthage-rs/examples/foo#baz";

    struct Foo<'doc> {
        bar: u64,
        baz: &'doc str,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum FooIri {
        This,
        Bar,
        Baz,
    }

    impl<'refr> IriLike<'refr> for FooIri {
        fn to_iri(&self) -> Iri {
            match *self {
                FooIri::This => "".parse().unwrap(),
                FooIri::Bar => BAR_IRI.parse().unwrap(),
                FooIri::Baz => BAZ_IRI.parse().unwrap(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum FooBlank {}

    impl<'refr> BlankLike<'refr> for FooBlank {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum FooLit<'refr> {
        U64(&'refr u64),
        Str(&'refr str),
    }

    impl<'refr> LiteralLike<'refr> for FooLit<'refr> {
        fn to_literal(&self) -> Literal {
            match *self {
                FooLit::U64(&val) => Literal::from(val),
                FooLit::Str(s) => Literal::from(s),
            }
        }

        fn downcast_ref<T: 'static>(&self) -> Option<&T> {
            use std::any::Any;

            match *self {
                FooLit::U64(val) => Any::downcast_ref(val),
                FooLit::Str(_) => None,
            }
        }

        fn downcast_str(&self) -> Option<&str> {
            match *self {
                FooLit::Str(s) => Some(s),
                _ => None,
            }
        }
    }

    struct FooView<'doc: 'refr, 'refr>(&'refr Foo<'doc>);

    struct FooQuads<'doc: 'refr, 'refr>(usize, &'refr Foo<'doc>);

    impl<'doc: 'refr, 'refr> Iterator for FooQuads<'doc, 'refr> {
        type Item = QuadLike<'refr, FooView<'doc, 'refr>>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.0 {
                0 => {
                    let quad = QuadLike {
                        subject: SubjectLike::Iri(FooIri::This),
                        predicate: PredicateLike(FooIri::Bar),
                        object: ObjectLike::Literal(FooLit::U64(&self.1.bar)),
                        context: None,
                    };
                    self.0 += 1;
                    Some(quad)
                }
                1 => {
                    let quad = QuadLike {
                        subject: SubjectLike::Iri(FooIri::This),
                        predicate: PredicateLike(FooIri::Baz),
                        object: ObjectLike::Literal(FooLit::Str(&self.1.baz)),
                        context: None,
                    };
                    self.0 += 1;
                    Some(quad)
                }
                _ => None,
            }
        }
    }

    bitflags! {
        struct FooReturned: u8 {
            const BAR  = 0b00000001;
            const BAZ  = 0b00000010;
        }
    }

    struct FooSearch<'doc: 'refr, 'refr>(FooReturned, &'refr Foo<'doc>);

    impl<'doc: 'refr, 'refr> Iterator for FooSearch<'doc, 'refr> {
        type Item = QuadLike<'refr, FooView<'doc, 'refr>>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.0.bits().trailing_zeros() {
                0 => {
                    self.0 -= FooReturned::BAR;
                    Some(QuadLike {
                        subject: SubjectLike::Iri(FooIri::This),
                        predicate: PredicateLike(FooIri::Bar),
                        object: ObjectLike::Literal(FooLit::U64(&self.1.bar)),
                        context: None,
                    })
                }
                1 => {
                    self.0 -= FooReturned::BAZ;
                    Some(QuadLike {
                        subject: SubjectLike::Iri(FooIri::This),
                        predicate: PredicateLike(FooIri::Baz),
                        object: ObjectLike::Literal(FooLit::Str(&self.1.baz)),
                        context: None,
                    })
                }
                _ => None,
            }
        }
    }

    impl<'doc: 'refr, 'refr> GraphView<'refr> for FooView<'doc, 'refr> {
        type Iri = FooIri;
        type Blank = FooBlank;
        type Literal = FooLit<'refr>;

        fn get_iri(&self, iri: &Iri) -> Option<Self::Iri> {
            match iri.as_str() {
                "" => Some(FooIri::This),
                BAR_IRI => Some(FooIri::Bar),
                BAZ_IRI => Some(FooIri::Baz),
                _ => None,
            }
        }

        type Quads = FooQuads<'doc, 'refr>;

        fn quads(&self) -> Self::Quads {
            FooQuads(0, self.0)
        }

        type Search = FooSearch<'doc, 'refr>;

        fn search(&self, term: TermLike<'refr, Self>) -> Self::Search {
            let mut set = FooReturned::all();

            match term.subject {
                Some(SubjectLike::Iri(ref iri)) if iri != &FooIri::This => {
                    set = FooReturned::empty()
                }
                _ => {}
            }

            match term.predicate {
                Some(PredicateLike(FooIri::Bar)) => set &= FooReturned::BAR,
                Some(PredicateLike(FooIri::Baz)) => set &= FooReturned::BAZ,
                Some(_) => set = FooReturned::empty(),
                None => {}
            }

            match term.object {
                Some(ObjectLike::Literal(FooLit::U64(_))) => set &= FooReturned::BAR,
                Some(ObjectLike::Literal(FooLit::Str(_))) => set &= FooReturned::BAZ,
                Some(_) => set = FooReturned::empty(),
                None => {}
            }

            if term.context.is_some() {
                set = FooReturned::empty();
            }

            FooSearch(set, self.0)
        }
    }

    impl<'doc: 'refr, 'refr> GraphLike<'refr> for Foo<'doc> {
        type View = FooView<'doc, 'refr>;

        fn as_view(&'refr self) -> Self::View {
            FooView(self)
        }
    }
}
