use std::str::CharIndices;

use regex::Regex;

const PN_CHARS_BASE_ADDS: &'static str = r#"a-zA-Z\x{00C0}-\x{00D6}\x{00D8}-\x{00F6}\x{00F8}-\x{02FF}\x{0370}-\x{037D}\x{037F}-\x{1FFF}\x{200C}-\x{200D}\x{2070}-\x{218F}\x{2C00}-\x{2FEF}\x{3001}-\x{D7FF}\x{F900}-\x{FDCF}\x{FDF0}-\x{FFFD}\x{10000}-\x{EFFFF}"#;
const PN_CHARS_U_ADDS: &'static str = r"_:";
const PN_CHARS_ADDS: &'static str = r"\-0-9\x{00B7}\x{0300}-\x{036F}\x{203F}-\x{2040}";

lazy_static! {
    static ref PN_CHARS_U: String = format!("{}{}", PN_CHARS_BASE_ADDS, PN_CHARS_U_ADDS);
    static ref PN_CHARS: String = format!("{}{}", *PN_CHARS_U, PN_CHARS_ADDS);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token<'input> {
    LangTag(&'input str),
    Eol,
    Period,
    IriRef(&'input str),
    StringLiteral(&'input str),
    BlankNodeLabel(&'input str),
}

#[derive(Debug, Clone, Copy)]
pub struct Error {
    pub location: usize,
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind, location: usize) -> Self {
        Self { kind, location }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    ExpectedLangTag,
    ExpectedEol,
    ExpectedIriRef,
    ExpectedStringLiteral,
    ExpectedBlankNodeLabel,
    UnrecognizedToken,
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug)]
pub struct Lexer<'input> {
    text: &'input str,
    char_indices: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Self {
        let mut char_indices = text.char_indices();
        let lookahead = char_indices.next();
        Self {
            text,
            char_indices,
            lookahead,
            shift,
        }
    }

    fn lang_tag(&self, idx0: usize) -> Spanned<Token<'input>, usize, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^@[a-zA-Z]+(?:-[a-zA-Z0-9]+)*").unwrap();
        }

        let mat = RE.find(&self.text[idx0..])
            .ok_or(Error::new(ErrorKind::ExpectedLangTag, idx0))?;
        let idx1 = idx0 + mat.end();
        let token = Token::LangTag(&self.text[idx0 + 1..idx1]); // trim '@'
        Ok((idx0, token, idx1))
    }

    fn eol(&self, idx0: usize) -> Spanned<Token<'input>, usize, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[\r\n]+").unwrap();
        }

        let mat = RE.find(&self.text[idx0..])
            .ok_or(Error::new(ErrorKind::ExpectedEol, idx0))?;
        let idx1 = idx0 + mat.end();
        Ok((idx0, Token::Eol, idx1))
    }

    fn iri_ref(&self, idx0: usize) -> Spanned<Token<'input>, usize, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"^<(?:[^\x{00}-\x{20}<>"{}|^`\\]|\\u[[:xdigit:]]{4}|\\U[[:xdigit:]]{8})*>"#).unwrap();
        }

        let mat = RE.find(&self.text[idx0..])
            .ok_or(Error::new(ErrorKind::ExpectedIriRef, idx0))?;
        let idx1 = idx0 + mat.end();
        let token = Token::IriRef(&self.text[idx0 + 1..idx1 - 1]); // trim '<' and '>'
        Ok((idx0, token, idx1))
    }

    fn string_literal(&self, idx0: usize) -> Spanned<Token<'input>, usize, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"^"([^"\\\r\n]|\\[tbrnf"'\\]|\\u[[:xdigit:]]{4}|\\U[[:xdigit:]]{8})*""#).unwrap();
        }

        let mat = RE.find(&self.text[idx0..])
            .ok_or(Error::new(ErrorKind::ExpectedStringLiteral, idx0))?;
        let idx1 = idx0 + mat.end();
        let token = Token::StringLiteral(&self.text[idx0 + 1..idx1 - 1]); // trim paired '\"'
        Ok((idx0, token, idx1))
    }

    fn blank_node_label(&self, idx0: usize) -> Spanned<Token<'input>, usize, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(&format!(r"^_:[{}0-9](?:[{}.]*[{}])?", *PN_CHARS_U, *PN_CHARS, *PN_CHARS)).unwrap();
        }

        let mat = RE.find(&self.text[idx0..])
            .ok_or(Error::new(ErrorKind::ExpectedBlankNodeLabel, idx0))?;
        let idx1 = idx0 + mat.end();
        let token = Token::BlankNodeLabel(&self.text[idx0 + 2..idx1]); // trim '_:'
        Ok((idx0, token, idx1))
    }

    fn next_unshifted(&mut self) -> Option<Spanned<Token<'input>, usize, Error>> {
        loop {
            let spanned_res = match self.lookahead.take() {
                Some((_, c)) if c == ' ' || c == '\t' => {
                    self.lookahead = self.char_indices.next();
                    continue;
                }
                Some((_, '#')) => {
                    self.lookahead = self.char_indices
                        .by_ref()
                        .skip_while(|&(_, c)| c != '\n' && c != '\r')
                        .next();
                    continue;
                }
                Some((idx0, '.')) => Ok((idx0, Token::Period, idx0 + 1)),
                Some((idx0, c)) if c == '\n' || c == '\r' => self.eol(idx0),
                Some((idx0, '@')) => self.lang_tag(idx0),
                Some((idx0, '<')) => self.iri_ref(idx0),
                Some((idx0, '\"')) => self.string_literal(idx0),
                Some((idx0, '_')) => self.blank_node_label(idx0),
                Some((idx0, _)) => Err(Error::new(ErrorKind::UnrecognizedToken, idx0)),
                None => return None,
            };

            if let Ok(ref spanned) = spanned_res {
                assert!(spanned.2 > spanned.0);
                self.lookahead = self.char_indices.by_ref().nth(spanned.2 - spanned.0);
            }

            return Some(spanned_res);
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_unshifted() {
            None => None,
            Some(Ok((l, t, r))) => Some(Ok((l + self.shift, t, r + self.shift))),
            Some(Err(Error { location, kind })) => Some(Err(Error {
                location: location + self.shift,
                kind: kind,
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{ffi::OsStr, fs::File, io::Read, path::{Path, PathBuf}};

    #[test]
    fn lang_tag_0() {
        let lexer = Lexer::new("@en @fr @fr-be", 0);
        assert_eq!(lexer.lang_tag(0).ok(), Some((0, Token::LangTag("en"), 3)));
        assert_eq!(lexer.lang_tag(4).ok(), Some((4, Token::LangTag("fr"), 7)));
        assert_eq!(
            lexer.lang_tag(8).ok(),
            Some((8, Token::LangTag("fr-be"), 14))
        );
    }

    #[test]
    fn eol_0() {
        let lexer = Lexer::new("\n\r \r  \n\n \r\n \n\r\r\n\r\n", 0);
        assert_eq!(lexer.eol(0).ok(), Some((0, Token::Eol, 2)));
        assert_eq!(lexer.eol(3).ok(), Some((3, Token::Eol, 4)));
        assert_eq!(lexer.eol(6).ok(), Some((6, Token::Eol, 8)));
        assert_eq!(lexer.eol(9).ok(), Some((9, Token::Eol, 11)));
        assert_eq!(lexer.eol(12).ok(), Some((12, Token::Eol, 18)));
    }

    #[test]
    fn iri_ref_0() {
        let lexer = Lexer::new("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .", 0);
        assert_eq!(
            lexer.iri_ref(14).ok(),
            Some((
                14,
                Token::IriRef("http://www.w3.org/2000/01/rdf-schema#"),
                53
            ))
        );
    }

    #[test]
    fn string_literal_0() {
        let lexer = Lexer::new("show:218 rdfs:label \"That Seventies Show\"^^<http://www.w3.org/2001/XMLSchema#string> .", 0);
        assert_eq!(
            lexer.string_literal(20).ok(),
            Some((20, Token::StringLiteral("That Seventies Show"), 41))
        );
    }

    #[test]
    fn blank_node_label_0() {
        let lexer = Lexer::new("_:alice <http://xmlns.com/foaf/0.1/knows> _:bob .", 0);
        assert_eq!(
            lexer.blank_node_label(42).ok(),
            Some((42, Token::BlankNodeLabel("bob"), 47))
        );
    }

    fn lex_file(path: &Path) -> Result<usize, Error> {
        let mut file = File::open(path).unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();

        let lexer = Lexer::new(&string, 0);
        lexer.collect::<Result<Vec<_>, _>>().map(|vec| vec.len())
    }

    #[test]
    fn lex_w3c_test_suite() {
        let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ntriples-tests");
        let results = test_dir
            .read_dir()
            .unwrap()
            .filter_map(|res| {
                let filtered_result = res.map(|entry| {
                    let path = entry.path();

                    if path.extension() == Some(OsStr::new("nt")) {
                        let result = lex_file(&path);
                        Some((path, result))
                    } else {
                        None
                    }
                });

                match filtered_result {
                    Ok(Some(t)) => Some(Ok(t)),
                    Ok(None) => None,
                    Err(e) => Some(Err(e)),
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let total = results.len();
        let mut passed = 0;
        let mut failed = 0;

        for &(ref path, ref result) in &results {
            // Simple and silly heuristic: if the filename contains "bad", it's a test intended to
            // fail.
            let should_pass = !path.file_name().unwrap().to_str().unwrap().contains("bad");
            let pass_fail = if result.is_ok() == should_pass {
                passed += 1;
                "PASS"
            } else {
                failed += 1;
                "FAIL"
            };

            print!("[{}] {}", pass_fail, path.display());
            match *result {
                Ok(n_tokens) => print!(", {} tokens parsed", n_tokens),
                Err(error) => print!("\n\t{:?}", error),
            }
            println!();
        }

        println!("{} files lexed, {} passed, {} failed", total, passed, failed);

        assert!(results.iter().all(|r| r.1.is_ok()));
    }
}
