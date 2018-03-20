use std::{fmt, usize,
          collections::{Bound, HashMap, btree_map::{BTreeMap, Entry, Keys},
                        btree_set::{BTreeSet, Range}},
          str::FromStr, sync::atomic::{AtomicUsize, Ordering}, u32};

use failure::*;
use regex::Regex;
use url::Url;

use index::{Index, IndexSet};
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
    /// Create a new string literal with the standard XML Schema string type. This is the "default"
    /// string type for serialization formats like N-Triples, which will infer the XML string type
    /// (IRI `http://www.w3.org/2001/XMLSchema#string`) for literals without datatype or language
    /// tags.
    pub fn string<S: AsRef<str>>(string: S) -> Self {
        Literal {
            value: RdfAtom::from(string.as_ref()),
            ty: Iri(rdf_atom!("http://www.w3.org/2001/XMLSchema#string")),
            lang: None,
        }
    }

    /// Create a new language-tagged string literal with the RDF `langString` type, IRI
    /// `http://www.w3.org/1999/02/22-rdf-syntax-ns#langString`. This is the string typed inferred
    /// by formats like N-Triples or N-Quads when parsing string literals with language tags.
    pub fn lang_string<S: AsRef<str>>(string: S, lang: LangTag) -> Self {
        Literal {
            value: RdfAtom::from(string.as_ref()),
            ty: Iri(rdf_atom!(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString"
            )),
            lang: Some(lang),
        }
    }

    /// Create a literal with a non-string or language-tagged string type, from the raw UTF-8
    /// string and then the datatype IRI.
    pub fn typed<S: AsRef<str>>(string: S, ty: Iri) -> Self {
        Literal {
            value: RdfAtom::from(string.as_ref()),
            ty,
            lang: None,
        }
    }

    /// Get the value of this literal as a string slice.
    pub fn as_value(&self) -> &str {
        &self.value
    }

    /// Get the datatype of this literal as an IRI.
    pub fn as_ty(&self) -> &Iri {
        &self.ty
    }

    /// Get the language tag of this literal, only if the literal is a language-tagged string.
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

/// An RDF quad: subject, predicate, and object.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quad {
    pub subject: Subject,
    pub predicate: Predicate,
    pub object: Object,
    pub context: Option<Subject>,
}

impl Quad {
    /// Check whether or not all blank nodes in this quad originate from the given document.
    fn has_doc_id(&self, doc_id: u32) -> bool {
        // Predicates are only ever IRIs, no need to check.
        self.subject.has_doc_id(doc_id) && self.object.has_doc_id(doc_id)
            && self.context
                .as_ref()
                .map(|ctx| ctx.has_doc_id(doc_id))
                .unwrap_or(true)
    }

    /// Convert this quad into a search term by filtering subjects/predicates by index flags.
    fn to_term_by_index(&self, index: Index) -> Term {
        let mut term = Term::default();

        if index.contains(Index::SUBJECT) {
            term.subject = Some(self.subject.clone());
        }

        if index.contains(Index::PREDICATE) {
            term.predicate = Some(self.predicate.clone());
        }

        if index.contains(Index::OBJECT) {
            term.object = Some(self.object.clone());
        }

        if index.contains(Index::CONTEXT) {
            term.context = Some(self.context.clone());
        }

        term
    }
}

/// A search term. This allows for selective searching for triples/quads with given
/// subjects/predicates/objects/contexts. A `None` value for any field indicates that when
/// searching, the given field should be a "wildcard" or "variable". To match on an empty context,
/// use `Some(None)` as a value for the `context` field.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Term {
    pub subject: Option<Subject>,
    pub predicate: Option<Predicate>,
    pub object: Option<Object>,
    pub context: Option<Option<Subject>>,
}

impl Term {
    /// Check the index required to parse a given term.
    fn index_of(&self) -> Index {
        let mut index = Index::empty();

        index.set(Index::SUBJECT, self.subject.is_some());
        index.set(Index::PREDICATE, self.predicate.is_some());
        index.set(Index::OBJECT, self.object.is_some());
        index.set(Index::CONTEXT, self.context.is_some());

        index
    }
}

#[derive(Debug)]
pub struct Document {
    doc_id: u32,

    next_blank: u32,
    next_quad: usize,

    supported_indices: IndexSet,
    index: BTreeSet<(Term, usize)>,

    quads: HashMap<usize, Quad>,
    unique: BTreeMap<Quad, usize>,
}

impl Document {
    /// Create a new RDF document which supports efficient indexing over the given search terms.
    pub fn new(supported_indices: IndexSet) -> Self {
        let doc_id = {
            let tmp = DOC_COUNTER.fetch_add(1, Ordering::Relaxed);
            assert!(tmp <= u32::MAX as usize);
            tmp as u32
        };

        Self {
            doc_id,

            next_blank: 0,
            next_quad: 0,

            supported_indices,
            index: BTreeSet::new(),

            quads: HashMap::new(),
            unique: BTreeMap::new(),
        }
    }

    /// Add new indices to the document's quadstore, to speed-up searches. This will add new search
    /// terms to the index, effectively populating any indices which are contained in the supplied
    /// index set but not already supported by the quadstore.
    pub fn add_indices(&mut self, index_set: IndexSet) {
        let diff = index_set - self.supported_indices;
        self.supported_indices |= index_set;
        let Self {
            index: ref mut quad_index,
            ref quads,
            ..
        } = *self;

        quad_index.extend(diff.into_iter().flat_map(|index| {
            quads
                .iter()
                .map(move |(&id, quad)| (quad.to_term_by_index(index), id))
        }));
    }

    /// Remove indices from the document's quadstore. This will remove any indices in the
    /// intersection of the document's supported indices and the supplied index set.
    pub fn remove_indices(&mut self, index_set: IndexSet) {
        let intersect = self.supported_indices & index_set;
        self.supported_indices -= index_set;
        let Self {
            index: ref mut quad_index,
            ref quads,
            ..
        } = *self;

        let to_remove = intersect.into_iter().flat_map(|index| {
            quads
                .iter()
                .map(move |(&id, quad)| (quad.to_term_by_index(index), id))
        });

        for index_entry in to_remove {
            quad_index.remove(&index_entry);
        }
    }

    /// Create a new blank node tied to this document.
    pub fn create_blank(&mut self) -> Blank {
        let doc_id = self.doc_id;
        let node_id = self.next_blank;

        self.next_blank += 1;

        Blank { doc_id, node_id }
    }

    /// Insert a new quad into the store.
    ///
    /// This function will panic if the quad contains a blank node which does not originate from
    /// this store.
    pub fn insert(&mut self, quad: Quad) {
        assert!(quad.has_doc_id(self.doc_id));

        let (quad, quad_id) = match self.unique.entry(quad) {
            Entry::Occupied(_) => return,
            Entry::Vacant(vacant) => {
                let quad = vacant.key().clone();
                let quad_id = self.next_quad;

                self.next_quad += 1;
                vacant.insert(quad_id);

                (quad, quad_id)
            }
        };

        for supported_index in self.supported_indices {
            let mut term = Term::default();

            if supported_index.contains(Index::SUBJECT) {
                term.subject = Some(quad.subject.clone());
            }

            if supported_index.contains(Index::PREDICATE) {
                term.predicate = Some(quad.predicate.clone());
            }

            if supported_index.contains(Index::OBJECT) {
                term.object = Some(quad.object.clone());
            }

            if supported_index.contains(Index::CONTEXT) {
                term.context = Some(quad.context.clone());
            }

            self.index.insert((term, quad_id));
        }

        self.quads.insert(quad_id, quad.clone());
    }

    /// Test if the document contains a quad.
    pub fn contains(&self, quad: &Quad) -> bool {
        self.unique.contains_key(quad)
    }

    /// Search for a term which is supported in the index. This function will panic if the required
    /// index is not supported by the store.
    pub fn search_index(&self, term: Term) -> Search {
        assert!(self.supported_indices.contains(term.index_of()));

        let start = (term.clone(), usize::MIN);
        let end = (term, usize::MAX);

        let iter = self.index
            .range((Bound::Included(&start), Bound::Included(&end)));
        let quads = &self.quads;

        Search { iter, quads }
    }

    /// Iterate over all quads in the store. This iterator goes in ascending order with respect
    /// to the `Ord` instance of `Quad`.
    pub fn iter(&self) -> Iter {
        Iter {
            unique: self.unique.keys(),
        }
    }
}

impl Extend<Quad> for Document {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Quad>,
    {
        iter.into_iter().for_each(|quad| self.insert(quad));
    }
}

/// An iterator representing a search through a store's index for quads matching a specific
/// search `Term`.
#[derive(Clone)]
pub struct Search<'a> {
    iter: Range<'a, (Term, usize)>,
    quads: &'a HashMap<usize, Quad>,
}

impl<'a> Iterator for Search<'a> {
    type Item = &'a Quad;

    fn next(&mut self) -> Option<Self::Item> {
        let Search {
            ref mut iter,
            quads,
        } = *self;

        iter.next().map(|&(_, ref id)| &quads[id])
    }
}

/// An iterator over all quads in a document.
#[derive(Clone)]
pub struct Iter<'a> {
    unique: Keys<'a, Quad, usize>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Quad;

    fn next(&mut self) -> Option<Self::Item> {
        self.unique.next()
    }
}
