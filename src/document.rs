use std::{cell::RefCell, cmp::Ordering, collections::{BTreeMap, HashMap, HashSet}, rc::Rc};

use string_interner::StringInterner;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Subject {
    Iri(usize),
    Blank(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Object {
    Iri(usize),
    Blank(usize),
    Literal(Literal),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Predicate(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Literal {
    value: usize,
    ty: Option<Datatype>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Datatype {
    Iri(usize),
    LangTag(usize),
}

#[derive(Debug, Clone)]
struct Triple {
    interner: Rc<RefCell<StringInterner<usize>>>,

    subject: Subject,
    predicate: Predicate,
    object: Object,
}

impl PartialEq for Triple {
    fn eq(&self, rhs: &Triple) -> bool {
        self.subject == rhs.subject && self.predicate == rhs.predicate && self.object == rhs.object
    }
}

impl Eq for Triple {}

impl PartialOrd for Triple {
    fn partial_cmp(&self, rhs: &Triple) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Triple {
    fn cmp(&self, rhs: &Triple) -> Ordering {
        let subject_ordering = match (self.subject, rhs.subject) {
            (Subject::Iri(_), Subject::Blank(_)) => Ordering::Less,
            (Subject::Blank(_), Subject::Iri(_)) => Ordering::Greater,
            (Subject::Iri(l), Subject::Iri(r)) | (Subject::Blank(l), Subject::Blank(r)) => {
                let (left_interner, right_interner) =
                    (self.interner.borrow(), rhs.interner.borrow());
                let (left_str, right_str) = (
                    left_interner.resolve(l).unwrap(),
                    right_interner.resolve(r).unwrap(),
                );
                left_str.cmp(right_str)
            }
        };

        if subject_ordering != Ordering::Equal {
            return subject_ordering;
        }

        let predicate_ordering = {
            let (left_interner, right_interner) = (self.interner.borrow(), rhs.interner.borrow());
            let (left_str, right_str) = (
                left_interner.resolve(self.predicate.0).unwrap(),
                right_interner.resolve(rhs.predicate.0).unwrap(),
            );
            left_str.cmp(right_str)
        };

        if predicate_ordering != Ordering::Equal {
            return predicate_ordering;
        }

        match (self.object, rhs.object) {
            (Object::Iri(l), Object::Iri(r)) | (Object::Blank(l), Object::Blank(r)) => {
                let (left_interner, right_interner) =
                    (self.interner.borrow(), rhs.interner.borrow());
                let (left_str, right_str) = (
                    left_interner.resolve(l).unwrap(),
                    right_interner.resolve(r).unwrap(),
                );
                left_str.cmp(right_str)
            }
            (Object::Literal(left_lit), Object::Literal(right_lit)) => {
                let (left_interner, right_interner) =
                    (self.interner.borrow(), rhs.interner.borrow());
                let (left_value, right_value) = (
                    left_interner.resolve(left_lit.value).unwrap(),
                    right_interner.resolve(right_lit.value).unwrap(),
                );

                let value_ordering = left_value.cmp(right_value);
                if value_ordering != Ordering::Equal {
                    return value_ordering;
                }

                match (left_lit.ty, right_lit.ty) {
                    (None, None) => Ordering::Equal,
                    (Some(Datatype::Iri(l)), Some(Datatype::Iri(r))) => left_interner
                        .resolve(l)
                        .unwrap()
                        .cmp(right_interner.resolve(r).unwrap()),
                    (Some(Datatype::LangTag(l)), Some(Datatype::LangTag(r))) => left_interner
                        .resolve(l)
                        .unwrap()
                        .cmp(right_interner.resolve(r).unwrap()),
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                    (Some(Datatype::Iri(_)), _) => Ordering::Greater,
                    (_, Some(Datatype::Iri(_))) => Ordering::Less,
                }
            }
            (Object::Iri(_), _) => Ordering::Greater,
            (_, Object::Iri(_)) => Ordering::Less,
            (Object::Blank(_), _) => Ordering::Greater,
            (_, Object::Blank(_)) => Ordering::Less,
        }
    }
}

pub struct RdfStore {
    interner: Rc<RefCell<StringInterner<usize>>>,

    by_subject: HashMap<Subject, HashSet<usize>>,
    by_predicate: HashMap<Predicate, HashSet<usize>>,
    by_object: HashMap<Object, HashSet<usize>>,

    triples: HashMap<usize, (Subject, Predicate, Object)>,
    unique: BTreeMap<Triple, usize>,
}
