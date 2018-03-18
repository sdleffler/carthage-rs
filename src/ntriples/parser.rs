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
        Nt_22EOL_22_2a(::std::vec::Vec<Token<'input>>),
        Nt_22EOL_22_2b(::std::vec::Vec<Token<'input>>),
        Nt_28_22EOL_22_2b_20Triple_29((::std::vec::Vec<Token<'input>>, Triple<'input>)),
        Nt_28_22EOL_22_2b_20Triple_29_2a(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
        Nt_28_22EOL_22_2b_20Triple_29_2b(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
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
        -32,
        // State 2
        -11,
        // State 3
        0,
        // State 4
        -10,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Datatype<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            2 => {
                // "EOL"* = "EOL"+ => ActionFn(19);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            3 => {
                // "EOL"+ = "EOL" => ActionFn(23);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            4 => {
                // "EOL"+ = "EOL"+, "EOL" => ActionFn(24);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            5 => {
                // ("EOL"+ Triple) = "EOL"+, Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__nt), __end));
                2
            }
            6 => {
                // ("EOL"+ Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            7 => {
                // ("EOL"+ Triple)* = ("EOL"+ Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            8 => {
                // ("EOL"+ Triple)+ = "EOL"+, Triple => ActionFn(31);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            9 => {
                // ("EOL"+ Triple)+ = ("EOL"+ Triple)+, "EOL"+, Triple => ActionFn(32);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            10 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            11 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            12 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            13 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            14 => {
                // Document = Triple => ActionFn(39);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            15 => {
                // Document =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            16 => {
                // Document = Triple, ("EOL"+ Triple)+ => ActionFn(41);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action41::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            17 => {
                // Document = ("EOL"+ Triple)+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            18 => {
                // Document = Triple, "EOL"+ => ActionFn(43);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            19 => {
                // Document = "EOL"+ => ActionFn(44);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            20 => {
                // Document = Triple, ("EOL"+ Triple)+, "EOL"+ => ActionFn(45);
                let __sym2 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            21 => {
                // Document = ("EOL"+ Triple)+, "EOL"+ => ActionFn(46);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            22 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(37);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            23 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(38);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            24 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            25 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            26 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            27 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            28 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            29 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                11
            }
            30 => {
                // Triple? = Triple => ActionFn(25);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            31 => {
                // Triple? =  => ActionFn(26);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action26::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            32 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                return Some(Ok(__nt));
            }
            33 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                14
            }
            34 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                15
            }
            35 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                16
            }
            36 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                17
            }
            37 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
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
    fn __pop_Nt_22EOL_22_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (::std::vec::Vec<Token<'input>>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__v), __r) => (__l, __v, __r),
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
        Nt_22EOL_22_2a(::std::vec::Vec<Token<'input>>),
        Nt_22EOL_22_2b(::std::vec::Vec<Token<'input>>),
        Nt_28_22EOL_22_2b_20Triple_29((::std::vec::Vec<Token<'input>>, Triple<'input>)),
        Nt_28_22EOL_22_2b_20Triple_29_2a(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
        Nt_28_22EOL_22_2b_20Triple_29_2b(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
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
        0, 7, 8, 9, 0, 0, 0,
        // State 1
        0, 7, 11, 9, 0, 0, 0,
        // State 2
        0, 0, 8, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 13, 0, 0, 0,
        // State 5
        0, 0, 8, 0, 0, 0, 0,
        // State 6
        0, 0, 0, -28, 0, 0, 0,
        // State 7
        0, -3, -3, -3, 0, 0, 0,
        // State 8
        0, 0, 0, -27, 0, 0, 0,
        // State 9
        0, 0, -8, 0, 0, 0, 0,
        // State 10
        0, -4, -4, -4, 0, 0, 0,
        // State 11
        0, 7, 11, 9, 0, 0, 0,
        // State 12
        0, 19, 0, 20, 0, 21, 0,
        // State 13
        0, 7, 11, 9, 0, 0, 0,
        // State 14
        0, 0, 8, 0, 0, 0, 0,
        // State 15
        0, 0, -9, 0, 0, 0, 0,
        // State 16
        -26, 0, 0, 0, 0, 0, 0,
        // State 17
        23, 0, 0, 0, 0, 0, 0,
        // State 18
        -25, 0, 0, 0, 0, 0, 0,
        // State 19
        -24, 0, 0, 0, 0, 0, 0,
        // State 20
        -23, 0, 0, 0, 25, 0, 26,
        // State 21
        0, 7, 11, 9, 0, 0, 0,
        // State 22
        0, 0, -29, 0, 0, 0, 0,
        // State 23
        -22, 0, 0, 0, 0, 0, 0,
        // State 24
        -11, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 27, 0, 0, 0,
        // State 26
        -10, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        -15,
        // State 1
        -19,
        // State 2
        -17,
        // State 3
        -33,
        // State 4
        0,
        // State 5
        -14,
        // State 6
        0,
        // State 7
        -3,
        // State 8
        0,
        // State 9
        -8,
        // State 10
        -4,
        // State 11
        -21,
        // State 12
        0,
        // State 13
        -18,
        // State 14
        -16,
        // State 15
        -9,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        -20,
        // State 22
        -29,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 2, 0, 0, 3, 0, 0, 4, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 16, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 16, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Document<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            2 => {
                // "EOL"* = "EOL"+ => ActionFn(19);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            3 => {
                // "EOL"+ = "EOL" => ActionFn(23);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            4 => {
                // "EOL"+ = "EOL"+, "EOL" => ActionFn(24);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            5 => {
                // ("EOL"+ Triple) = "EOL"+, Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__nt), __end));
                2
            }
            6 => {
                // ("EOL"+ Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            7 => {
                // ("EOL"+ Triple)* = ("EOL"+ Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            8 => {
                // ("EOL"+ Triple)+ = "EOL"+, Triple => ActionFn(31);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            9 => {
                // ("EOL"+ Triple)+ = ("EOL"+ Triple)+, "EOL"+, Triple => ActionFn(32);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            10 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            11 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            12 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            13 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            14 => {
                // Document = Triple => ActionFn(39);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            15 => {
                // Document =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            16 => {
                // Document = Triple, ("EOL"+ Triple)+ => ActionFn(41);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action41::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            17 => {
                // Document = ("EOL"+ Triple)+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            18 => {
                // Document = Triple, "EOL"+ => ActionFn(43);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            19 => {
                // Document = "EOL"+ => ActionFn(44);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            20 => {
                // Document = Triple, ("EOL"+ Triple)+, "EOL"+ => ActionFn(45);
                let __sym2 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            21 => {
                // Document = ("EOL"+ Triple)+, "EOL"+ => ActionFn(46);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            22 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(37);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            23 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(38);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            24 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            25 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            26 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            27 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            28 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            29 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                11
            }
            30 => {
                // Triple? = Triple => ActionFn(25);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            31 => {
                // Triple? =  => ActionFn(26);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action26::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            32 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                13
            }
            33 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            34 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                15
            }
            35 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                16
            }
            36 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                17
            }
            37 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
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
    fn __pop_Nt_22EOL_22_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (::std::vec::Vec<Token<'input>>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__v), __r) => (__l, __v, __r),
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
        Nt_22EOL_22_2a(::std::vec::Vec<Token<'input>>),
        Nt_22EOL_22_2b(::std::vec::Vec<Token<'input>>),
        Nt_28_22EOL_22_2b_20Triple_29((::std::vec::Vec<Token<'input>>, Triple<'input>)),
        Nt_28_22EOL_22_2b_20Triple_29_2a(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
        Nt_28_22EOL_22_2b_20Triple_29_2b(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
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
        -34,
        // State 2
        -23,
        // State 3
        -22,
        // State 4
        -11,
        // State 5
        0,
        // State 6
        -10,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Literal<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            2 => {
                // "EOL"* = "EOL"+ => ActionFn(19);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            3 => {
                // "EOL"+ = "EOL" => ActionFn(23);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            4 => {
                // "EOL"+ = "EOL"+, "EOL" => ActionFn(24);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            5 => {
                // ("EOL"+ Triple) = "EOL"+, Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__nt), __end));
                2
            }
            6 => {
                // ("EOL"+ Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            7 => {
                // ("EOL"+ Triple)* = ("EOL"+ Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            8 => {
                // ("EOL"+ Triple)+ = "EOL"+, Triple => ActionFn(31);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            9 => {
                // ("EOL"+ Triple)+ = ("EOL"+ Triple)+, "EOL"+, Triple => ActionFn(32);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            10 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            11 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            12 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            13 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            14 => {
                // Document = Triple => ActionFn(39);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            15 => {
                // Document =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            16 => {
                // Document = Triple, ("EOL"+ Triple)+ => ActionFn(41);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action41::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            17 => {
                // Document = ("EOL"+ Triple)+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            18 => {
                // Document = Triple, "EOL"+ => ActionFn(43);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            19 => {
                // Document = "EOL"+ => ActionFn(44);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            20 => {
                // Document = Triple, ("EOL"+ Triple)+, "EOL"+ => ActionFn(45);
                let __sym2 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            21 => {
                // Document = ("EOL"+ Triple)+, "EOL"+ => ActionFn(46);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            22 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(37);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            23 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(38);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            24 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            25 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            26 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            27 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            28 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            29 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                11
            }
            30 => {
                // Triple? = Triple => ActionFn(25);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            31 => {
                // Triple? =  => ActionFn(26);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action26::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            32 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                13
            }
            33 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                14
            }
            34 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                return Some(Ok(__nt));
            }
            35 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                16
            }
            36 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                17
            }
            37 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
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
    fn __pop_Nt_22EOL_22_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (::std::vec::Vec<Token<'input>>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__v), __r) => (__l, __v, __r),
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
        Nt_22EOL_22_2a(::std::vec::Vec<Token<'input>>),
        Nt_22EOL_22_2b(::std::vec::Vec<Token<'input>>),
        Nt_28_22EOL_22_2b_20Triple_29((::std::vec::Vec<Token<'input>>, Triple<'input>)),
        Nt_28_22EOL_22_2b_20Triple_29_2a(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
        Nt_28_22EOL_22_2b_20Triple_29_2b(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
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
        -26,
        // State 2
        -35,
        // State 3
        -25,
        // State 4
        -24,
        // State 5
        -23,
        // State 6
        -22,
        // State 7
        -11,
        // State 8
        0,
        // State 9
        -10,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Object<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            2 => {
                // "EOL"* = "EOL"+ => ActionFn(19);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            3 => {
                // "EOL"+ = "EOL" => ActionFn(23);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            4 => {
                // "EOL"+ = "EOL"+, "EOL" => ActionFn(24);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            5 => {
                // ("EOL"+ Triple) = "EOL"+, Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__nt), __end));
                2
            }
            6 => {
                // ("EOL"+ Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            7 => {
                // ("EOL"+ Triple)* = ("EOL"+ Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            8 => {
                // ("EOL"+ Triple)+ = "EOL"+, Triple => ActionFn(31);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            9 => {
                // ("EOL"+ Triple)+ = ("EOL"+ Triple)+, "EOL"+, Triple => ActionFn(32);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            10 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            11 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            12 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            13 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            14 => {
                // Document = Triple => ActionFn(39);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            15 => {
                // Document =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            16 => {
                // Document = Triple, ("EOL"+ Triple)+ => ActionFn(41);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action41::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            17 => {
                // Document = ("EOL"+ Triple)+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            18 => {
                // Document = Triple, "EOL"+ => ActionFn(43);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            19 => {
                // Document = "EOL"+ => ActionFn(44);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            20 => {
                // Document = Triple, ("EOL"+ Triple)+, "EOL"+ => ActionFn(45);
                let __sym2 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            21 => {
                // Document = ("EOL"+ Triple)+, "EOL"+ => ActionFn(46);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            22 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(37);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            23 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(38);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            24 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            25 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            26 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            27 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            28 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            29 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                11
            }
            30 => {
                // Triple? = Triple => ActionFn(25);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            31 => {
                // Triple? =  => ActionFn(26);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action26::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            32 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                13
            }
            33 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                14
            }
            34 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                15
            }
            35 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                return Some(Ok(__nt));
            }
            36 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                17
            }
            37 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
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
    fn __pop_Nt_22EOL_22_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (::std::vec::Vec<Token<'input>>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__v), __r) => (__l, __v, __r),
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
        Nt_22EOL_22_2a(::std::vec::Vec<Token<'input>>),
        Nt_22EOL_22_2b(::std::vec::Vec<Token<'input>>),
        Nt_28_22EOL_22_2b_20Triple_29((::std::vec::Vec<Token<'input>>, Triple<'input>)),
        Nt_28_22EOL_22_2b_20Triple_29_2a(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
        Nt_28_22EOL_22_2b_20Triple_29_2b(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
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
        -36,
        // State 2
        -28,
        // State 3
        -27,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Subject<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            2 => {
                // "EOL"* = "EOL"+ => ActionFn(19);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            3 => {
                // "EOL"+ = "EOL" => ActionFn(23);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            4 => {
                // "EOL"+ = "EOL"+, "EOL" => ActionFn(24);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            5 => {
                // ("EOL"+ Triple) = "EOL"+, Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__nt), __end));
                2
            }
            6 => {
                // ("EOL"+ Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            7 => {
                // ("EOL"+ Triple)* = ("EOL"+ Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            8 => {
                // ("EOL"+ Triple)+ = "EOL"+, Triple => ActionFn(31);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            9 => {
                // ("EOL"+ Triple)+ = ("EOL"+ Triple)+, "EOL"+, Triple => ActionFn(32);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            10 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            11 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            12 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            13 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            14 => {
                // Document = Triple => ActionFn(39);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            15 => {
                // Document =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            16 => {
                // Document = Triple, ("EOL"+ Triple)+ => ActionFn(41);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action41::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            17 => {
                // Document = ("EOL"+ Triple)+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            18 => {
                // Document = Triple, "EOL"+ => ActionFn(43);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            19 => {
                // Document = "EOL"+ => ActionFn(44);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            20 => {
                // Document = Triple, ("EOL"+ Triple)+, "EOL"+ => ActionFn(45);
                let __sym2 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            21 => {
                // Document = ("EOL"+ Triple)+, "EOL"+ => ActionFn(46);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            22 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(37);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            23 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(38);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            24 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            25 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            26 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            27 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            28 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            29 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                11
            }
            30 => {
                // Triple? = Triple => ActionFn(25);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            31 => {
                // Triple? =  => ActionFn(26);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action26::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            32 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                13
            }
            33 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                14
            }
            34 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                15
            }
            35 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                16
            }
            36 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            37 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Triple(__nt), __end));
                18
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
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
    fn __pop_Nt_22EOL_22_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (::std::vec::Vec<Token<'input>>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__v), __r) => (__l, __v, __r),
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
        Nt_22EOL_22_2a(::std::vec::Vec<Token<'input>>),
        Nt_22EOL_22_2b(::std::vec::Vec<Token<'input>>),
        Nt_28_22EOL_22_2b_20Triple_29((::std::vec::Vec<Token<'input>>, Triple<'input>)),
        Nt_28_22EOL_22_2b_20Triple_29_2a(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
        Nt_28_22EOL_22_2b_20Triple_29_2b(::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>),
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
        0, 0, 0, -28, 0, 0, 0,
        // State 4
        0, 0, 0, -27, 0, 0, 0,
        // State 5
        0, 9, 0, 10, 0, 11, 0,
        // State 6
        -26, 0, 0, 0, 0, 0, 0,
        // State 7
        12, 0, 0, 0, 0, 0, 0,
        // State 8
        -25, 0, 0, 0, 0, 0, 0,
        // State 9
        -24, 0, 0, 0, 0, 0, 0,
        // State 10
        -23, 0, 0, 0, 14, 0, 15,
        // State 11
        0, 0, 0, 0, 0, 0, 0,
        // State 12
        -22, 0, 0, 0, 0, 0, 0,
        // State 13
        -11, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 16, 0, 0, 0,
        // State 15
        -10, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -37,
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
        -29,
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
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Triple<'input>,__lalrpop_util::ParseError<usize, Token<'input>, lexer::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // "EOL"* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            2 => {
                // "EOL"* = "EOL"+ => ActionFn(19);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2a(__nt), __end));
                0
            }
            3 => {
                // "EOL"+ = "EOL" => ActionFn(23);
                let __sym0 = __pop_Term_22EOL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            4 => {
                // "EOL"+ = "EOL"+, "EOL" => ActionFn(24);
                let __sym1 = __pop_Term_22EOL_22(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_22EOL_22_2b(__nt), __end));
                1
            }
            5 => {
                // ("EOL"+ Triple) = "EOL"+, Triple => ActionFn(22);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__nt), __end));
                2
            }
            6 => {
                // ("EOL"+ Triple)* =  => ActionFn(20);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action20::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            7 => {
                // ("EOL"+ Triple)* = ("EOL"+ Triple)+ => ActionFn(21);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__nt), __end));
                3
            }
            8 => {
                // ("EOL"+ Triple)+ = "EOL"+, Triple => ActionFn(31);
                let __sym1 = __pop_NtTriple(__symbols);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            9 => {
                // ("EOL"+ Triple)+ = ("EOL"+ Triple)+, "EOL"+, Triple => ActionFn(32);
                let __sym2 = __pop_NtTriple(__symbols);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__nt), __end));
                4
            }
            10 => {
                // Datatype = "^^", "IRIREF" => ActionFn(14);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_Term_22_5e_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            11 => {
                // Datatype = "LANGTAG" => ActionFn(15);
                let __sym0 = __pop_Term_22LANGTAG_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype(__nt), __end));
                5
            }
            12 => {
                // Datatype? = Datatype => ActionFn(16);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            13 => {
                // Datatype? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDatatype_3f(__nt), __end));
                6
            }
            14 => {
                // Document = Triple => ActionFn(39);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            15 => {
                // Document =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            16 => {
                // Document = Triple, ("EOL"+ Triple)+ => ActionFn(41);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action41::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            17 => {
                // Document = ("EOL"+ Triple)+ => ActionFn(42);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            18 => {
                // Document = Triple, "EOL"+ => ActionFn(43);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            19 => {
                // Document = "EOL"+ => ActionFn(44);
                let __sym0 = __pop_Nt_22EOL_22_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            20 => {
                // Document = Triple, ("EOL"+ Triple)+, "EOL"+ => ActionFn(45);
                let __sym2 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym1 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            21 => {
                // Document = ("EOL"+ Triple)+, "EOL"+ => ActionFn(46);
                let __sym1 = __pop_Nt_22EOL_22_2b(__symbols);
                let __sym0 = __pop_Nt_28_22EOL_22_2b_20Triple_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDocument(__nt), __end));
                7
            }
            22 => {
                // Literal = "STRING_LITERAL_QUOTE", Datatype => ActionFn(37);
                let __sym1 = __pop_NtDatatype(__symbols);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            23 => {
                // Literal = "STRING_LITERAL_QUOTE" => ActionFn(38);
                let __sym0 = __pop_Term_22STRING__LITERAL__QUOTE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                8
            }
            24 => {
                // Object = "IRIREF" => ActionFn(10);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            25 => {
                // Object = "BLANK_NODE_LABEL" => ActionFn(11);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            26 => {
                // Object = Literal => ActionFn(12);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtObject(__nt), __end));
                9
            }
            27 => {
                // Subject = "IRIREF" => ActionFn(8);
                let __sym0 = __pop_Term_22IRIREF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            28 => {
                // Subject = "BLANK_NODE_LABEL" => ActionFn(9);
                let __sym0 = __pop_Term_22BLANK__NODE__LABEL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSubject(__nt), __end));
                10
            }
            29 => {
                // Triple = Subject, "IRIREF", Object, "." => ActionFn(7);
                let __sym3 = __pop_Term_22_2e_22(__symbols);
                let __sym2 = __pop_NtObject(__symbols);
                let __sym1 = __pop_Term_22IRIREF_22(__symbols);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTriple(__nt), __end));
                11
            }
            30 => {
                // Triple? = Triple => ActionFn(25);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            31 => {
                // Triple? =  => ActionFn(26);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action26::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTriple_3f(__nt), __end));
                12
            }
            32 => {
                // __Datatype = Datatype => ActionFn(5);
                let __sym0 = __pop_NtDatatype(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Datatype(__nt), __end));
                13
            }
            33 => {
                // __Document = Document => ActionFn(0);
                let __sym0 = __pop_NtDocument(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Document(__nt), __end));
                14
            }
            34 => {
                // __Literal = Literal => ActionFn(4);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Literal(__nt), __end));
                15
            }
            35 => {
                // __Object = Object => ActionFn(3);
                let __sym0 = __pop_NtObject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Object(__nt), __end));
                16
            }
            36 => {
                // __Subject = Subject => ActionFn(2);
                let __sym0 = __pop_NtSubject(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Subject(__nt), __end));
                17
            }
            37 => {
                // __Triple = Triple => ActionFn(1);
                let __sym0 = __pop_NtTriple(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
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
    fn __pop_Nt_22EOL_22_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22EOL_22_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22EOL_22_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (::std::vec::Vec<Token<'input>>, Triple<'input>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22EOL_22_2b_20Triple_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22EOL_22_2b_20Triple_29_2b(__v), __r) => (__l, __v, __r),
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

fn __action0<
    'input,
>(
    (_, __0, _): (usize, Document<'input>, usize),
) -> Document<'input>
{
    (__0)
}

fn __action1<
    'input,
>(
    (_, __0, _): (usize, Triple<'input>, usize),
) -> Triple<'input>
{
    (__0)
}

fn __action2<
    'input,
>(
    (_, __0, _): (usize, Subject<'input>, usize),
) -> Subject<'input>
{
    (__0)
}

fn __action3<
    'input,
>(
    (_, __0, _): (usize, Object<'input>, usize),
) -> Object<'input>
{
    (__0)
}

fn __action4<
    'input,
>(
    (_, __0, _): (usize, Literal<'input>, usize),
) -> Literal<'input>
{
    (__0)
}

fn __action5<
    'input,
>(
    (_, __0, _): (usize, Datatype<'input>, usize),
) -> Datatype<'input>
{
    (__0)
}

fn __action6<
    'input,
>(
    (_, head, _): (usize, ::std::option::Option<Triple<'input>>, usize),
    (_, body, _): (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
    (_, _, _): (usize, ::std::vec::Vec<Token<'input>>, usize),
) -> Document<'input>
{
    Document {
    triples: head.into_iter()
        .chain(body.into_iter().map(|(_, triple)| triple))
        .collect(),
}
}

fn __action7<
    'input,
>(
    (_, subject, _): (usize, Subject<'input>, usize),
    (_, predicate, _): (usize, &'input str, usize),
    (_, object, _): (usize, Object<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Triple<'input>
{
    Triple {subject:subject, predicate:predicate, object:object}
}

fn __action8<
    'input,
>(
    (_, __0, _): (usize, &'input str, usize),
) -> Subject<'input>
{
    Subject::IriRef(__0)
}

fn __action9<
    'input,
>(
    (_, __0, _): (usize, &'input str, usize),
) -> Subject<'input>
{
    Subject::BlankNodeLabel(__0)
}

fn __action10<
    'input,
>(
    (_, __0, _): (usize, &'input str, usize),
) -> Object<'input>
{
    Object::IriRef(__0)
}

fn __action11<
    'input,
>(
    (_, __0, _): (usize, &'input str, usize),
) -> Object<'input>
{
    Object::BlankNodeLabel(__0)
}

fn __action12<
    'input,
>(
    (_, __0, _): (usize, Literal<'input>, usize),
) -> Object<'input>
{
    Object::Literal(__0)
}

fn __action13<
    'input,
>(
    (_, value, _): (usize, &'input str, usize),
    (_, datatype, _): (usize, ::std::option::Option<Datatype<'input>>, usize),
) -> Literal<'input>
{
    Literal {value:value, datatype:datatype}
}

fn __action14<
    'input,
>(
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, &'input str, usize),
) -> Datatype<'input>
{
    Datatype::IriRef(__0)
}

fn __action15<
    'input,
>(
    (_, __0, _): (usize, &'input str, usize),
) -> Datatype<'input>
{
    Datatype::LangTag(__0)
}

fn __action16<
    'input,
>(
    (_, __0, _): (usize, Datatype<'input>, usize),
) -> ::std::option::Option<Datatype<'input>>
{
    Some(__0)
}

fn __action17<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Datatype<'input>>
{
    None
}

fn __action18<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Token<'input>>
{
    vec![]
}

fn __action19<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Token<'input>>, usize),
) -> ::std::vec::Vec<Token<'input>>
{
    v
}

fn __action20<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>
{
    vec![]
}

fn __action21<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
) -> ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>
{
    v
}

fn __action22<
    'input,
>(
    (_, __0, _): (usize, ::std::vec::Vec<Token<'input>>, usize),
    (_, __1, _): (usize, Triple<'input>, usize),
) -> (::std::vec::Vec<Token<'input>>, Triple<'input>)
{
    (__0, __1)
}

fn __action23<
    'input,
>(
    (_, __0, _): (usize, Token<'input>, usize),
) -> ::std::vec::Vec<Token<'input>>
{
    vec![__0]
}

fn __action24<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Token<'input>>, usize),
    (_, e, _): (usize, Token<'input>, usize),
) -> ::std::vec::Vec<Token<'input>>
{
    { let mut v = v; v.push(e); v }
}

fn __action25<
    'input,
>(
    (_, __0, _): (usize, Triple<'input>, usize),
) -> ::std::option::Option<Triple<'input>>
{
    Some(__0)
}

fn __action26<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Triple<'input>>
{
    None
}

fn __action27<
    'input,
>(
    (_, __0, _): (usize, (::std::vec::Vec<Token<'input>>, Triple<'input>), usize),
) -> ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>
{
    vec![__0]
}

fn __action28<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
    (_, e, _): (usize, (::std::vec::Vec<Token<'input>>, Triple<'input>), usize),
) -> ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>
{
    { let mut v = v; v.push(e); v }
}

fn __action29<
    'input,
>(
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
) -> Document<'input>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __0,
        __1,
        __temp0,
    )
}

fn __action30<
    'input,
>(
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
    __2: (usize, ::std::vec::Vec<Token<'input>>, usize),
) -> Document<'input>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action19(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __0,
        __1,
        __temp0,
    )
}

fn __action31<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Token<'input>>, usize),
    __1: (usize, Triple<'input>, usize),
) -> ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action22(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        __temp0,
    )
}

fn __action32<
    'input,
>(
    __0: (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
    __1: (usize, ::std::vec::Vec<Token<'input>>, usize),
    __2: (usize, Triple<'input>, usize),
) -> ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action22(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        __0,
        __temp0,
    )
}

fn __action33<
    'input,
>(
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
) -> Document<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action20(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __0,
        __temp0,
    )
}

fn __action34<
    'input,
>(
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
) -> Document<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __0,
        __temp0,
    )
}

fn __action35<
    'input,
>(
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, ::std::vec::Vec<Token<'input>>, usize),
) -> Document<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action20(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        __0,
        __temp0,
        __1,
    )
}

fn __action36<
    'input,
>(
    __0: (usize, ::std::option::Option<Triple<'input>>, usize),
    __1: (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
    __2: (usize, ::std::vec::Vec<Token<'input>>, usize),
) -> Document<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        __0,
        __temp0,
        __2,
    )
}

fn __action37<
    'input,
>(
    __0: (usize, &'input str, usize),
    __1: (usize, Datatype<'input>, usize),
) -> Literal<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action16(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __temp0,
    )
}

fn __action38<
    'input,
>(
    __0: (usize, &'input str, usize),
) -> Literal<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action17(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __temp0,
    )
}

fn __action39<
    'input,
>(
    __0: (usize, Triple<'input>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        __temp0,
    )
}

fn __action40<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Document<'input>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        __temp0,
    )
}

fn __action41<
    'input,
>(
    __0: (usize, Triple<'input>, usize),
    __1: (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        __temp0,
        __1,
    )
}

fn __action42<
    'input,
>(
    __0: (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        __temp0,
        __0,
    )
}

fn __action43<
    'input,
>(
    __0: (usize, Triple<'input>, usize),
    __1: (usize, ::std::vec::Vec<Token<'input>>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __temp0,
        __1,
    )
}

fn __action44<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Token<'input>>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __temp0,
        __0,
    )
}

fn __action45<
    'input,
>(
    __0: (usize, Triple<'input>, usize),
    __1: (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
    __2: (usize, ::std::vec::Vec<Token<'input>>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __temp0,
        __1,
        __2,
    )
}

fn __action46<
    'input,
>(
    __0: (usize, ::std::vec::Vec<(::std::vec::Vec<Token<'input>>, Triple<'input>)>, usize),
    __1: (usize, ::std::vec::Vec<Token<'input>>, usize),
) -> Document<'input>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __temp0,
        __0,
        __1,
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
