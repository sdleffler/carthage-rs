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
/// part of the statically interned set.
#[macro_export]
macro_rules! iri {
    ($interned:tt) => { Iri::unsafe_from_raw_atom(rdf_atom!($interned)) };
}

impl FromStr for Iri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // NB the empty string is special-cased: while not a valid URL it represents references to
        // a document being described; a sort of "this" IRI.
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

    /// Check whether or not a quad "matches" this term.
    fn is_match(&self, quad: &Quad) -> bool {
        let subject_matches = self.subject
            .as_ref()
            .map(|subj| subj == &quad.subject)
            .unwrap_or(true);

        let predicate_matches = self.predicate
            .as_ref()
            .map(|pred| pred == &quad.predicate)
            .unwrap_or(true);

        let object_matches = self.object
            .as_ref()
            .map(|obj| obj == &quad.object)
            .unwrap_or(true);

        let context_matches = self.context
            .as_ref()
            .map(|ctx| ctx == &quad.context)
            .unwrap_or(true);

        subject_matches && predicate_matches && object_matches && context_matches
    }

    /// Remove all components of this term not tracked by the given index.
    fn remove_untracked(mut self, idx: &Index) -> Self {
        {
            let Self {
                ref mut subject,
                ref mut predicate,
                ref mut object,
                ref mut context,
            } = self;

            *subject = subject.take().filter(|_| idx.contains(Index::SUBJECT));
            *predicate = predicate.take().filter(|_| idx.contains(Index::PREDICATE));
            *object = object.take().filter(|_| idx.contains(Index::OBJECT));
            *context = context.take().filter(|_| idx.contains(Index::CONTEXT));
        }

        self
    }
}

/// An RDF quad-store which supports efficient configurable indexing.
///
/// There are sixteen possible indices for the quad-store to support, corresponding to the sixteen
/// possible combinations of `Index::{SUBJECT, PREDICATE, OBJECT, CONTEXT}`. While allowed, the
/// combinations `Index::all()` and `Index::empty()` - corresponding to a unique set of quads which
/// is already kept internally and a useless index consisting of an unordered set of all quads in
/// the map - have no reason to ever be used.
///
/// Internally, there are three main components:
/// - The index, a `BTreeSet` used as a multimap, and queried for ranges whenever the quadstore is
///   searched; elements are tuples of search terms and integer IDs.
/// - The quad map, a `HashMap` from internal integer quad IDs to the quads themselves
/// - The unique map, a `BTreeMap` from quads to internal integer IDs.
///
/// The unique map is used to find when a quad is already in the store, and the `quads` map exists
/// as a more efficient alternative to simply storing every single quad after a relevant search
/// term. Despite quads being stored in two places (and search terms being stored as well) strings
/// are efficiently handled; all strings stored in a `Document` are interned.
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
    ///
    /// Adding or removing indices will not add or remove quads from the store.
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
    ///
    /// Adding or removing indices will not add or remove quads from the store.
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

    /// Search the *index* for a given range, optionally filtered by a search term (in case the
    /// given range term is just an approximation.)
    fn search_index_range(&self, term: Term, filter: Option<Term>) -> Search {
        let start = (term.clone(), usize::MIN);
        let end = (term, usize::MAX);

        let iter = self.index
            .range((Bound::Included(&start), Bound::Included(&end)));
        let quads = &self.quads;

        Search {
            inner: SearchInner::Index {
                filter,
                iter,
                quads,
            },
        }
    }

    /// Search for a term which is supported in the index.
    pub fn search(&self, search_term: Term) -> Search {
        let term_index = search_term.index_of();

        if self.supported_indices.contains(term_index) {
            self.search_index_range(search_term, None)
        } else if let Some(approx) = self.supported_indices.find_approximation(term_index) {
            let filter = search_term.clone();
            let term = search_term.remove_untracked(&approx);

            self.search_index_range(term, Some(filter))
        } else {
            Search {
                inner: SearchInner::Quads {
                    filter: search_term,
                    iter: self.unique.keys(),
                },
            }
        }
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

/// The internal search iterator.
#[derive(Clone)]
enum SearchInner<'a> {
    /// If the search found an efficient index to use, then we have an available range query on the
    /// index set, an optional search term filter in case the index isn't optimal, and a reference
    /// to the quad map to resolve quad IDs to quad references.
    Index {
        filter: Option<Term>,
        iter: Range<'a, (Term, usize)>,
        quads: &'a HashMap<usize, Quad>,
    },

    /// If the search didn't find any relevant index, we have to search through all quads. This
    /// variant represents that with an iterator over the keys of the unique mapping as well as the
    /// search term filter.
    Quads {
        filter: Term,
        iter: Keys<'a, Quad, usize>,
    },
}

impl<'a> Iterator for SearchInner<'a> {
    type Item = &'a Quad;

    fn next(&mut self) -> Option<Self::Item> {
        use self::SearchInner::*;

        match *self {
            Index {
                ref filter,
                ref mut iter,
                quads,
            } => iter.map(|&(_, ref id)| &quads[id])
                .filter(|quad| {
                    filter
                        .as_ref()
                        .map(|term| term.is_match(quad))
                        .unwrap_or(true)
                })
                .next(),
            Quads {
                ref filter,
                ref mut iter,
            } => iter.filter(|quad| filter.is_match(quad)).next(),
        }
    }
}

/// An iterator representing a search through a store's index for quads matching a specific
/// search `Term`.
#[derive(Clone)]
pub struct Search<'a> {
    inner: SearchInner<'a>,
}

impl<'a> Iterator for Search<'a> {
    type Item = &'a Quad;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
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
