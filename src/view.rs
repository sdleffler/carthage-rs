use document::{Blank, Iri, Literal};

pub trait IriLike<'doc: 'refr, 'refr> {
    fn to_iri(&self) -> Iri;
}

pub trait BlankLike<'doc: 'refr, 'refr> {
    fn to_blank(&self) -> Blank;
}

pub trait LiteralLike<'doc: 'refr, 'refr> {
    fn to_literal(&self) -> Literal;

    fn downcast_ref<T: 'static>(&self) -> Option<&T>;
    fn downcast_str(&self) -> Option<&str>;
}

pub enum SubjectLike<'doc: 'refr, 'refr, D: ?Sized + DocumentView<'doc, 'refr>> {
    Iri(D::Iri),
    Blank(D::Blank),
}

pub struct PredicateLike<'doc: 'refr, 'refr, D: ?Sized + DocumentView<'doc, 'refr>>(pub D::Iri);

pub enum ObjectLike<'doc: 'refr, 'refr, D: ?Sized + DocumentView<'doc, 'refr>> {
    Iri(D::Iri),
    Blank(D::Blank),
    Literal(D::Literal),
}

pub struct TripleLike<'doc: 'refr, 'refr, D: ?Sized + DocumentView<'doc, 'refr>> {
    pub subject: SubjectLike<'doc, 'refr, D>,
    pub predicate: PredicateLike<'doc, 'refr, D>,
    pub object: ObjectLike<'doc, 'refr, D>,
}

pub trait DocumentViewFamily<'doc: 'refr, 'refr> {
    type View: DocumentView<'doc, 'refr>;
}

pub trait DocumentView<'doc: 'refr, 'refr> {
    type Iri: IriLike<'doc, 'refr>;
    type Blank: BlankLike<'doc, 'refr>;
    type Literal: LiteralLike<'doc, 'refr>;

    type Triples: IntoIterator<Item = TripleLike<'doc, 'refr, Self>>;

    fn triples(&self) -> Self::Triples;
}

pub trait DocumentLike<'doc> {
    type ViewFamily;

    fn as_view<'refr>(&'refr self) -> <Self::ViewFamily as DocumentViewFamily<'doc, 'refr>>::View
    where
        'doc: 'refr,
        Self::ViewFamily: DocumentViewFamily<'doc, 'refr>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Foo<'doc> {
        bar: u64,
        baz: &'doc str,
    }

    enum FooIri {
        This,
        Bar,
        Baz,
    }

    impl<'doc: 'refr, 'refr> IriLike<'doc, 'refr> for FooIri {
        fn to_iri(&self) -> Iri {
            match *self {
                FooIri::This => "".parse().unwrap(),
                FooIri::Bar => "http://purl.org/sleffy/carthage-rs/examples/foo#bar"
                    .parse()
                    .unwrap(),
                FooIri::Baz => "http://purl.org/sleffy/carthage-rs/examples/foo#baz"
                    .parse()
                    .unwrap(),
            }
        }
    }

    enum FooBlank {}

    impl<'doc: 'refr, 'refr> BlankLike<'doc, 'refr> for FooBlank {
        fn to_blank(&self) -> Blank {
            match *self {}
        }
    }

    enum FooLit<'refr> {
        U64(&'refr u64),
        Str(&'refr str),
    }

    impl<'doc: 'refr, 'refr> LiteralLike<'doc, 'refr> for FooLit<'refr> {
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

    struct FooTriples<'doc: 'refr, 'refr>(usize, &'refr Foo<'doc>);

    impl<'doc: 'refr, 'refr> Iterator for FooTriples<'doc, 'refr> {
        type Item = TripleLike<'doc, 'refr, FooView<'doc, 'refr>>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.0 {
                0 => {
                    let triple = TripleLike {
                        subject: SubjectLike::Iri(FooIri::This),
                        predicate: PredicateLike(FooIri::Bar),
                        object: ObjectLike::Literal(FooLit::U64(&self.1.bar)),
                    };
                    self.0 += 1;
                    Some(triple)
                }
                1 => {
                    let triple = TripleLike {
                        subject: SubjectLike::Iri(FooIri::This),
                        predicate: PredicateLike(FooIri::Baz),
                        object: ObjectLike::Literal(FooLit::Str(&self.1.baz)),
                    };
                    self.0 += 1;
                    Some(triple)
                }
                _ => None,
            }
        }
    }

    impl<'doc: 'refr, 'refr> DocumentView<'doc, 'refr> for FooView<'doc, 'refr> {
        type Iri = FooIri;
        type Blank = FooBlank;
        type Literal = FooLit<'refr>;

        type Triples = FooTriples<'doc, 'refr>;

        fn triples(&self) -> Self::Triples {
            FooTriples(0, self.0)
        }
    }

    struct FooViewFam;

    impl<'doc: 'refr, 'refr> DocumentViewFamily<'doc, 'refr> for FooViewFam {
        type View = FooView<'doc, 'refr>;
    }

    impl<'doc> DocumentLike<'doc> for Foo<'doc> {
        type ViewFamily = FooViewFam;

        fn as_view<'refr>(&'refr self) -> <FooViewFam as DocumentViewFamily<'doc, 'refr>>::View
        where
            'doc: 'refr,
        {
            FooView(self)
        }
    }
}
