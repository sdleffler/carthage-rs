// auto-generated: "lalrpop 0.14.0"
use ntriples::ast::{Document, Triple, Subject, Object, Literal, Datatype};
use ntriples::lexer::{self, Token};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Datatype {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ntriples::ast::{Document, Triple, Subject, Object, Literal, Datatype};
    use ntriples::lexer::{self, Token};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_2e_22(Token<'input>),
        Term_22BLANK__NODE__LABEL_22(&'input str),
        Term_22EOL_22(Token<'input>),
        Term_22IRIREF_22(&'input str),
        Term_22LANGTAG_22(&'input str),
        Term_22STRING__LITERAL__QUOTE_22(&'input str),
        Term_22_5e_5e_22(Token<'input>),
        Nt_22EOL_22_3f(::std::option::Option<Token<'input>>),
        Nt_28_22EOL_22_20Triple_29((Token<'input>, Triple<'input>)),
        Nt_28_22EOL_22_20Triple_29_2a(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        Nt_28_22EOL_22_20Triple_29_2b(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        NtDatatype(Datatype<'input>),
        NtDatatype_3f(::std::option::Option<Datatype<'input>>),
        NtDocument(Document<'input>),
        NtLiteral(Literal<'input>),
        NtObject(Object<'input>),
        NtSubject(Subject<'input>),
        NtTriple(Triple<'input>),
        NtTriple_3f(::std::option::Option<Triple<'input>>),
        Nt____Datatype(Datatype<'input>),
        Nt____Document(Document<'input>),
        Nt____Literal(Literal<'input>),
        Nt____Object(Object<'input>),
        Nt____Subject(Subject<'input>),
        Nt____Triple(Triple<'input>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 3, 0, 4,
        // State 1
        0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 5, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        -30,
        // State 2
        -9,
        // State 3
        0,
        // State 4
        -8,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"".""###,
            r###""BLANK_NODE_LABEL""###,
            r###""EOL""###,
            r###""IRIREF""###,
            r###""LANGTAG""###,
            r###""STRING_LITERAL_QUOTE""###,
            r###""^^""###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Datatype<
        'input,
        __TOKEN: __ToTriple<'input, Error=lexer::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        text: &'input str,
        __tokens0: __TOKENS,
    ) -> Result<Datatype<'input>, __lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let __last_location = &mut Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            *__last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::Period if true => 0,
                Token::BlankNodeLabel(_) if true => 1,
                Token::Eol if true => 2,
                Token::IriRef(_) if true => 3,
                Token::LangTag(_) if true => 4,
                Token::StringLiteral(_) if true => 5,
                Token::DoubleCaret if true => 6,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::Period => __Symbol::Term_22_2e_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token::BlankNodeLabel(__tok0) => __Symbol::Term_22BLANK__NODE__LABEL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Eol => __Symbol::Term_22EOL_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token::IriRef(__tok0) => __Symbol::Term_22IRIREF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token::LangTag(__tok0) => __Symbol::Term_22LANGTAG_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token::StringLiteral(__tok0) => __Symbol::Term_22STRING__LITERAL__QUOTE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::DoubleCaret => __Symbol::Term_22_5e_5e_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(text, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        if r.is_err() {
                            return r;
                        }
                        return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                    }
                } else {
                    let mut __err_lookahead = Some(__lookahead);
                    let mut __err_integer: Option<usize> = Some(__integer);
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(text, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let mut __err_lookahead = None;
                let mut __err_integer: Option<usize> = None;
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: __err_lookahead,
                    expected: __expected_tokens(__state),
                };
                return Err(__error)
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        text: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Datatype<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"? = "EOL" => ActionFn(18);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            2 => {
                // "EOL"? =  => ActionFn(19);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action19::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            3 => {
                // ("EOL" Triple) = "EOL", Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29(__nt), __end));
                1
            }
            4 => {
                // ("EOL" Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            5 => {
                // ("EOL" Triple)* = ("EOL" Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            6 => {
                // ("EOL" Triple)+ = "EOL", Triple => ActionFn(29);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            7 => {
                // ("EOL" Triple)+ = ("EOL" Triple)+, "EOL", Triple => ActionFn(30);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            8 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            9 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            10 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            11 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            12 => {
                // Document = Triple, "EOL" => ActionFn(37);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            13 => {
                // Document = "EOL" => ActionFn(38);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            14 => {
                // Document = Triple, ("EOL" Triple)+, "EOL" => ActionFn(39);
                let __sym2 = __pop_Term_22EOL_22(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            15 => {
                // Document = ("EOL" Triple)+, "EOL" => ActionFn(40);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action40::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            16 => {
                // Document = Triple => ActionFn(41);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            17 => {
                // Document =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            18 => {
                // Document = Triple, ("EOL" Triple)+ => ActionFn(43);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            19 => {
                // Document = ("EOL" Triple)+ => ActionFn(44);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            20 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(35);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            21 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(36);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            22 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            23 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            24 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            25 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            26 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            27 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                10
            }
            28 => {
                // Triple? = Triple => ActionFn(23);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            29 => {
                // Triple? =  => ActionFn(24);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action24::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            30 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(text, __sym0);
                return Some(Ok(__nt));
            }
            31 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                13
            }
            32 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                14
            }
            33 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                15
            }
            34 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                16
            }
            35 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22BLANK__NODE__LABEL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22BLANK__NODE__LABEL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22EOL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22EOL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22IRIREF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22IRIREF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LANGTAG_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LANGTAG_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STRING__LITERAL__QUOTE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STRING__LITERAL__QUOTE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Token<'input>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Datatype<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDocument<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDocument(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteral<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtObject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtObject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSubject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSubject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Triple<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Datatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Datatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Document<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Document(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Literal<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Literal(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Object<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Object(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Subject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Subject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Triple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Triple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Datatype::parse_Datatype;

mod __parse__Document {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ntriples::ast::{Document, Triple, Subject, Object, Literal, Datatype};
    use ntriples::lexer::{self, Token};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_2e_22(Token<'input>),
        Term_22BLANK__NODE__LABEL_22(&'input str),
        Term_22EOL_22(Token<'input>),
        Term_22IRIREF_22(&'input str),
        Term_22LANGTAG_22(&'input str),
        Term_22STRING__LITERAL__QUOTE_22(&'input str),
        Term_22_5e_5e_22(Token<'input>),
        Nt_22EOL_22_3f(::std::option::Option<Token<'input>>),
        Nt_28_22EOL_22_20Triple_29((Token<'input>, Triple<'input>)),
        Nt_28_22EOL_22_20Triple_29_2a(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        Nt_28_22EOL_22_20Triple_29_2b(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        NtDatatype(Datatype<'input>),
        NtDatatype_3f(::std::option::Option<Datatype<'input>>),
        NtDocument(Document<'input>),
        NtLiteral(Literal<'input>),
        NtObject(Object<'input>),
        NtSubject(Subject<'input>),
        NtTriple(Triple<'input>),
        NtTriple_3f(::std::option::Option<Triple<'input>>),
        Nt____Datatype(Datatype<'input>),
        Nt____Document(Document<'input>),
        Nt____Literal(Literal<'input>),
        Nt____Object(Object<'input>),
        Nt____Subject(Subject<'input>),
        Nt____Triple(Triple<'input>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 6, 7, 8, 0, 0, 0,
        // State 1
        0, 0, 9, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 10, 0, 0, 0,
        // State 4
        0, 0, 12, 0, 0, 0, 0,
        // State 5
        0, 0, 0, -26, 0, 0, 0,
        // State 6
        0, 6, 0, 8, 0, 0, 0,
        // State 7
        0, 0, 0, -25, 0, 0, 0,
        // State 8
        0, 6, 0, 8, 0, 0, 0,
        // State 9
        0, 17, 0, 18, 0, 19, 0,
        // State 10
        0, 0, 20, 0, 0, 0, 0,
        // State 11
        0, 6, 0, 8, 0, 0, 0,
        // State 12
        0, 0, -6, 0, 0, 0, 0,
        // State 13
        0, 0, -7, 0, 0, 0, 0,
        // State 14
        -24, 0, 0, 0, 0, 0, 0,
        // State 15
        21, 0, 0, 0, 0, 0, 0,
        // State 16
        -23, 0, 0, 0, 0, 0, 0,
        // State 17
        -22, 0, 0, 0, 0, 0, 0,
        // State 18
        -21, 0, 0, 0, 23, 0, 24,
        // State 19
        0, 6, 0, 8, 0, 0, 0,
        // State 20
        0, 0, -27, 0, 0, 0, 0,
        // State 21
        -20, 0, 0, 0, 0, 0, 0,
        // State 22
        -9, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 25, 0, 0, 0,
        // State 24
        -8, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        -17,
        // State 1
        -19,
        // State 2
        -31,
        // State 3
        0,
        // State 4
        -16,
        // State 5
        0,
        // State 6
        -13,
        // State 7
        0,
        // State 8
        -15,
        // State 9
        0,
        // State 10
        -18,
        // State 11
        -12,
        // State 12
        -6,
        // State 13
        -7,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        -14,
        // State 20
        -27,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 2, 0, 0, 3, 0, 0, 4, 5, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 13, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 13, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"".""###,
            r###""BLANK_NODE_LABEL""###,
            r###""EOL""###,
            r###""IRIREF""###,
            r###""LANGTAG""###,
            r###""STRING_LITERAL_QUOTE""###,
            r###""^^""###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Document<
        'input,
        __TOKEN: __ToTriple<'input, Error=lexer::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        text: &'input str,
        __tokens0: __TOKENS,
    ) -> Result<Document<'input>, __lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let __last_location = &mut Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            *__last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::Period if true => 0,
                Token::BlankNodeLabel(_) if true => 1,
                Token::Eol if true => 2,
                Token::IriRef(_) if true => 3,
                Token::LangTag(_) if true => 4,
                Token::StringLiteral(_) if true => 5,
                Token::DoubleCaret if true => 6,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::Period => __Symbol::Term_22_2e_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token::BlankNodeLabel(__tok0) => __Symbol::Term_22BLANK__NODE__LABEL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Eol => __Symbol::Term_22EOL_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token::IriRef(__tok0) => __Symbol::Term_22IRIREF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token::LangTag(__tok0) => __Symbol::Term_22LANGTAG_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token::StringLiteral(__tok0) => __Symbol::Term_22STRING__LITERAL__QUOTE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::DoubleCaret => __Symbol::Term_22_5e_5e_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(text, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        if r.is_err() {
                            return r;
                        }
                        return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                    }
                } else {
                    let mut __err_lookahead = Some(__lookahead);
                    let mut __err_integer: Option<usize> = Some(__integer);
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(text, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let mut __err_lookahead = None;
                let mut __err_integer: Option<usize> = None;
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: __err_lookahead,
                    expected: __expected_tokens(__state),
                };
                return Err(__error)
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        text: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Document<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"? = "EOL" => ActionFn(18);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            2 => {
                // "EOL"? =  => ActionFn(19);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action19::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            3 => {
                // ("EOL" Triple) = "EOL", Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29(__nt), __end));
                1
            }
            4 => {
                // ("EOL" Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            5 => {
                // ("EOL" Triple)* = ("EOL" Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            6 => {
                // ("EOL" Triple)+ = "EOL", Triple => ActionFn(29);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            7 => {
                // ("EOL" Triple)+ = ("EOL" Triple)+, "EOL", Triple => ActionFn(30);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            8 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            9 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            10 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            11 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            12 => {
                // Document = Triple, "EOL" => ActionFn(37);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            13 => {
                // Document = "EOL" => ActionFn(38);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            14 => {
                // Document = Triple, ("EOL" Triple)+, "EOL" => ActionFn(39);
                let __sym2 = __pop_Term_22EOL_22(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            15 => {
                // Document = ("EOL" Triple)+, "EOL" => ActionFn(40);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action40::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            16 => {
                // Document = Triple => ActionFn(41);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            17 => {
                // Document =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            18 => {
                // Document = Triple, ("EOL" Triple)+ => ActionFn(43);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            19 => {
                // Document = ("EOL" Triple)+ => ActionFn(44);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            20 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(35);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            21 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(36);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            22 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            23 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            24 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            25 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            26 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            27 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                10
            }
            28 => {
                // Triple? = Triple => ActionFn(23);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            29 => {
                // Triple? =  => ActionFn(24);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action24::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            30 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                12
            }
            31 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(text, __sym0);
                return Some(Ok(__nt));
            }
            32 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                14
            }
            33 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                15
            }
            34 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                16
            }
            35 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22BLANK__NODE__LABEL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22BLANK__NODE__LABEL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22EOL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22EOL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22IRIREF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22IRIREF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LANGTAG_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LANGTAG_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STRING__LITERAL__QUOTE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STRING__LITERAL__QUOTE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Token<'input>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Datatype<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDocument<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDocument(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteral<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtObject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtObject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSubject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSubject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Triple<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Datatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Datatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Document<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Document(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Literal<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Literal(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Object<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Object(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Subject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Subject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Triple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Triple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Document::parse_Document;

mod __parse__Literal {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ntriples::ast::{Document, Triple, Subject, Object, Literal, Datatype};
    use ntriples::lexer::{self, Token};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_2e_22(Token<'input>),
        Term_22BLANK__NODE__LABEL_22(&'input str),
        Term_22EOL_22(Token<'input>),
        Term_22IRIREF_22(&'input str),
        Term_22LANGTAG_22(&'input str),
        Term_22STRING__LITERAL__QUOTE_22(&'input str),
        Term_22_5e_5e_22(Token<'input>),
        Nt_22EOL_22_3f(::std::option::Option<Token<'input>>),
        Nt_28_22EOL_22_20Triple_29((Token<'input>, Triple<'input>)),
        Nt_28_22EOL_22_20Triple_29_2a(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        Nt_28_22EOL_22_20Triple_29_2b(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        NtDatatype(Datatype<'input>),
        NtDatatype_3f(::std::option::Option<Datatype<'input>>),
        NtDocument(Document<'input>),
        NtLiteral(Literal<'input>),
        NtObject(Object<'input>),
        NtSubject(Subject<'input>),
        NtTriple(Triple<'input>),
        NtTriple_3f(::std::option::Option<Triple<'input>>),
        Nt____Datatype(Datatype<'input>),
        Nt____Document(Document<'input>),
        Nt____Literal(Literal<'input>),
        Nt____Object(Object<'input>),
        Nt____Subject(Subject<'input>),
        Nt____Triple(Triple<'input>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 3, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 5, 0, 6,
        // State 3
        0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 7, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        -32,
        // State 2
        -21,
        // State 3
        -20,
        // State 4
        -9,
        // State 5
        0,
        // State 6
        -8,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"".""###,
            r###""BLANK_NODE_LABEL""###,
            r###""EOL""###,
            r###""IRIREF""###,
            r###""LANGTAG""###,
            r###""STRING_LITERAL_QUOTE""###,
            r###""^^""###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Literal<
        'input,
        __TOKEN: __ToTriple<'input, Error=lexer::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        text: &'input str,
        __tokens0: __TOKENS,
    ) -> Result<Literal<'input>, __lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let __last_location = &mut Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            *__last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::Period if true => 0,
                Token::BlankNodeLabel(_) if true => 1,
                Token::Eol if true => 2,
                Token::IriRef(_) if true => 3,
                Token::LangTag(_) if true => 4,
                Token::StringLiteral(_) if true => 5,
                Token::DoubleCaret if true => 6,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::Period => __Symbol::Term_22_2e_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token::BlankNodeLabel(__tok0) => __Symbol::Term_22BLANK__NODE__LABEL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Eol => __Symbol::Term_22EOL_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token::IriRef(__tok0) => __Symbol::Term_22IRIREF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token::LangTag(__tok0) => __Symbol::Term_22LANGTAG_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token::StringLiteral(__tok0) => __Symbol::Term_22STRING__LITERAL__QUOTE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::DoubleCaret => __Symbol::Term_22_5e_5e_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(text, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        if r.is_err() {
                            return r;
                        }
                        return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                    }
                } else {
                    let mut __err_lookahead = Some(__lookahead);
                    let mut __err_integer: Option<usize> = Some(__integer);
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(text, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let mut __err_lookahead = None;
                let mut __err_integer: Option<usize> = None;
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: __err_lookahead,
                    expected: __expected_tokens(__state),
                };
                return Err(__error)
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        text: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Literal<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"? = "EOL" => ActionFn(18);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            2 => {
                // "EOL"? =  => ActionFn(19);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action19::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            3 => {
                // ("EOL" Triple) = "EOL", Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29(__nt), __end));
                1
            }
            4 => {
                // ("EOL" Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            5 => {
                // ("EOL" Triple)* = ("EOL" Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            6 => {
                // ("EOL" Triple)+ = "EOL", Triple => ActionFn(29);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            7 => {
                // ("EOL" Triple)+ = ("EOL" Triple)+, "EOL", Triple => ActionFn(30);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            8 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            9 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            10 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            11 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            12 => {
                // Document = Triple, "EOL" => ActionFn(37);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            13 => {
                // Document = "EOL" => ActionFn(38);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            14 => {
                // Document = Triple, ("EOL" Triple)+, "EOL" => ActionFn(39);
                let __sym2 = __pop_Term_22EOL_22(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            15 => {
                // Document = ("EOL" Triple)+, "EOL" => ActionFn(40);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action40::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            16 => {
                // Document = Triple => ActionFn(41);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            17 => {
                // Document =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            18 => {
                // Document = Triple, ("EOL" Triple)+ => ActionFn(43);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            19 => {
                // Document = ("EOL" Triple)+ => ActionFn(44);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            20 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(35);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            21 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(36);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            22 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            23 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            24 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            25 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            26 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            27 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                10
            }
            28 => {
                // Triple? = Triple => ActionFn(23);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            29 => {
                // Triple? =  => ActionFn(24);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action24::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            30 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                12
            }
            31 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                13
            }
            32 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(text, __sym0);
                return Some(Ok(__nt));
            }
            33 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                15
            }
            34 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                16
            }
            35 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22BLANK__NODE__LABEL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22BLANK__NODE__LABEL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22EOL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22EOL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22IRIREF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22IRIREF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LANGTAG_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LANGTAG_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STRING__LITERAL__QUOTE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STRING__LITERAL__QUOTE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Token<'input>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Datatype<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDocument<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDocument(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteral<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtObject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtObject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSubject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSubject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Triple<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Datatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Datatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Document<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Document(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Literal<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Literal(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Object<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Object(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Subject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Subject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Triple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Triple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Literal::parse_Literal;

mod __parse__Object {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ntriples::ast::{Document, Triple, Subject, Object, Literal, Datatype};
    use ntriples::lexer::{self, Token};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_2e_22(Token<'input>),
        Term_22BLANK__NODE__LABEL_22(&'input str),
        Term_22EOL_22(Token<'input>),
        Term_22IRIREF_22(&'input str),
        Term_22LANGTAG_22(&'input str),
        Term_22STRING__LITERAL__QUOTE_22(&'input str),
        Term_22_5e_5e_22(Token<'input>),
        Nt_22EOL_22_3f(::std::option::Option<Token<'input>>),
        Nt_28_22EOL_22_20Triple_29((Token<'input>, Triple<'input>)),
        Nt_28_22EOL_22_20Triple_29_2a(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        Nt_28_22EOL_22_20Triple_29_2b(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        NtDatatype(Datatype<'input>),
        NtDatatype_3f(::std::option::Option<Datatype<'input>>),
        NtDocument(Document<'input>),
        NtLiteral(Literal<'input>),
        NtObject(Object<'input>),
        NtSubject(Subject<'input>),
        NtTriple(Triple<'input>),
        NtTriple_3f(::std::option::Option<Triple<'input>>),
        Nt____Datatype(Datatype<'input>),
        Nt____Document(Document<'input>),
        Nt____Literal(Literal<'input>),
        Nt____Object(Object<'input>),
        Nt____Subject(Subject<'input>),
        Nt____Triple(Triple<'input>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 4, 0, 5, 0, 6, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 8, 0, 9,
        // State 6
        0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 10, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        -24,
        // State 2
        -33,
        // State 3
        -23,
        // State 4
        -22,
        // State 5
        -21,
        // State 6
        -20,
        // State 7
        -9,
        // State 8
        0,
        // State 9
        -8,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"".""###,
            r###""BLANK_NODE_LABEL""###,
            r###""EOL""###,
            r###""IRIREF""###,
            r###""LANGTAG""###,
            r###""STRING_LITERAL_QUOTE""###,
            r###""^^""###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Object<
        'input,
        __TOKEN: __ToTriple<'input, Error=lexer::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        text: &'input str,
        __tokens0: __TOKENS,
    ) -> Result<Object<'input>, __lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let __last_location = &mut Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            *__last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::Period if true => 0,
                Token::BlankNodeLabel(_) if true => 1,
                Token::Eol if true => 2,
                Token::IriRef(_) if true => 3,
                Token::LangTag(_) if true => 4,
                Token::StringLiteral(_) if true => 5,
                Token::DoubleCaret if true => 6,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::Period => __Symbol::Term_22_2e_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token::BlankNodeLabel(__tok0) => __Symbol::Term_22BLANK__NODE__LABEL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Eol => __Symbol::Term_22EOL_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token::IriRef(__tok0) => __Symbol::Term_22IRIREF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token::LangTag(__tok0) => __Symbol::Term_22LANGTAG_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token::StringLiteral(__tok0) => __Symbol::Term_22STRING__LITERAL__QUOTE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::DoubleCaret => __Symbol::Term_22_5e_5e_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(text, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        if r.is_err() {
                            return r;
                        }
                        return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                    }
                } else {
                    let mut __err_lookahead = Some(__lookahead);
                    let mut __err_integer: Option<usize> = Some(__integer);
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(text, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let mut __err_lookahead = None;
                let mut __err_integer: Option<usize> = None;
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: __err_lookahead,
                    expected: __expected_tokens(__state),
                };
                return Err(__error)
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        text: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Object<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"? = "EOL" => ActionFn(18);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            2 => {
                // "EOL"? =  => ActionFn(19);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action19::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            3 => {
                // ("EOL" Triple) = "EOL", Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29(__nt), __end));
                1
            }
            4 => {
                // ("EOL" Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            5 => {
                // ("EOL" Triple)* = ("EOL" Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            6 => {
                // ("EOL" Triple)+ = "EOL", Triple => ActionFn(29);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            7 => {
                // ("EOL" Triple)+ = ("EOL" Triple)+, "EOL", Triple => ActionFn(30);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            8 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            9 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            10 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            11 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            12 => {
                // Document = Triple, "EOL" => ActionFn(37);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            13 => {
                // Document = "EOL" => ActionFn(38);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            14 => {
                // Document = Triple, ("EOL" Triple)+, "EOL" => ActionFn(39);
                let __sym2 = __pop_Term_22EOL_22(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            15 => {
                // Document = ("EOL" Triple)+, "EOL" => ActionFn(40);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action40::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            16 => {
                // Document = Triple => ActionFn(41);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            17 => {
                // Document =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            18 => {
                // Document = Triple, ("EOL" Triple)+ => ActionFn(43);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            19 => {
                // Document = ("EOL" Triple)+ => ActionFn(44);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            20 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(35);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            21 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(36);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            22 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            23 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            24 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            25 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            26 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            27 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                10
            }
            28 => {
                // Triple? = Triple => ActionFn(23);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            29 => {
                // Triple? =  => ActionFn(24);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action24::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            30 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                12
            }
            31 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                13
            }
            32 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                14
            }
            33 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(text, __sym0);
                return Some(Ok(__nt));
            }
            34 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                16
            }
            35 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22BLANK__NODE__LABEL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22BLANK__NODE__LABEL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22EOL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22EOL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22IRIREF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22IRIREF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LANGTAG_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LANGTAG_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STRING__LITERAL__QUOTE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STRING__LITERAL__QUOTE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Token<'input>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Datatype<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDocument<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDocument(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteral<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtObject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtObject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSubject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSubject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Triple<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Datatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Datatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Document<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Document(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Literal<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Literal(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Object<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Object(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Subject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Subject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Triple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Triple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Object::parse_Object;

mod __parse__Subject {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ntriples::ast::{Document, Triple, Subject, Object, Literal, Datatype};
    use ntriples::lexer::{self, Token};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_2e_22(Token<'input>),
        Term_22BLANK__NODE__LABEL_22(&'input str),
        Term_22EOL_22(Token<'input>),
        Term_22IRIREF_22(&'input str),
        Term_22LANGTAG_22(&'input str),
        Term_22STRING__LITERAL__QUOTE_22(&'input str),
        Term_22_5e_5e_22(Token<'input>),
        Nt_22EOL_22_3f(::std::option::Option<Token<'input>>),
        Nt_28_22EOL_22_20Triple_29((Token<'input>, Triple<'input>)),
        Nt_28_22EOL_22_20Triple_29_2a(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        Nt_28_22EOL_22_20Triple_29_2b(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        NtDatatype(Datatype<'input>),
        NtDatatype_3f(::std::option::Option<Datatype<'input>>),
        NtDocument(Document<'input>),
        NtLiteral(Literal<'input>),
        NtObject(Object<'input>),
        NtSubject(Subject<'input>),
        NtTriple(Triple<'input>),
        NtTriple_3f(::std::option::Option<Triple<'input>>),
        Nt____Datatype(Datatype<'input>),
        Nt____Document(Document<'input>),
        Nt____Literal(Literal<'input>),
        Nt____Object(Object<'input>),
        Nt____Subject(Subject<'input>),
        Nt____Triple(Triple<'input>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 3, 0, 4, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        -34,
        // State 2
        -26,
        // State 3
        -25,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"".""###,
            r###""BLANK_NODE_LABEL""###,
            r###""EOL""###,
            r###""IRIREF""###,
            r###""LANGTAG""###,
            r###""STRING_LITERAL_QUOTE""###,
            r###""^^""###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Subject<
        'input,
        __TOKEN: __ToTriple<'input, Error=lexer::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        text: &'input str,
        __tokens0: __TOKENS,
    ) -> Result<Subject<'input>, __lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let __last_location = &mut Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            *__last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::Period if true => 0,
                Token::BlankNodeLabel(_) if true => 1,
                Token::Eol if true => 2,
                Token::IriRef(_) if true => 3,
                Token::LangTag(_) if true => 4,
                Token::StringLiteral(_) if true => 5,
                Token::DoubleCaret if true => 6,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::Period => __Symbol::Term_22_2e_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token::BlankNodeLabel(__tok0) => __Symbol::Term_22BLANK__NODE__LABEL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Eol => __Symbol::Term_22EOL_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token::IriRef(__tok0) => __Symbol::Term_22IRIREF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token::LangTag(__tok0) => __Symbol::Term_22LANGTAG_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token::StringLiteral(__tok0) => __Symbol::Term_22STRING__LITERAL__QUOTE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::DoubleCaret => __Symbol::Term_22_5e_5e_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(text, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        if r.is_err() {
                            return r;
                        }
                        return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                    }
                } else {
                    let mut __err_lookahead = Some(__lookahead);
                    let mut __err_integer: Option<usize> = Some(__integer);
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(text, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let mut __err_lookahead = None;
                let mut __err_integer: Option<usize> = None;
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: __err_lookahead,
                    expected: __expected_tokens(__state),
                };
                return Err(__error)
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        text: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Subject<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"? = "EOL" => ActionFn(18);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            2 => {
                // "EOL"? =  => ActionFn(19);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action19::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            3 => {
                // ("EOL" Triple) = "EOL", Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29(__nt), __end));
                1
            }
            4 => {
                // ("EOL" Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            5 => {
                // ("EOL" Triple)* = ("EOL" Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            6 => {
                // ("EOL" Triple)+ = "EOL", Triple => ActionFn(29);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            7 => {
                // ("EOL" Triple)+ = ("EOL" Triple)+, "EOL", Triple => ActionFn(30);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            8 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            9 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            10 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            11 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            12 => {
                // Document = Triple, "EOL" => ActionFn(37);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            13 => {
                // Document = "EOL" => ActionFn(38);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            14 => {
                // Document = Triple, ("EOL" Triple)+, "EOL" => ActionFn(39);
                let __sym2 = __pop_Term_22EOL_22(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            15 => {
                // Document = ("EOL" Triple)+, "EOL" => ActionFn(40);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action40::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            16 => {
                // Document = Triple => ActionFn(41);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            17 => {
                // Document =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            18 => {
                // Document = Triple, ("EOL" Triple)+ => ActionFn(43);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            19 => {
                // Document = ("EOL" Triple)+ => ActionFn(44);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            20 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(35);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            21 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(36);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            22 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            23 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            24 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            25 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            26 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            27 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                10
            }
            28 => {
                // Triple? = Triple => ActionFn(23);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            29 => {
                // Triple? =  => ActionFn(24);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action24::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            30 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                12
            }
            31 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                13
            }
            32 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                14
            }
            33 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                15
            }
            34 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(text, __sym0);
                return Some(Ok(__nt));
            }
            35 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                17
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22BLANK__NODE__LABEL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22BLANK__NODE__LABEL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22EOL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22EOL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22IRIREF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22IRIREF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LANGTAG_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LANGTAG_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STRING__LITERAL__QUOTE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STRING__LITERAL__QUOTE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Token<'input>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Datatype<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDocument<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDocument(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteral<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtObject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtObject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSubject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSubject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Triple<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Datatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Datatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Document<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Document(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Literal<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Literal(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Object<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Object(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Subject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Subject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Triple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Triple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Subject::parse_Subject;

mod __parse__Triple {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use ntriples::ast::{Document, Triple, Subject, Object, Literal, Datatype};
    use ntriples::lexer::{self, Token};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_2e_22(Token<'input>),
        Term_22BLANK__NODE__LABEL_22(&'input str),
        Term_22EOL_22(Token<'input>),
        Term_22IRIREF_22(&'input str),
        Term_22LANGTAG_22(&'input str),
        Term_22STRING__LITERAL__QUOTE_22(&'input str),
        Term_22_5e_5e_22(Token<'input>),
        Nt_22EOL_22_3f(::std::option::Option<Token<'input>>),
        Nt_28_22EOL_22_20Triple_29((Token<'input>, Triple<'input>)),
        Nt_28_22EOL_22_20Triple_29_2a(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        Nt_28_22EOL_22_20Triple_29_2b(::std::vec::Vec<(Token<'input>, Triple<'input>)>),
        NtDatatype(Datatype<'input>),
        NtDatatype_3f(::std::option::Option<Datatype<'input>>),
        NtDocument(Document<'input>),
        NtLiteral(Literal<'input>),
        NtObject(Object<'input>),
        NtSubject(Subject<'input>),
        NtTriple(Triple<'input>),
        NtTriple_3f(::std::option::Option<Triple<'input>>),
        Nt____Datatype(Datatype<'input>),
        Nt____Document(Document<'input>),
        Nt____Literal(Literal<'input>),
        Nt____Object(Object<'input>),
        Nt____Subject(Subject<'input>),
        Nt____Triple(Triple<'input>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 4, 0, 5, 0, 0, 0,
        // State 1
        0, 0, 0, 6, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, -26, 0, 0, 0,
        // State 4
        0, 0, 0, -25, 0, 0, 0,
        // State 5
        0, 9, 0, 10, 0, 11, 0,
        // State 6
        -24, 0, 0, 0, 0, 0, 0,
        // State 7
        12, 0, 0, 0, 0, 0, 0,
        // State 8
        -23, 0, 0, 0, 0, 0, 0,
        // State 9
        -22, 0, 0, 0, 0, 0, 0,
        // State 10
        -21, 0, 0, 0, 14, 0, 15,
        // State 11
        0, 0, 0, 0, 0, 0, 0,
        // State 12
        -20, 0, 0, 0, 0, 0, 0,
        // State 13
        -9, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 16, 0, 0, 0,
        // State 15
        -8, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -35,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -27,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"".""###,
            r###""BLANK_NODE_LABEL""###,
            r###""EOL""###,
            r###""IRIREF""###,
            r###""LANGTAG""###,
            r###""STRING_LITERAL_QUOTE""###,
            r###""^^""###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Triple<
        'input,
        __TOKEN: __ToTriple<'input, Error=lexer::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        text: &'input str,
        __tokens0: __TOKENS,
    ) -> Result<Triple<'input>, __lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let __last_location = &mut Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            *__last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::Period if true => 0,
                Token::BlankNodeLabel(_) if true => 1,
                Token::Eol if true => 2,
                Token::IriRef(_) if true => 3,
                Token::LangTag(_) if true => 4,
                Token::StringLiteral(_) if true => 5,
                Token::DoubleCaret if true => 6,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::Period => __Symbol::Term_22_2e_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token::BlankNodeLabel(__tok0) => __Symbol::Term_22BLANK__NODE__LABEL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Eol => __Symbol::Term_22EOL_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token::IriRef(__tok0) => __Symbol::Term_22IRIREF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token::LangTag(__tok0) => __Symbol::Term_22LANGTAG_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token::StringLiteral(__tok0) => __Symbol::Term_22STRING__LITERAL__QUOTE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::DoubleCaret => __Symbol::Term_22_5e_5e_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(text, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        if r.is_err() {
                            return r;
                        }
                        return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                    }
                } else {
                    let mut __err_lookahead = Some(__lookahead);
                    let mut __err_integer: Option<usize> = Some(__integer);
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(text, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let mut __err_lookahead = None;
                let mut __err_integer: Option<usize> = None;
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: __err_lookahead,
                    expected: __expected_tokens(__state),
                };
                return Err(__error)
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        text: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Triple<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"? = "EOL" => ActionFn(18);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            2 => {
                // "EOL"? =  => ActionFn(19);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action19::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_3f(__nt), __end));
                0
            }
            3 => {
                // ("EOL" Triple) = "EOL", Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29(__nt), __end));
                1
            }
            4 => {
                // ("EOL" Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            5 => {
                // ("EOL" Triple)* = ("EOL" Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__nt), __end));
                2
            }
            6 => {
                // ("EOL" Triple)+ = "EOL", Triple => ActionFn(29);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            7 => {
                // ("EOL" Triple)+ = ("EOL" Triple)+, "EOL", Triple => ActionFn(30);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__nt), __end));
                3
            }
            8 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            9 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                4
            }
            10 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            11 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                5
            }
            12 => {
                // Document = Triple, "EOL" => ActionFn(37);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            13 => {
                // Document = "EOL" => ActionFn(38);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            14 => {
                // Document = Triple, ("EOL" Triple)+, "EOL" => ActionFn(39);
                let __sym2 = __pop_Term_22EOL_22(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            15 => {
                // Document = ("EOL" Triple)+, "EOL" => ActionFn(40);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action40::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            16 => {
                // Document = Triple => ActionFn(41);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            17 => {
                // Document =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            18 => {
                // Document = Triple, ("EOL" Triple)+ => ActionFn(43);
                let __sym1 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            19 => {
                // Document = ("EOL" Triple)+ => ActionFn(44);
                let __sym0 = __pop_Nt_28_22EOL_22_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                6
            }
            20 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(35);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            21 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(36);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                7
            }
            22 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            23 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            24 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                8
            }
            25 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            26 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                9
            }
            27 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                10
            }
            28 => {
                // Triple? = Triple => ActionFn(23);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            29 => {
                // Triple? =  => ActionFn(24);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action24::<>(text, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                11
            }
            30 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                12
            }
            31 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                13
            }
            32 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                14
            }
            33 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                15
            }
            34 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                16
            }
            35 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(text, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22BLANK__NODE__LABEL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22BLANK__NODE__LABEL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22EOL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22EOL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22IRIREF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22IRIREF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LANGTAG_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LANGTAG_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STRING__LITERAL__QUOTE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STRING__LITERAL__QUOTE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Token<'input>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_20Triple_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDatatype_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Datatype<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDatatype_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDocument<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDocument(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteral<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtObject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtObject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSubject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSubject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTriple_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Triple<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTriple_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Datatype<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Datatype<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Datatype(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Document<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Document<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Document(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Literal<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Literal(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Object<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Object<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Object(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Subject<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Subject<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Subject(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Triple<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Triple<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Triple(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Triple::parse_Triple;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Document<'input>, usize),
) -> Document<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Triple<'input>, usize),
) -> Triple<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Subject<'input>, usize),
) -> Subject<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Object<'input>, usize),
) -> Object<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Literal<'input>, usize),
) -> Literal<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Datatype<'input>, usize),
) -> Datatype<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    text: &'input str,
    (_, head, _): (usize, ::std::option::Option<Triple<'input>>, usize),
    (_, body, _): (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
    (_, _, _): (usize, ::std::option::Option<Token<'input>>, usize),
) -> Document<'input>
{
    Document { triples: vec![] }
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    text: &'input str,
    (_, subject, _): (usize, Subject<'input>, usize),
    (_, predicate, _): (usize, &'input str, usize),
    (_, object, _): (usize, Object<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Triple<'input>
{
    Triple {subject:subject, predicate:predicate, object:object}
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Subject<'input>
{
    Subject::IriRef(__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Subject<'input>
{
    Subject::BlankNodeLabel(__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Object<'input>
{
    Object::IriRef(__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Object<'input>
{
    Object::BlankNodeLabel(__0)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Literal<'input>, usize),
) -> Object<'input>
{
    Object::Literal(__0)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    text: &'input str,
    (_, value, _): (usize, &'input str, usize),
    (_, datatype, _): (usize, ::std::option::Option<Datatype<'input>>, usize),
) -> Literal<'input>
{
    Literal {value:value, datatype:datatype}
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, &'input str, usize),
) -> Datatype<'input>
{
    Datatype::IriRef(__0)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Datatype<'input>
{
    Datatype::LangTag(__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Datatype<'input>, usize),
) -> ::std::option::Option<Datatype<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    text: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Datatype<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Token<'input>, usize),
) -> ::std::option::Option<Token<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    text: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Token<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    text: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(Token<'input>, Triple<'input>)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
) -> ::std::vec::Vec<(Token<'input>, Triple<'input>)>
{
    v
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Token<'input>, usize),
    (_, __1, _): (usize, Triple<'input>, usize),
) -> (Token<'input>, Triple<'input>)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Triple<'input>, usize),
) -> ::std::option::Option<Triple<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    text: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Triple<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, (Token<'input>, Triple<'input>), usize),
) -> ::std::vec::Vec<(Token<'input>, Triple<'input>)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
    (_, e, _): (usize, (Token<'input>, Triple<'input>), usize),
) -> ::std::vec::Vec<(Token<'input>, Triple<'input>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
    __2: (usize, Token<'input>, usize),
) -> Document<'input>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action18(
        text,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        text,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
) -> Document<'input>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action19(
        text,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        text,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    text: &'input str,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Triple<'input>, usize),
) -> ::std::vec::Vec<(Token<'input>, Triple<'input>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action22(
        text,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        text,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, Triple<'input>, usize),
) -> ::std::vec::Vec<(Token<'input>, Triple<'input>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action22(
        text,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        text,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, Token<'input>, usize),
) -> Document<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action20(
        text,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        text,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
    __2: (usize, Token<'input>, usize),
) -> Document<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        text,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        text,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
) -> Document<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action20(
        text,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        text,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
) -> Document<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        text,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        text,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    text: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Datatype<'input>, usize),
) -> Literal<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action16(
        text,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        text,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    text: &'input str,
    __0: (usize, &'input str, usize),
) -> Literal<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action17(
        text,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        text,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    text: &'input str,
    __0: (usize, Triple<'input>, usize),
    __1: (usize, Token<'input>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action23(
        text,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        text,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    text: &'input str,
    __0: (usize, Token<'input>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action24(
        text,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        text,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    text: &'input str,
    __0: (usize, Triple<'input>, usize),
    __1: (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
    __2: (usize, Token<'input>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action23(
        text,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        text,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
    __1: (usize, Token<'input>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action24(
        text,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        text,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    text: &'input str,
    __0: (usize, Triple<'input>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action23(
        text,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        text,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    text: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Document<'input>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action24(
        text,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        text,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    text: &'input str,
    __0: (usize, Triple<'input>, usize),
    __1: (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action23(
        text,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        text,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::vec::Vec<(Token<'input>, Triple<'input>)>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action24(
        text,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        text,
        __temp0,
        __0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    type Error = lexer::Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),lexer::Error> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize),lexer::Error> {
    type Error = lexer::Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),lexer::Error> {
        value
    }
}
