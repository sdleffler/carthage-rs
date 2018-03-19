use std::{fmt, str::CharIndices};

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
    Eol,
    Period,
    DoubleCaret,
    LangTag(&'input str),
    IriRef(&'input str),
    StringLiteral(&'input str),
    BlankNodeLabel(&'input str),
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Eol => writeln!(f, ""),
            Token::Period => writeln!(f, "."),
            Token::DoubleCaret => writeln!(f, "^^"),
            Token::LangTag(string) => writeln!(f, "@{}", string),
            Token::IriRef(string) => writeln!(f, "<{}>", string),
            Token::StringLiteral(string) => writeln!(f, "\"{}\"", string),
            Token::BlankNodeLabel(string) => writeln!(f, "_:{}", string),
        }
    }
}

#[derive(Debug, Clone, Copy, Fail)]
#[fail(display = "lexer error {:?} at byte {:?}", location, kind)]
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
                    self.lookahead = self.char_indices
                        .by_ref()
                        .skip_while(|&(_, c)| c == ' ' || c == '\t')
                        .next();
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
                Some((idx0, '^')) if self.text[idx0..].starts_with("^^") => {
                    Ok((idx0, Token::DoubleCaret, idx0 + 2))
                }
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
                self.lookahead = self.char_indices
                    .by_ref()
                    .skip_while(|&(idx, _)| idx < spanned.2)
                    .next();
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

    use std::{fs::File, path::PathBuf};

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

    macro_rules! lex_test_files {
        (@test $(#[$m:meta])* $test:ident) => {
            lex_test_files!(@test $(#[$m:meta])* $test where path = stringify!($test));
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

                let lexer = Lexer::new(&string, 0);
                let mut tokens_parsed = 0;

                for token_res in lexer {
                    match token_res {
                        Ok(token) => {
                            println!("Parsed token {} as {:?}", tokens_parsed, token);
                            tokens_parsed += 1;
                        },
                        Err(error) => panic!("{} tokens parsed before failure: {:?}", tokens_parsed, error),
                    }
                }
            }
        };
        ($( $test_name:ident { $($(#[$m:meta])* $test:ident $(where $opt:tt = $val:expr)*,)* } )*) => {
            $(
                mod $test_name {
                    use super::*;

                    $(lex_test_files!(@test $(#[$m])* $test $(where $opt = $val)*);)*
                }
            )*
        };
    }

    lex_test_files! {
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

            // Shouldn't panic - will *lex* just fine. Caught during parsing.
            nt_syntax_bad_base_01 where path = "nt-syntax-bad-base-01",

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

            // Shouldn't panic - will *lex* just fine. Caught during parsing.
            nt_syntax_bad_string_05 where path = "nt-syntax-bad-string-05",

            #[should_panic] nt_syntax_bad_string_06 where path = "nt-syntax-bad-string-06",
            #[should_panic] nt_syntax_bad_string_07 where path = "nt-syntax-bad-string-07",
            #[should_panic] nt_syntax_bad_struct_01 where path = "nt-syntax-bad-struct-01",
            #[should_panic] nt_syntax_bad_struct_02 where path = "nt-syntax-bad-struct-02",

            #[should_panic] nt_syntax_bad_uri_01 where path = "nt-syntax-bad-uri-01",
            #[should_panic] nt_syntax_bad_uri_02 where path = "nt-syntax-bad-uri-02",
            #[should_panic] nt_syntax_bad_uri_03 where path = "nt-syntax-bad-uri-03",
            #[should_panic] nt_syntax_bad_uri_04 where path = "nt-syntax-bad-uri-04",
            #[should_panic] nt_syntax_bad_uri_05 where path = "nt-syntax-bad-uri-05",

            // Following bad-uri tests are parse-fails, not lex-fails
            nt_syntax_bad_uri_06 where path = "nt-syntax-bad-uri-06",
            nt_syntax_bad_uri_07 where path = "nt-syntax-bad-uri-07",
            nt_syntax_bad_uri_08 where path = "nt-syntax-bad-uri-08",
            nt_syntax_bad_uri_09 where path = "nt-syntax-bad-uri-09",

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
