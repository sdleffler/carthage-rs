use std::{collections::{Bound, HashMap, btree_map::{BTreeMap, Entry},
                        btree_set::{BTreeSet, Range}},
          str::FromStr, sync::atomic::{AtomicUsize, Ordering}};

use failure::*;
use regex::Regex;
use url::Url;

use rdf::RdfAtom;

/// Counter for uniquely identifying documents so that blank nodes from one document cannot be
/// mixed with another.
static DOC_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// An IRI (Internationalized Resource Identifier) for use in an RDF store.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Iri(RdfAtom);

impl From<Url> for Iri {
    fn from(url: Url) -> Self {
        Iri(RdfAtom::from(url.as_str()))
    }
}

/// A blank node is an anonymous node with respect to one particular document.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blank {
    doc_id: usize,
    node_id: usize,
}

/// RDF subjects are either IRIs or blank nodes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Subject {
    Iri(Iri),
    Blank(Blank),
}

impl Subject {
    /// If the given subject is a blank node, then check to see whether the blank node originates
    /// from the given document. Otherwise, the subject is vacuously from the given document.
    fn has_doc_id(&self, doc_id: usize) -> bool {
        match *self {
            Subject::Blank(ref blank) => blank.doc_id == doc_id,
            _ => true,
        }
    }
}

/// RDF objects are either IRIs, blank nodes, or literals.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Object {
    Iri(Iri),
    Blank(Blank),
    Literal(Literal),
}

impl Object {
    /// If the given object is a blank node, then check to see whether the blank node originates
    /// from the given document. Otherwise, the object is vacuously from the given document.
    fn has_doc_id(&self, doc_id: usize) -> bool {
        match *self {
            Object::Blank(ref blank) => blank.doc_id == doc_id,
            _ => true,
        }
    }
}

/// RDF predicates are restricted to be IRIs only.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Predicate(pub Iri);

/// An RDF literal: a string literal with datatype or a language-tagged string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal {
    value: RdfAtom,
    ty: Iri,
    lang: Option<LangTag>,
}

impl Literal {
    pub fn string<S: AsRef<str>>(string: S) -> Self {
        Literal {
            value: RdfAtom::from(string.as_ref()),
            ty: Iri(rdf_atom!("http://www.w3.org/2001/XMLSchema#string")),
            lang: None,
        }
    }

    pub fn lang_string<S: AsRef<str>>(string: S, lang: LangTag) -> Self {
        Literal {
            value: RdfAtom::from(string.as_ref()),
            ty: Iri(rdf_atom!(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString"
            )),
            lang: Some(lang),
        }
    }

    pub fn typed<S: AsRef<str>>(string: S, ty: Iri) -> Self {
        Literal {
            value: RdfAtom::from(string.as_ref()),
            ty,
            lang: None,
        }
    }
}

/// A valid language tag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LangTag(RdfAtom);

impl FromStr for LangTag {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[a-zA-Z]+(?:-[a-zA-Z0-9]+)*").unwrap();
        }

        ensure!(RE.is_match(string), "bad language tag!");
        Ok(LangTag(RdfAtom::from(string)))
    }
}

/// An RDF triple: subject, predicate, and object.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Triple {
    pub subject: Subject,
    pub predicate: Predicate,
    pub object: Object,
}

impl Triple {
    /// Check whether or not all blank nodes in this triple originate from the given document.
    fn has_doc_id(&self, doc_id: usize) -> bool {
        // Predicates are only ever IRIs, no need to check.
        self.subject.has_doc_id(doc_id) && self.object.has_doc_id(doc_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Term {
    Subject(Subject),
    Predicate(Predicate),
    Object(Object),
    Alibi(Subject, Object),
    Perpetrator(Subject, Predicate),
    Victim(Predicate, Object),
}

impl Term {
    fn index_of(&self) -> Indices {
        match *self {
            Term::Subject(_) => Indices::BY_SUBJECT,
            Term::Predicate(_) => Indices::BY_PREDICATE,
            Term::Object(_) => Indices::BY_OBJECT,
            Term::Alibi(..) => Indices::BY_ALIBI,
            Term::Perpetrator(..) => Indices::BY_PERPETRATOR,
            Term::Victim(..) => Indices::BY_VICTIM,
        }
    }
}

bitflags! {
    pub struct Indices: u8 {
        const BY_SUBJECT     = 0b00000001;
        const BY_PREDICATE   = 0b00000010;
        const BY_OBJECT      = 0b00000100;
        const BY_ALIBI       = 0b00001000;
        const BY_PERPETRATOR = 0b00010000;
        const BY_VICTIM      = 0b00100000;
        const BY_SPO         = Self::BY_SUBJECT.bits
                             | Self::BY_PREDICATE.bits
                             | Self::BY_OBJECT.bits;
        const BY_ALL         = Self::BY_SPO.bits
                             | Self::BY_ALIBI.bits
                             | Self::BY_PERPETRATOR.bits
                             | Self::BY_VICTIM.bits;
    }
}

#[derive(Debug)]
pub struct Document {
    doc_id: usize,

    next_blank: usize,
    next_triple: usize,

    supported_indices: Indices,
    index: BTreeSet<(Term, usize)>,

    triples: HashMap<usize, Triple>,
    unique: BTreeMap<Triple, usize>,
}

impl Document {
    /// Create a new RDF document which supports efficient indexing over the given search terms.
    pub fn new(supported_indices: Indices) -> Self {
        let doc_id = DOC_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            doc_id,

            next_blank: 0,
            next_triple: 0,

            supported_indices,
            index: BTreeSet::new(),

            triples: HashMap::new(),
            unique: BTreeMap::new(),
        }
    }

    /// Create a new blank node tied to this document.
    pub fn create_blank(&mut self) -> Blank {
        let doc_id = self.doc_id;
        let node_id = self.next_blank;

        self.next_blank += 1;

        Blank { doc_id, node_id }
    }

    /// Insert a new triple into the store.
    ///
    /// This function will panic if the triple contains a blank node which does not originate from
    /// this store.
    pub fn insert(&mut self, triple: Triple) {
        assert!(triple.has_doc_id(self.doc_id));

        let (triple, triple_id) = match self.unique.entry(triple) {
            Entry::Occupied(_) => return,
            Entry::Vacant(vacant) => {
                let triple = vacant.key().clone();
                let triple_id = self.next_triple;

                self.next_triple += 1;
                vacant.insert(triple_id);

                (triple, triple_id)
            }
        };

        if self.supported_indices.contains(Indices::BY_SUBJECT) {
            self.index
                .insert((Term::Subject(triple.subject.clone()), triple_id));
        }

        if self.supported_indices.contains(Indices::BY_PREDICATE) {
            self.index
                .insert((Term::Predicate(triple.predicate.clone()), triple_id));
        }

        if self.supported_indices.contains(Indices::BY_OBJECT) {
            self.index
                .insert((Term::Object(triple.object.clone()), triple_id));
        }

        if self.supported_indices.contains(Indices::BY_ALIBI) {
            self.index.insert((
                Term::Alibi(triple.subject.clone(), triple.object.clone()),
                triple_id,
            ));
        }

        if self.supported_indices.contains(Indices::BY_PERPETRATOR) {
            self.index.insert((
                Term::Perpetrator(triple.subject.clone(), triple.predicate.clone()),
                triple_id,
            ));
        }

        if self.supported_indices.contains(Indices::BY_VICTIM) {
            self.index.insert((
                Term::Victim(triple.predicate.clone(), triple.object.clone()),
                triple_id,
            ));
        }

        self.triples.insert(triple_id, triple.clone());
    }

    /// Test if the document contains a triple.
    pub fn contains(&self, triple: &Triple) -> bool {
        self.unique.contains_key(triple)
    }

    /// Search for a term which is supported in the index. This function will panic if the required
    /// index is not supported by the store.
    pub fn search_index(&self, term: Term) -> Search {
        assert!(self.supported_indices.contains(term.index_of()));

        let start = (term, 0usize);

        let iter = self.index
            .range((Bound::Included(&start), Bound::Unbounded));
        let term = start.0;
        let triples = &self.triples;

        Search {
            term,
            iter,
            triples,
        }
    }
}

/// An iterator representing a search through a store's index for triples matching a specific
/// search `Term`.
#[derive(Clone)]
pub struct Search<'a> {
    term: Term,
    iter: Range<'a, (Term, usize)>,
    triples: &'a HashMap<usize, Triple>,
}

impl<'a> Iterator for Search<'a> {
    type Item = &'a Triple;

    fn next(&mut self) -> Option<Self::Item> {
        let Search {
            ref term,
            ref mut iter,
            triples,
        } = *self;

        // If we used `filter` instead of `take_while` we'd end up going through the entire index.
        iter.by_ref()
            .take_while(|&&(ref index_term, _)| index_term == term)
            .map(|&(_, ref id)| &triples[id])
            .next()
    }
}
