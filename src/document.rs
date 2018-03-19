use std::{fmt, usize,
          collections::{Bound, HashMap, btree_map::{BTreeMap, Entry, Keys},
                        btree_set::{BTreeSet, Range}},
          str::FromStr, sync::atomic::{AtomicUsize, Ordering}, u32};

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

/// Macro to create an IRI from a raw atom, which will only compile if the relevant string is
/// interned.
#[macro_export]
macro_rules! iri {
    ($interned:tt) => { Iri::unsafe_from_raw_atom(rdf_atom!($interned)) };
}

impl FromStr for Iri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let atom = if s != "" {
            let url = Url::parse(s)?;
            RdfAtom::from(url.as_str())
        } else {
            rdf_atom!("")
        };

        Ok(Iri(atom))
    }
}

impl Iri {
    /// Create an IRI from a raw interned string. This is dangerous because it does not check that
    /// the atom is parseable as a valid IRI.
    pub const fn unsafe_from_raw_atom(atom: RdfAtom) -> Self {
        Iri(atom)
    }

    /// Get this IRI as a formatted string.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Iri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A blank node is an anonymous node with respect to one particular document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blank {
    doc_id: u32,
    node_id: u32,
}

impl Blank {
    pub fn to_id(&self) -> u64 {
        ((self.doc_id as u64) << 32) | (self.node_id as u64)
    }
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
    fn has_doc_id(&self, doc_id: u32) -> bool {
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
    fn has_doc_id(&self, doc_id: u32) -> bool {
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

impl<'a> From<&'a str> for Literal {
    fn from(s: &'a str) -> Self {
        Self::string(s)
    }
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

    pub fn as_value(&self) -> &str {
        &self.value
    }

    pub fn as_ty(&self) -> &Iri {
        &self.ty
    }

    pub fn as_lang_tag(&self) -> Option<&LangTag> {
        self.lang.as_ref()
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

impl fmt::Display for LangTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
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
    fn has_doc_id(&self, doc_id: u32) -> bool {
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
    doc_id: u32,

    next_blank: u32,
    next_triple: usize,

    supported_indices: Indices,
    index: BTreeSet<(Term, usize)>,

    triples: HashMap<usize, Triple>,
    unique: BTreeMap<Triple, usize>,
}

impl Document {
    /// Create a new RDF document which supports efficient indexing over the given search terms.
    pub fn new(supported_indices: Indices) -> Self {
        let doc_id = {
            let tmp = DOC_COUNTER.fetch_add(1, Ordering::Relaxed);
            assert!(tmp <= u32::MAX as usize);
            tmp as u32
        };

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

        let start = (term.clone(), usize::MIN);
        let end = (term, usize::MAX);

        let iter = self.index
            .range((Bound::Included(&start), Bound::Included(&end)));
        let triples = &self.triples;

        Search { iter, triples }
    }

    /// Iterate over all triples in the store. This iterator goes in ascending order with respect
    /// to the `Ord` instance of `Triple`.
    pub fn iter(&self) -> Iter {
        Iter {
            unique: self.unique.keys(),
        }
    }
}

impl Extend<Triple> for Document {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Triple>,
    {
        iter.into_iter().for_each(|triple| self.insert(triple));
    }
}

/// An iterator representing a search through a store's index for triples matching a specific
/// search `Term`.
#[derive(Clone)]
pub struct Search<'a> {
    iter: Range<'a, (Term, usize)>,
    triples: &'a HashMap<usize, Triple>,
}

impl<'a> Iterator for Search<'a> {
    type Item = &'a Triple;

    fn next(&mut self) -> Option<Self::Item> {
        let Search {
            ref mut iter,
            triples,
        } = *self;

        iter.next().map(|&(_, ref id)| &triples[id])
    }
}

/// An iterator over all triples in a document.
#[derive(Clone)]
pub struct Iter<'a> {
    unique: Keys<'a, Triple, usize>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Triple;

    fn next(&mut self) -> Option<Self::Item> {
        self.unique.next()
    }
}
