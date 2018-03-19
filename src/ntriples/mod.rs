mod ast;
mod lexer;

// rustfmt does not like LALRPOP :c
#[cfg_attr(rustfmt, rustfmt_skip)]
mod parser;

use std::{fmt, borrow::Borrow, collections::HashMap, hash::{Hash, Hasher}, ops::{Deref, DerefMut},
          str::FromStr};

use failure::*;

use document::{Blank, Document, Indices, Literal, Object, Predicate, Subject, Triple};
use rdf::RdfAtom;

/// Wrapper type for interned strings which hashes on the string data instead of the ID of the
/// interned string, thus allowing it to safely implement `Borrow<str>` and be used as a convenient
/// key in hashmaps.
#[derive(Debug, PartialEq, Eq)]
struct Stringy(RdfAtom);

impl Borrow<str> for Stringy {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Hash for Stringy {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        (&*self.0).hash(hasher);
    }
}

/// A document in N-Triples format. This is a wrapper over the `Document` type which includes a
/// mapping from strings to blank nodes, which during formatting as N-Triples is inverted and used
/// to name blank nodes in the output N-Triples.
#[derive(Debug)]
pub struct NTriplesDocument {
    blanks: HashMap<Stringy, Blank>,
    doc: Document,
}

impl From<Document> for NTriplesDocument {
    fn from(doc: Document) -> Self {
        Self {
            blanks: HashMap::new(),
            doc,
        }
    }
}

impl Deref for NTriplesDocument {
    type Target = Document;

    fn deref(&self) -> &Self::Target {
        &self.doc
    }
}

impl DerefMut for NTriplesDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.doc
    }
}

impl FromStr for NTriplesDocument {
    type Err = Error;

    fn from_str<'a>(string: &'a str) -> Result<Self, Self::Err> {
        let mut doc = NTriplesDocument::new(Document::new(Indices::BY_ALL));
        doc.extend_from_str(string)?;
        Ok(doc)
    }
}

impl NTriplesDocument {
    /// Create a new `NTriplesDocument` with an empty blank node mapping from a raw RDF document.
    pub fn new(document: Document) -> Self {
        NTriplesDocument::from(document)
    }

    /// Extend the document with N-Triples parsed from the given string. If a parse error results,
    /// no triples will have been added to the document.
    pub fn extend_from_str<S: AsRef<str>>(&mut self, string: S) -> Result<(), Error> {
        use self::lexer::Lexer;

        let s = string.as_ref();
        let triples = match parser::parse_Document(Lexer::new(s, 0)) {
            Ok(doc) => doc.triples,
            Err(parse_err) => bail!("{}", parse_err),
        };

        for triple in triples {
            let subject = match triple.subject {
                ast::Subject::IriRef(string) => Subject::Iri(string.parse()?),
                ast::Subject::BlankNodeLabel(string) => {
                    Subject::Blank(self.create_named_blank(string))
                }
            };
            let predicate = Predicate(triple.predicate.parse()?);
            let object = match triple.object {
                ast::Object::IriRef(string) => Object::Iri(string.parse()?),
                ast::Object::BlankNodeLabel(string) => {
                    Object::Blank(self.create_named_blank(string))
                }
                ast::Object::Literal(literal) => {
                    let literal = match literal.datatype {
                        None => Literal::string(literal.value),
                        Some(ast::Datatype::IriRef(string)) => {
                            Literal::typed(literal.value, string.parse()?)
                        }
                        Some(ast::Datatype::LangTag(string)) => {
                            Literal::lang_string(literal.value, string.parse()?)
                        }
                    };

                    Object::Literal(literal)
                }
            };

            self.insert(Triple {
                subject,
                predicate,
                object,
            });
        }

        Ok(())
    }

    /// Retrieve the `Blank` associated with the given string name. Or, create a new blank node in
    /// the underlying `Document` and associate it with the given string (if there is no such node
    /// already.)
    pub fn create_named_blank<S: AsRef<str>>(&mut self, name: S) -> Blank {
        let NTriplesDocument {
            ref mut blanks,
            ref mut doc,
        } = *self;

        blanks.get(name.as_ref()).cloned().unwrap_or_else(|| {
            let blank = doc.create_blank();
            blanks.insert(Stringy(name.as_ref().into()), blank.clone());
            blank
        })
    }
}

/// Re-format an `NTriplesDocument` as a string of UTF-8 in valid N-Triples format.
impl fmt::Display for NTriplesDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut inverse = self.blanks
            .iter()
            .map(|(stringy, blank)| (blank.clone(), stringy.0.clone()))
            .collect::<HashMap<_, _>>();

        for triple in self.doc.iter() {
            match triple.subject {
                Subject::Iri(ref iri) => write!(f, "<{}>", iri)?,
                Subject::Blank(ref blank) => match inverse.get(blank).cloned() {
                    Some(string) => write!(f, "_:{}", string)?,
                    None => {
                        // FIXME simply shove underscores onto the end until it no longer collides
                        // with crap
                        let mut s = format!("{}", blank.to_id());
                        if self.blanks.contains_key(s.as_str()) {
                            s.push('_');
                        }
                        inverse.insert(blank.clone(), s.as_str().into());
                        write!(f, "_:{}", s)?
                    }
                },
            }

            write!(f, " <{}> ", triple.predicate.0)?;

            match triple.object {
                Object::Iri(ref iri) => write!(f, "<{}>", iri)?,
                Object::Blank(ref blank) => match inverse.get(blank).cloned() {
                    Some(string) => write!(f, "_:{}", string)?,
                    None => {
                        // FIXME simply shove underscores onto the end until it no longer collides
                        // with crap
                        let mut s = format!("{}", blank.to_id());
                        if self.blanks.contains_key(s.as_str()) {
                            s.push('_');
                        }
                        inverse.insert(blank.clone(), s.as_str().into());
                        write!(f, "_:{}", s)?
                    }
                },
                Object::Literal(ref literal) => {
                    write!(f, "{}", literal.as_value())?;

                    match literal.as_lang_tag() {
                        Some(ref lang_tag) => write!(f, "@{}", lang_tag)?,
                        None => write!(f, "^^<{}>", literal.as_ty())?,
                    }
                }
            }

            writeln!(f, " . ")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs::File, path::PathBuf};

    macro_rules! parse_test_files {
        (@test $(#[$m:meta])* $test:ident) => {
            parse_test_files!(@test $(#[$m:meta])* $test where path = stringify!($test));
        };
        (@test $(#[$m:meta])* $test:ident where path = $path:expr) => {
            #[test]
            $(#[$m])*
            fn $test() {
                use std::io::Read;

                let string = {
                    let mut path = PathBuf::from("ntriples-tests");
                    path.push(String::from($path) + ".nt");
                    let mut file = File::open(path).unwrap();
                    let mut buf = String::new();
                    file.read_to_string(&mut buf).unwrap();
                    buf
                };

                match NTriplesDocument::from_str(&string) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("{}", error);
                        panic!("Test failed!");
                    }
                }
            }
        };
        ($( $test_name:ident { $($(#[$m:meta])* $test:ident $(where $opt:tt = $val:expr)*,)* } )*) => {
            $(
                mod $test_name {
                    use super::*;

                    $(parse_test_files!(@test $(#[$m])* $test $(where $opt = $val)*);)*
                }
            )*
        };
    }

    parse_test_files! {
        arbitrary {
            test,
        }

        w3c {
            comment_following_triple,
            langtagged_string,
            lantag_with_subtag,
            literal_all_controls,
            literal_all_punctuation,
            literal_ascii_boundaries,
            literal_false,
            literal,
            literal_true,
            literal_with_2_dquotes,
            literal_with_2_squotes,
            literal_with_backspace where path = "literal_with_BACKSPACE",
            literal_with_carriage_return where path = "literal_with_CARRIAGE_RETURN",
            literal_with_character_tabulation where path = "literal_with_CHARACTER_TABULATION",
            literal_with_dquote,
            literal_with_form_feed where path = "literal_with_FORM_FEED",
            literal_with_line_feed where path = "literal_with_LINE_FEED",
            literal_with_numeric_escape4,
            literal_with_numeric_escape8,
            literal_with_reverse_solidus2 where path = "literal_with_REVERSE_SOLIDUS2",
            literal_with_reverse_solidus where path = "literal_with_REVERSE_SOLIDUS",
            literal_with_squote,
            literal_with_utf8_boundaries where path = "literal_with_UTF8_boundaries",
            minimal_whitespace,

            #[should_panic] nt_syntax_bad_base_01 where path = "nt-syntax-bad-base-01",

            #[should_panic] nt_syntax_bad_esc_01 where path = "nt-syntax-bad-esc-01",
            #[should_panic] nt_syntax_bad_esc_02 where path = "nt-syntax-bad-esc-02",
            #[should_panic] nt_syntax_bad_esc_03 where path = "nt-syntax-bad-esc-03",
            #[should_panic] nt_syntax_bad_lang_01 where path = "nt-syntax-bad-lang-01",
            #[should_panic] nt_syntax_bad_num_01 where path = "nt-syntax-bad-num-01",
            #[should_panic] nt_syntax_bad_num_02 where path = "nt-syntax-bad-num-02",
            #[should_panic] nt_syntax_bad_num_03 where path = "nt-syntax-bad-num-03",
            #[should_panic] nt_syntax_bad_prefix_01 where path = "nt-syntax-bad-prefix-01",
            #[should_panic] nt_syntax_bad_string_01 where path = "nt-syntax-bad-string-01",
            #[should_panic] nt_syntax_bad_string_02 where path = "nt-syntax-bad-string-02",
            #[should_panic] nt_syntax_bad_string_03 where path = "nt-syntax-bad-string-03",
            #[should_panic] nt_syntax_bad_string_04 where path = "nt-syntax-bad-string-04",
            #[should_panic] nt_syntax_bad_string_05 where path = "nt-syntax-bad-string-05",

            #[should_panic] nt_syntax_bad_string_06 where path = "nt-syntax-bad-string-06",
            #[should_panic] nt_syntax_bad_string_07 where path = "nt-syntax-bad-string-07",
            #[should_panic] nt_syntax_bad_struct_01 where path = "nt-syntax-bad-struct-01",
            #[should_panic] nt_syntax_bad_struct_02 where path = "nt-syntax-bad-struct-02",

            #[should_panic] nt_syntax_bad_uri_01 where path = "nt-syntax-bad-uri-01",
            #[should_panic] nt_syntax_bad_uri_02 where path = "nt-syntax-bad-uri-02",
            #[should_panic] nt_syntax_bad_uri_03 where path = "nt-syntax-bad-uri-03",
            #[should_panic] nt_syntax_bad_uri_04 where path = "nt-syntax-bad-uri-04",
            #[should_panic] nt_syntax_bad_uri_05 where path = "nt-syntax-bad-uri-05",
            #[should_panic] nt_syntax_bad_uri_06 where path = "nt-syntax-bad-uri-06",
            #[should_panic] nt_syntax_bad_uri_07 where path = "nt-syntax-bad-uri-07",
            #[should_panic] nt_syntax_bad_uri_08 where path = "nt-syntax-bad-uri-08",
            #[should_panic] nt_syntax_bad_uri_09 where path = "nt-syntax-bad-uri-09",

            nt_syntax_bnode_01 where path = "nt-syntax-bnode-01",
            nt_syntax_bnode_02 where path = "nt-syntax-bnode-02",
            nt_syntax_bnode_03 where path = "nt-syntax-bnode-03",

            nt_syntax_datatypes_01 where path = "nt-syntax-datatypes-01",
            nt_syntax_datatypes_02 where path = "nt-syntax-datatypes-02",

            nt_syntax_file_01 where path = "nt-syntax-file-01",
            nt_syntax_file_02 where path = "nt-syntax-file-02",
            nt_syntax_file_03 where path = "nt-syntax-file-03",

            nt_syntax_str_esc_01 where path = "nt-syntax-str-esc-01",
            nt_syntax_str_esc_02 where path = "nt-syntax-str-esc-02",
            nt_syntax_str_esc_03 where path = "nt-syntax-str-esc-03",

            nt_syntax_string_01 where path = "nt-syntax-string-01",
            nt_syntax_string_02 where path = "nt-syntax-string-02",
            nt_syntax_string_03 where path = "nt-syntax-string-03",

            nt_syntax_subm_01 where path = "nt-syntax-subm-01",

            nt_syntax_uri_01 where path = "nt-syntax-uri-01",
            nt_syntax_uri_02 where path = "nt-syntax-uri-02",
            nt_syntax_uri_03 where path = "nt-syntax-uri-03",
            nt_syntax_uri_04 where path = "nt-syntax-uri-04",
        }
    }
}
