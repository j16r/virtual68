#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::{Register, Opcode, Instruction, Expression};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Register, Opcode, Instruction, Expression};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Program<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Instruction>, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Program(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_20_22_2b(::std::vec::Vec<&'input str>),
        _22_5c_5cn_22_3f(::std::option::Option<&'input str>),
        Expression(Expression),
        Instruction(Instruction),
        Instruction_2a(::std::vec::Vec<Instruction>),
        Instruction_2b(::std::vec::Vec<Instruction>),
        Num(u8),
        Opcode(Opcode),
        Program(Vec<Instruction>),
        Register(Register),
        ____Program(Vec<Instruction>),
    }

    // State 0
    //   Instruction = (*) Opcode " "+ Expression [EOF]
    //   Instruction = (*) Opcode " "+ Expression ["\\n"]
    //   Instruction = (*) Opcode " "+ Expression [r#"[a-z]+"#]
    //   Instruction = (*) Opcode " "+ Expression " "+ Expression [EOF]
    //   Instruction = (*) Opcode " "+ Expression " "+ Expression ["\\n"]
    //   Instruction = (*) Opcode " "+ Expression " "+ Expression [r#"[a-z]+"#]
    //   Instruction = (*) "\\n" Opcode " "+ Expression [EOF]
    //   Instruction = (*) "\\n" Opcode " "+ Expression ["\\n"]
    //   Instruction = (*) "\\n" Opcode " "+ Expression [r#"[a-z]+"#]
    //   Instruction = (*) "\\n" Opcode " "+ Expression " "+ Expression [EOF]
    //   Instruction = (*) "\\n" Opcode " "+ Expression " "+ Expression ["\\n"]
    //   Instruction = (*) "\\n" Opcode " "+ Expression " "+ Expression [r#"[a-z]+"#]
    //   Instruction+ = (*) Instruction [EOF]
    //   Instruction+ = (*) Instruction ["\\n"]
    //   Instruction+ = (*) Instruction [r#"[a-z]+"#]
    //   Instruction+ = (*) Instruction+ Instruction [EOF]
    //   Instruction+ = (*) Instruction+ Instruction ["\\n"]
    //   Instruction+ = (*) Instruction+ Instruction [r#"[a-z]+"#]
    //   Opcode = (*) r#"[a-z]+"# [" "]
    //   Program = (*) [EOF]
    //   Program = (*) Instruction+ [EOF]
    //   __Program = (*) Program [EOF]
    //
    //   EOF -> Reduce(Program =  => ActionFn(21);)
    //   "\\n" -> Shift(S5)
    //   r#"[a-z]+"# -> Shift(S6)
    //
    //   Instruction -> S1
    //   Instruction+ -> S2
    //   Opcode -> S3
    //   Program -> S4
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0));
            }
            None => {
                let __nt = super::__action21(input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Program(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Instruction(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Instruction_2b(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Opcode(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Program(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Instruction+ = Instruction (*) [EOF]
    //   Instruction+ = Instruction (*) ["\\n"]
    //   Instruction+ = Instruction (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Instruction+ = Instruction => ActionFn(15);)
    //   "\\n" -> Reduce(Instruction+ = Instruction => ActionFn(15);)
    //   r#"[a-z]+"# -> Reduce(Instruction+ = Instruction => ActionFn(15);)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Instruction>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Instruction_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   Instruction = (*) Opcode " "+ Expression [EOF]
    //   Instruction = (*) Opcode " "+ Expression ["\\n"]
    //   Instruction = (*) Opcode " "+ Expression [r#"[a-z]+"#]
    //   Instruction = (*) Opcode " "+ Expression " "+ Expression [EOF]
    //   Instruction = (*) Opcode " "+ Expression " "+ Expression ["\\n"]
    //   Instruction = (*) Opcode " "+ Expression " "+ Expression [r#"[a-z]+"#]
    //   Instruction = (*) "\\n" Opcode " "+ Expression [EOF]
    //   Instruction = (*) "\\n" Opcode " "+ Expression ["\\n"]
    //   Instruction = (*) "\\n" Opcode " "+ Expression [r#"[a-z]+"#]
    //   Instruction = (*) "\\n" Opcode " "+ Expression " "+ Expression [EOF]
    //   Instruction = (*) "\\n" Opcode " "+ Expression " "+ Expression ["\\n"]
    //   Instruction = (*) "\\n" Opcode " "+ Expression " "+ Expression [r#"[a-z]+"#]
    //   Instruction+ = Instruction+ (*) Instruction [EOF]
    //   Instruction+ = Instruction+ (*) Instruction ["\\n"]
    //   Instruction+ = Instruction+ (*) Instruction [r#"[a-z]+"#]
    //   Opcode = (*) r#"[a-z]+"# [" "]
    //   Program = Instruction+ (*) [EOF]
    //
    //   EOF -> Reduce(Program = Instruction+ => ActionFn(22);)
    //   "\\n" -> Shift(S5)
    //   r#"[a-z]+"# -> Shift(S6)
    //
    //   Instruction -> S7
    //   Opcode -> S3
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Instruction>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Program(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Instruction(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Opcode(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 3
    //   " "+ = (*) " "+ " " [" "]
    //   " "+ = (*) " "+ " " [r#"[0-9]+"#]
    //   " "+ = (*) " "+ " " [r#"[a-z]+"#]
    //   " "+ = (*) " " [" "]
    //   " "+ = (*) " " [r#"[0-9]+"#]
    //   " "+ = (*) " " [r#"[a-z]+"#]
    //   Instruction = Opcode (*) " "+ Expression [EOF]
    //   Instruction = Opcode (*) " "+ Expression ["\\n"]
    //   Instruction = Opcode (*) " "+ Expression [r#"[a-z]+"#]
    //   Instruction = Opcode (*) " "+ Expression " "+ Expression [EOF]
    //   Instruction = Opcode (*) " "+ Expression " "+ Expression ["\\n"]
    //   Instruction = Opcode (*) " "+ Expression " "+ Expression [r#"[a-z]+"#]
    //
    //   " " -> Shift(S9)
    //
    //   " "+ -> S8
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Opcode>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_20_22_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 4
    //   __Program = Program (*) [EOF]
    //
    //   EOF -> Reduce(__Program = Program => ActionFn(0);)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<Instruction>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Program(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   Instruction = "\\n" (*) Opcode " "+ Expression [EOF]
    //   Instruction = "\\n" (*) Opcode " "+ Expression ["\\n"]
    //   Instruction = "\\n" (*) Opcode " "+ Expression [r#"[a-z]+"#]
    //   Instruction = "\\n" (*) Opcode " "+ Expression " "+ Expression [EOF]
    //   Instruction = "\\n" (*) Opcode " "+ Expression " "+ Expression ["\\n"]
    //   Instruction = "\\n" (*) Opcode " "+ Expression " "+ Expression [r#"[a-z]+"#]
    //   Opcode = (*) r#"[a-z]+"# [" "]
    //
    //   r#"[a-z]+"# -> Shift(S6)
    //
    //   Opcode -> S10
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Opcode(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 6
    //   Opcode = r#"[a-z]+"# (*) [" "]
    //
    //   " " -> Reduce(Opcode = r#"[a-z]+"# => ActionFn(7);)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Opcode(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 7
    //   Instruction+ = Instruction+ Instruction (*) [EOF]
    //   Instruction+ = Instruction+ Instruction (*) ["\\n"]
    //   Instruction+ = Instruction+ Instruction (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Instruction+ = Instruction+, Instruction => ActionFn(16);)
    //   "\\n" -> Reduce(Instruction+ = Instruction+, Instruction => ActionFn(16);)
    //   r#"[a-z]+"# -> Reduce(Instruction+ = Instruction+, Instruction => ActionFn(16);)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Instruction>>,
        __sym1: &mut Option<Instruction>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action16(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Instruction_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   " "+ = " "+ (*) " " [" "]
    //   " "+ = " "+ (*) " " [r#"[0-9]+"#]
    //   " "+ = " "+ (*) " " [r#"[a-z]+"#]
    //   Expression = (*) Num [EOF]
    //   Expression = (*) Num [" "]
    //   Expression = (*) Num ["\\n"]
    //   Expression = (*) Num [r#"[a-z]+"#]
    //   Expression = (*) Register [EOF]
    //   Expression = (*) Register [" "]
    //   Expression = (*) Register ["\\n"]
    //   Expression = (*) Register [r#"[a-z]+"#]
    //   Instruction = Opcode " "+ (*) Expression [EOF]
    //   Instruction = Opcode " "+ (*) Expression ["\\n"]
    //   Instruction = Opcode " "+ (*) Expression [r#"[a-z]+"#]
    //   Instruction = Opcode " "+ (*) Expression " "+ Expression [EOF]
    //   Instruction = Opcode " "+ (*) Expression " "+ Expression ["\\n"]
    //   Instruction = Opcode " "+ (*) Expression " "+ Expression [r#"[a-z]+"#]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# [" "]
    //   Num = (*) r#"[0-9]+"# ["\\n"]
    //   Num = (*) r#"[0-9]+"# [r#"[a-z]+"#]
    //   Register = (*) r#"[a-z]+"# [EOF]
    //   Register = (*) r#"[a-z]+"# [" "]
    //   Register = (*) r#"[a-z]+"# ["\\n"]
    //   Register = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   " " -> Shift(S14)
    //   r#"[0-9]+"# -> Shift(S15)
    //   r#"[a-z]+"# -> Shift(S16)
    //
    //   Expression -> S11
    //   Num -> S12
    //   Register -> S13
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Opcode>,
        __sym1: &mut Option<::std::vec::Vec<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Register(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   " "+ = " " (*) [" "]
    //   " "+ = " " (*) [r#"[0-9]+"#]
    //   " "+ = " " (*) [r#"[a-z]+"#]
    //
    //   " " -> Reduce(" "+ = " " => ActionFn(9);)
    //   r#"[0-9]+"# -> Reduce(" "+ = " " => ActionFn(9);)
    //   r#"[a-z]+"# -> Reduce(" "+ = " " => ActionFn(9);)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_22_20_22_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 10
    //   " "+ = (*) " "+ " " [" "]
    //   " "+ = (*) " "+ " " [r#"[0-9]+"#]
    //   " "+ = (*) " "+ " " [r#"[a-z]+"#]
    //   " "+ = (*) " " [" "]
    //   " "+ = (*) " " [r#"[0-9]+"#]
    //   " "+ = (*) " " [r#"[a-z]+"#]
    //   Instruction = "\\n" Opcode (*) " "+ Expression [EOF]
    //   Instruction = "\\n" Opcode (*) " "+ Expression ["\\n"]
    //   Instruction = "\\n" Opcode (*) " "+ Expression [r#"[a-z]+"#]
    //   Instruction = "\\n" Opcode (*) " "+ Expression " "+ Expression [EOF]
    //   Instruction = "\\n" Opcode (*) " "+ Expression " "+ Expression ["\\n"]
    //   Instruction = "\\n" Opcode (*) " "+ Expression " "+ Expression [r#"[a-z]+"#]
    //
    //   " " -> Shift(S9)
    //
    //   " "+ -> S17
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Opcode>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_20_22_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 11
    //   " "+ = (*) " "+ " " [" "]
    //   " "+ = (*) " "+ " " [r#"[0-9]+"#]
    //   " "+ = (*) " "+ " " [r#"[a-z]+"#]
    //   " "+ = (*) " " [" "]
    //   " "+ = (*) " " [r#"[0-9]+"#]
    //   " "+ = (*) " " [r#"[a-z]+"#]
    //   Instruction = Opcode " "+ Expression (*) [EOF]
    //   Instruction = Opcode " "+ Expression (*) ["\\n"]
    //   Instruction = Opcode " "+ Expression (*) [r#"[a-z]+"#]
    //   Instruction = Opcode " "+ Expression (*) " "+ Expression [EOF]
    //   Instruction = Opcode " "+ Expression (*) " "+ Expression ["\\n"]
    //   Instruction = Opcode " "+ Expression (*) " "+ Expression [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Instruction = Opcode, " "+, Expression => ActionFn(18);)
    //   " " -> Shift(S9)
    //   "\\n" -> Reduce(Instruction = Opcode, " "+, Expression => ActionFn(18);)
    //   r#"[a-z]+"# -> Reduce(Instruction = Opcode, " "+, Expression => ActionFn(18);)
    //
    //   " "+ -> S18
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Opcode>,
        __sym1: &mut Option<::std::vec::Vec<&'input str>>,
        __sym2: &mut Option<Expression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym3));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action18(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Instruction(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_20_22_2b(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Expression = Num (*) [EOF]
    //   Expression = Num (*) [" "]
    //   Expression = Num (*) ["\\n"]
    //   Expression = Num (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Expression = Num => ActionFn(4);)
    //   " " -> Reduce(Expression = Num => ActionFn(4);)
    //   "\\n" -> Reduce(Expression = Num => ActionFn(4);)
    //   r#"[a-z]+"# -> Reduce(Expression = Num => ActionFn(4);)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<u8>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   Expression = Register (*) [EOF]
    //   Expression = Register (*) [" "]
    //   Expression = Register (*) ["\\n"]
    //   Expression = Register (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Expression = Register => ActionFn(5);)
    //   " " -> Reduce(Expression = Register => ActionFn(5);)
    //   "\\n" -> Reduce(Expression = Register => ActionFn(5);)
    //   r#"[a-z]+"# -> Reduce(Expression = Register => ActionFn(5);)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Register>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   " "+ = " "+ " " (*) [" "]
    //   " "+ = " "+ " " (*) [r#"[0-9]+"#]
    //   " "+ = " "+ " " (*) [r#"[a-z]+"#]
    //
    //   " " -> Reduce(" "+ = " "+, " " => ActionFn(10);)
    //   r#"[0-9]+"# -> Reduce(" "+ = " "+, " " => ActionFn(10);)
    //   r#"[a-z]+"# -> Reduce(" "+ = " "+, " " => ActionFn(10);)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<&'input str>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action10(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_22_20_22_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   Num = r#"[0-9]+"# (*) [EOF]
    //   Num = r#"[0-9]+"# (*) [" "]
    //   Num = r#"[0-9]+"# (*) ["\\n"]
    //   Num = r#"[0-9]+"# (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Num = r#"[0-9]+"# => ActionFn(6);)
    //   " " -> Reduce(Num = r#"[0-9]+"# => ActionFn(6);)
    //   "\\n" -> Reduce(Num = r#"[0-9]+"# => ActionFn(6);)
    //   r#"[a-z]+"# -> Reduce(Num = r#"[0-9]+"# => ActionFn(6);)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Num(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
    //   Register = r#"[a-z]+"# (*) [EOF]
    //   Register = r#"[a-z]+"# (*) [" "]
    //   Register = r#"[a-z]+"# (*) ["\\n"]
    //   Register = r#"[a-z]+"# (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Register = r#"[a-z]+"# => ActionFn(8);)
    //   " " -> Reduce(Register = r#"[a-z]+"# => ActionFn(8);)
    //   "\\n" -> Reduce(Register = r#"[a-z]+"# => ActionFn(8);)
    //   r#"[a-z]+"# -> Reduce(Register = r#"[a-z]+"# => ActionFn(8);)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Register(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 17
    //   " "+ = " "+ (*) " " [" "]
    //   " "+ = " "+ (*) " " [r#"[0-9]+"#]
    //   " "+ = " "+ (*) " " [r#"[a-z]+"#]
    //   Expression = (*) Num [EOF]
    //   Expression = (*) Num [" "]
    //   Expression = (*) Num ["\\n"]
    //   Expression = (*) Num [r#"[a-z]+"#]
    //   Expression = (*) Register [EOF]
    //   Expression = (*) Register [" "]
    //   Expression = (*) Register ["\\n"]
    //   Expression = (*) Register [r#"[a-z]+"#]
    //   Instruction = "\\n" Opcode " "+ (*) Expression [EOF]
    //   Instruction = "\\n" Opcode " "+ (*) Expression ["\\n"]
    //   Instruction = "\\n" Opcode " "+ (*) Expression [r#"[a-z]+"#]
    //   Instruction = "\\n" Opcode " "+ (*) Expression " "+ Expression [EOF]
    //   Instruction = "\\n" Opcode " "+ (*) Expression " "+ Expression ["\\n"]
    //   Instruction = "\\n" Opcode " "+ (*) Expression " "+ Expression [r#"[a-z]+"#]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# [" "]
    //   Num = (*) r#"[0-9]+"# ["\\n"]
    //   Num = (*) r#"[0-9]+"# [r#"[a-z]+"#]
    //   Register = (*) r#"[a-z]+"# [EOF]
    //   Register = (*) r#"[a-z]+"# [" "]
    //   Register = (*) r#"[a-z]+"# ["\\n"]
    //   Register = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   " " -> Shift(S14)
    //   r#"[0-9]+"# -> Shift(S15)
    //   r#"[a-z]+"# -> Shift(S16)
    //
    //   Expression -> S19
    //   Num -> S12
    //   Register -> S13
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Opcode>,
        __sym2: &mut Option<::std::vec::Vec<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expression(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Register(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   " "+ = " "+ (*) " " [" "]
    //   " "+ = " "+ (*) " " [r#"[0-9]+"#]
    //   " "+ = " "+ (*) " " [r#"[a-z]+"#]
    //   Expression = (*) Num [EOF]
    //   Expression = (*) Num ["\\n"]
    //   Expression = (*) Num [r#"[a-z]+"#]
    //   Expression = (*) Register [EOF]
    //   Expression = (*) Register ["\\n"]
    //   Expression = (*) Register [r#"[a-z]+"#]
    //   Instruction = Opcode " "+ Expression " "+ (*) Expression [EOF]
    //   Instruction = Opcode " "+ Expression " "+ (*) Expression ["\\n"]
    //   Instruction = Opcode " "+ Expression " "+ (*) Expression [r#"[a-z]+"#]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["\\n"]
    //   Num = (*) r#"[0-9]+"# [r#"[a-z]+"#]
    //   Register = (*) r#"[a-z]+"# [EOF]
    //   Register = (*) r#"[a-z]+"# ["\\n"]
    //   Register = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   " " -> Shift(S14)
    //   r#"[0-9]+"# -> Shift(S23)
    //   r#"[a-z]+"# -> Shift(S24)
    //
    //   Expression -> S20
    //   Num -> S21
    //   Register -> S22
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Opcode>,
        __sym1: &mut Option<::std::vec::Vec<&'input str>>,
        __sym2: &mut Option<Expression>,
        __sym3: &mut Option<::std::vec::Vec<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym3, __sym4));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state24(input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expression(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Register(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 19
    //   " "+ = (*) " "+ " " [" "]
    //   " "+ = (*) " "+ " " [r#"[0-9]+"#]
    //   " "+ = (*) " "+ " " [r#"[a-z]+"#]
    //   " "+ = (*) " " [" "]
    //   " "+ = (*) " " [r#"[0-9]+"#]
    //   " "+ = (*) " " [r#"[a-z]+"#]
    //   Instruction = "\\n" Opcode " "+ Expression (*) [EOF]
    //   Instruction = "\\n" Opcode " "+ Expression (*) ["\\n"]
    //   Instruction = "\\n" Opcode " "+ Expression (*) [r#"[a-z]+"#]
    //   Instruction = "\\n" Opcode " "+ Expression (*) " "+ Expression [EOF]
    //   Instruction = "\\n" Opcode " "+ Expression (*) " "+ Expression ["\\n"]
    //   Instruction = "\\n" Opcode " "+ Expression (*) " "+ Expression [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Instruction = "\\n", Opcode, " "+, Expression => ActionFn(17);)
    //   " " -> Shift(S9)
    //   "\\n" -> Reduce(Instruction = "\\n", Opcode, " "+, Expression => ActionFn(17);)
    //   r#"[a-z]+"# -> Reduce(Instruction = "\\n", Opcode, " "+, Expression => ActionFn(17);)
    //
    //   " "+ -> S25
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Opcode>,
        __sym2: &mut Option<::std::vec::Vec<&'input str>>,
        __sym3: &mut Option<Expression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym4));
            }
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action17(input, __sym0, __sym1, __sym2, __sym3, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Instruction(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_22_20_22_2b(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 20
    //   Instruction = Opcode " "+ Expression " "+ Expression (*) [EOF]
    //   Instruction = Opcode " "+ Expression " "+ Expression (*) ["\\n"]
    //   Instruction = Opcode " "+ Expression " "+ Expression (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Instruction = Opcode, " "+, Expression, " "+, Expression => ActionFn(20);)
    //   "\\n" -> Reduce(Instruction = Opcode, " "+, Expression, " "+, Expression => ActionFn(20);)
    //   r#"[a-z]+"# -> Reduce(Instruction = Opcode, " "+, Expression, " "+, Expression => ActionFn(20);)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Opcode>,
        __sym1: &mut Option<::std::vec::Vec<&'input str>>,
        __sym2: &mut Option<Expression>,
        __sym3: &mut Option<::std::vec::Vec<&'input str>>,
        __sym4: &mut Option<Expression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action20(input, __sym0, __sym1, __sym2, __sym3, __sym4, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Instruction(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 21
    //   Expression = Num (*) [EOF]
    //   Expression = Num (*) ["\\n"]
    //   Expression = Num (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Expression = Num => ActionFn(4);)
    //   "\\n" -> Reduce(Expression = Num => ActionFn(4);)
    //   r#"[a-z]+"# -> Reduce(Expression = Num => ActionFn(4);)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<u8>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //   Expression = Register (*) [EOF]
    //   Expression = Register (*) ["\\n"]
    //   Expression = Register (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Expression = Register => ActionFn(5);)
    //   "\\n" -> Reduce(Expression = Register => ActionFn(5);)
    //   r#"[a-z]+"# -> Reduce(Expression = Register => ActionFn(5);)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Register>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expression(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 23
    //   Num = r#"[0-9]+"# (*) [EOF]
    //   Num = r#"[0-9]+"# (*) ["\\n"]
    //   Num = r#"[0-9]+"# (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Num = r#"[0-9]+"# => ActionFn(6);)
    //   "\\n" -> Reduce(Num = r#"[0-9]+"# => ActionFn(6);)
    //   r#"[a-z]+"# -> Reduce(Num = r#"[0-9]+"# => ActionFn(6);)
    //
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Num(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 24
    //   Register = r#"[a-z]+"# (*) [EOF]
    //   Register = r#"[a-z]+"# (*) ["\\n"]
    //   Register = r#"[a-z]+"# (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Register = r#"[a-z]+"# => ActionFn(8);)
    //   "\\n" -> Reduce(Register = r#"[a-z]+"# => ActionFn(8);)
    //   r#"[a-z]+"# -> Reduce(Register = r#"[a-z]+"# => ActionFn(8);)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Register(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 25
    //   " "+ = " "+ (*) " " [" "]
    //   " "+ = " "+ (*) " " [r#"[0-9]+"#]
    //   " "+ = " "+ (*) " " [r#"[a-z]+"#]
    //   Expression = (*) Num [EOF]
    //   Expression = (*) Num ["\\n"]
    //   Expression = (*) Num [r#"[a-z]+"#]
    //   Expression = (*) Register [EOF]
    //   Expression = (*) Register ["\\n"]
    //   Expression = (*) Register [r#"[a-z]+"#]
    //   Instruction = "\\n" Opcode " "+ Expression " "+ (*) Expression [EOF]
    //   Instruction = "\\n" Opcode " "+ Expression " "+ (*) Expression ["\\n"]
    //   Instruction = "\\n" Opcode " "+ Expression " "+ (*) Expression [r#"[a-z]+"#]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# ["\\n"]
    //   Num = (*) r#"[0-9]+"# [r#"[a-z]+"#]
    //   Register = (*) r#"[a-z]+"# [EOF]
    //   Register = (*) r#"[a-z]+"# ["\\n"]
    //   Register = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   " " -> Shift(S14)
    //   r#"[0-9]+"# -> Shift(S23)
    //   r#"[a-z]+"# -> Shift(S24)
    //
    //   Expression -> S26
    //   Num -> S21
    //   Register -> S22
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Opcode>,
        __sym2: &mut Option<::std::vec::Vec<&'input str>>,
        __sym3: &mut Option<Expression>,
        __sym4: &mut Option<::std::vec::Vec<&'input str>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym4, __sym5));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state24(input, __lookbehind, __tokens, __sym5));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym4.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expression(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
                }
                __Nonterminal::Num(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Register(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 26
    //   Instruction = "\\n" Opcode " "+ Expression " "+ Expression (*) [EOF]
    //   Instruction = "\\n" Opcode " "+ Expression " "+ Expression (*) ["\\n"]
    //   Instruction = "\\n" Opcode " "+ Expression " "+ Expression (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Instruction = "\\n", Opcode, " "+, Expression, " "+, Expression => ActionFn(19);)
    //   "\\n" -> Reduce(Instruction = "\\n", Opcode, " "+, Expression, " "+, Expression => ActionFn(19);)
    //   r#"[a-z]+"# -> Reduce(Instruction = "\\n", Opcode, " "+, Expression, " "+, Expression => ActionFn(19);)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Opcode>,
        __sym2: &mut Option<::std::vec::Vec<&'input str>>,
        __sym3: &mut Option<Expression>,
        __sym4: &mut Option<::std::vec::Vec<&'input str>>,
        __sym5: &mut Option<Expression>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __nt = super::__action19(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Instruction(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Program::parse_Program;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ' ' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '\\' => {
                            __current_state = 3;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        's' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        't' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'n' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        's' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        't' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    __0: Vec<Instruction>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: ::std::vec::Vec<Instruction>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    _: ::std::option::Option<&'input str>,
    op: Opcode,
    _: ::std::vec::Vec<&'input str>,
    arg: Expression,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Instruction
{
    match op {
        Opcode::Push => Instruction::Push(arg),
        Opcode::Pop => Instruction::Pop(arg),
        _ => panic!("wrong number of arguments for opcode"),
    }
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    _: ::std::option::Option<&'input str>,
    op: Opcode,
    _: ::std::vec::Vec<&'input str>,
    lhs: Expression,
    _: ::std::vec::Vec<&'input str>,
    rhs: Expression,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Instruction
{
    match op {
        Opcode::Mov => Instruction::Mov(lhs, rhs),
        _ => panic!("wrong number of arguments for opcode"),
    }
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    v: u8,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Expression
{
    Expression::Value(v)
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    r: Register,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Expression
{
    Expression::Register(r)
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> u8
{
    u8::from_str(__0).unwrap()
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Opcode
{
    match __0 {
    "push" => Opcode::Push,
    "pop" => Opcode::Pop,
    "mov" => Opcode::Mov,
    _ => panic!("unrecognized opcode"),
}
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Register
{
    match __0 {
    "acca" => Register::ACCA,
    "accb" => Register::ACCB,
    "x" => Register::X,
    "pc" => Register::PC,
    "sp" => Register::SP,
    "cc" => Register::CC,
    _ => panic!("unrecognized opcode"),
}
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<&'input str>,
    e: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::option::Option<&'input str>
{
    Some(__0)
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::option::Option<&'input str>
{
    None
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Instruction>
{
    vec![]
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<Instruction>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Instruction>
{
    v
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: Instruction,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Instruction>
{
    vec![__0]
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<Instruction>,
    e: Instruction,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Instruction>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __1: Opcode,
    __2: ::std::vec::Vec<&'input str>,
    __3: Expression,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Instruction
{
    let __temp0 = __action11(
        input,
        __0,
        __lookbehind,
        __lookahead,
    );
    __action2(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    __0: Opcode,
    __1: ::std::vec::Vec<&'input str>,
    __2: Expression,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Instruction
{
    let __temp0 = __action12(
        input,
        __lookbehind,
        __lookahead,
    );
    __action2(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __1: Opcode,
    __2: ::std::vec::Vec<&'input str>,
    __3: Expression,
    __4: ::std::vec::Vec<&'input str>,
    __5: Expression,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Instruction
{
    let __temp0 = __action11(
        input,
        __0,
        __lookbehind,
        __lookahead,
    );
    __action3(
        input,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    __0: Opcode,
    __1: ::std::vec::Vec<&'input str>,
    __2: Expression,
    __3: ::std::vec::Vec<&'input str>,
    __4: Expression,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Instruction
{
    let __temp0 = __action12(
        input,
        __lookbehind,
        __lookahead,
    );
    __action3(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    let __temp0 = __action13(
        input,
        __lookbehind,
        __lookahead,
    );
    __action1(
        input,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    __0: ::std::vec::Vec<Instruction>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    let __temp0 = __action14(
        input,
        __0,
        __lookbehind,
        __lookahead,
    );
    __action1(
        input,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
