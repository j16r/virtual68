#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use ast::{Register, Operation, Instruction, Place};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Register, Operation, Instruction, Place};
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
    pub enum __Nonterminal<> {
        _28_3cInstruction_3e_20_22_3b_22_29(Instruction),
        _28_3cInstruction_3e_20_22_3b_22_29_2a(::std::vec::Vec<Instruction>),
        _28_3cInstruction_3e_20_22_3b_22_29_2b(::std::vec::Vec<Instruction>),
        Instruction(Instruction),
        Instruction_3f(::std::option::Option<Instruction>),
        Num(u8),
        Operation(Operation),
        Place(Place),
        Program(Vec<Instruction>),
        Register(Register),
        ____Program(Vec<Instruction>),
    }

    // State 0
    //   (<Instruction> ";")+ = (*) (<Instruction> ";")+ Instruction ";" [EOF]
    //   (<Instruction> ";")+ = (*) (<Instruction> ";")+ Instruction ";" [r#"[a-z]+"#]
    //   (<Instruction> ";")+ = (*) Instruction ";" [EOF]
    //   (<Instruction> ";")+ = (*) Instruction ";" [r#"[a-z]+"#]
    //   Instruction = (*) Operation Place [EOF]
    //   Instruction = (*) Operation Place [";"]
    //   Instruction = (*) Operation Place Place [EOF]
    //   Instruction = (*) Operation Place Place [";"]
    //   Operation = (*) r#"[a-z]+"# [r#"[0-9]+"#]
    //   Operation = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //   Program = (*) [EOF]
    //   Program = (*) (<Instruction> ";")+ [EOF]
    //   Program = (*) (<Instruction> ";")+ Instruction [EOF]
    //   Program = (*) Instruction [EOF]
    //   __Program = (*) Program [EOF]
    //
    //   EOF -> Reduce(Program =  => ActionFn(21);)
    //   r#"[a-z]+"# -> Shift(S5)
    //
    //   (<Instruction> ";")+ -> S1
    //   Instruction -> S2
    //   Operation -> S3
    //   Program -> S4
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
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
                __Nonterminal::_28_3cInstruction_3e_20_22_3b_22_29_2b(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Instruction(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Operation(__nt) => {
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
    //   (<Instruction> ";")+ = (<Instruction> ";")+ (*) Instruction ";" [EOF]
    //   (<Instruction> ";")+ = (<Instruction> ";")+ (*) Instruction ";" [r#"[a-z]+"#]
    //   Instruction = (*) Operation Place [EOF]
    //   Instruction = (*) Operation Place [";"]
    //   Instruction = (*) Operation Place Place [EOF]
    //   Instruction = (*) Operation Place Place [";"]
    //   Operation = (*) r#"[a-z]+"# [r#"[0-9]+"#]
    //   Operation = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //   Program = (<Instruction> ";")+ (*) [EOF]
    //   Program = (<Instruction> ";")+ (*) Instruction [EOF]
    //
    //   EOF -> Reduce(Program = (<Instruction> ";")+ => ActionFn(23);)
    //   r#"[a-z]+"# -> Shift(S5)
    //
    //   Instruction -> S6
    //   Operation -> S3
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Instruction>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action23(input, __sym0, &__lookbehind, &__lookahead);
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
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Operation(__nt) => {
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

    // State 2
    //   (<Instruction> ";")+ = Instruction (*) ";" [EOF]
    //   (<Instruction> ";")+ = Instruction (*) ";" [r#"[a-z]+"#]
    //   Program = Instruction (*) [EOF]
    //
    //   EOF -> Reduce(Program = Instruction => ActionFn(20);)
    //   ";" -> Shift(S7)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Instruction>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action20(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Program(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Instruction = Operation (*) Place [EOF]
    //   Instruction = Operation (*) Place [";"]
    //   Instruction = Operation (*) Place Place [EOF]
    //   Instruction = Operation (*) Place Place [";"]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# [";"]
    //   Num = (*) r#"[0-9]+"# [r#"[0-9]+"#]
    //   Num = (*) r#"[0-9]+"# [r#"[a-z]+"#]
    //   Place = (*) Num [EOF]
    //   Place = (*) Num [";"]
    //   Place = (*) Num [r#"[0-9]+"#]
    //   Place = (*) Num [r#"[a-z]+"#]
    //   Place = (*) Register [EOF]
    //   Place = (*) Register [";"]
    //   Place = (*) Register [r#"[0-9]+"#]
    //   Place = (*) Register [r#"[a-z]+"#]
    //   Register = (*) r#"[a-z]+"# [EOF]
    //   Register = (*) r#"[a-z]+"# [";"]
    //   Register = (*) r#"[a-z]+"# [r#"[0-9]+"#]
    //   Register = (*) r#"[a-z]+"# [r#"[a-z]+"#]
    //
    //   r#"[0-9]+"# -> Shift(S11)
    //   r#"[a-z]+"# -> Shift(S12)
    //
    //   Num -> S8
    //   Place -> S9
    //   Register -> S10
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Operation>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym1));
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
                __Nonterminal::Num(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Place(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Register(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym1));
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
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
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
    //   Operation = r#"[a-z]+"# (*) [r#"[0-9]+"#]
    //   Operation = r#"[a-z]+"# (*) [r#"[a-z]+"#]
    //
    //   r#"[0-9]+"# -> Reduce(Operation = r#"[a-z]+"# => ActionFn(4);)
    //   r#"[a-z]+"# -> Reduce(Operation = r#"[a-z]+"# => ActionFn(4);)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Operation(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   (<Instruction> ";")+ = (<Instruction> ";")+ Instruction (*) ";" [EOF]
    //   (<Instruction> ";")+ = (<Instruction> ";")+ Instruction (*) ";" [r#"[a-z]+"#]
    //   Program = (<Instruction> ";")+ Instruction (*) [EOF]
    //
    //   EOF -> Reduce(Program = (<Instruction> ";")+, Instruction => ActionFn(22);)
    //   ";" -> Shift(S13)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Instruction>>,
        __sym1: &mut Option<Instruction>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action22(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Program(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 7
    //   (<Instruction> ";")+ = Instruction ";" (*) [EOF]
    //   (<Instruction> ";")+ = Instruction ";" (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce((<Instruction> ";")+ = Instruction, ";" => ActionFn(16);)
    //   r#"[a-z]+"# -> Reduce((<Instruction> ";")+ = Instruction, ";" => ActionFn(16);)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Instruction>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action16(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cInstruction_3e_20_22_3b_22_29_2b(__nt)));
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
    //   Place = Num (*) [EOF]
    //   Place = Num (*) [";"]
    //   Place = Num (*) [r#"[0-9]+"#]
    //   Place = Num (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Place = Num => ActionFn(6);)
    //   ";" -> Reduce(Place = Num => ActionFn(6);)
    //   r#"[0-9]+"# -> Reduce(Place = Num => ActionFn(6);)
    //   r#"[a-z]+"# -> Reduce(Place = Num => ActionFn(6);)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<u8>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Place(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   Instruction = Operation Place (*) [EOF]
    //   Instruction = Operation Place (*) [";"]
    //   Instruction = Operation Place (*) Place [EOF]
    //   Instruction = Operation Place (*) Place [";"]
    //   Num = (*) r#"[0-9]+"# [EOF]
    //   Num = (*) r#"[0-9]+"# [";"]
    //   Place = (*) Num [EOF]
    //   Place = (*) Num [";"]
    //   Place = (*) Register [EOF]
    //   Place = (*) Register [";"]
    //   Register = (*) r#"[a-z]+"# [EOF]
    //   Register = (*) r#"[a-z]+"# [";"]
    //
    //   EOF -> Reduce(Instruction = Operation, Place => ActionFn(3);)
    //   ";" -> Reduce(Instruction = Operation, Place => ActionFn(3);)
    //   r#"[0-9]+"# -> Shift(S17)
    //   r#"[a-z]+"# -> Shift(S18)
    //
    //   Num -> S14
    //   Place -> S15
    //   Register -> S16
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Operation>,
        __sym1: &mut Option<Place>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2));
            }
            None |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action3(input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Instruction(__nt)));
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
                __Nonterminal::Num(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Place(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Register(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   Place = Register (*) [EOF]
    //   Place = Register (*) [";"]
    //   Place = Register (*) [r#"[0-9]+"#]
    //   Place = Register (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Place = Register => ActionFn(5);)
    //   ";" -> Reduce(Place = Register => ActionFn(5);)
    //   r#"[0-9]+"# -> Reduce(Place = Register => ActionFn(5);)
    //   r#"[a-z]+"# -> Reduce(Place = Register => ActionFn(5);)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Register>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Place(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   Num = r#"[0-9]+"# (*) [EOF]
    //   Num = r#"[0-9]+"# (*) [";"]
    //   Num = r#"[0-9]+"# (*) [r#"[0-9]+"#]
    //   Num = r#"[0-9]+"# (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Num = r#"[0-9]+"# => ActionFn(8);)
    //   ";" -> Reduce(Num = r#"[0-9]+"# => ActionFn(8);)
    //   r#"[0-9]+"# -> Reduce(Num = r#"[0-9]+"# => ActionFn(8);)
    //   r#"[a-z]+"# -> Reduce(Num = r#"[0-9]+"# => ActionFn(8);)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0, &__lookbehind, &__lookahead);
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

    // State 12
    //   Register = r#"[a-z]+"# (*) [EOF]
    //   Register = r#"[a-z]+"# (*) [";"]
    //   Register = r#"[a-z]+"# (*) [r#"[0-9]+"#]
    //   Register = r#"[a-z]+"# (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce(Register = r#"[a-z]+"# => ActionFn(7);)
    //   ";" -> Reduce(Register = r#"[a-z]+"# => ActionFn(7);)
    //   r#"[0-9]+"# -> Reduce(Register = r#"[a-z]+"# => ActionFn(7);)
    //   r#"[a-z]+"# -> Reduce(Register = r#"[a-z]+"# => ActionFn(7);)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0, &__lookbehind, &__lookahead);
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

    // State 13
    //   (<Instruction> ";")+ = (<Instruction> ";")+ Instruction ";" (*) [EOF]
    //   (<Instruction> ";")+ = (<Instruction> ";")+ Instruction ";" (*) [r#"[a-z]+"#]
    //
    //   EOF -> Reduce((<Instruction> ";")+ = (<Instruction> ";")+, Instruction, ";" => ActionFn(17);)
    //   r#"[a-z]+"# -> Reduce((<Instruction> ";")+ = (<Instruction> ";")+, Instruction, ";" => ActionFn(17);)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<Instruction>>,
        __sym1: &mut Option<Instruction>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action17(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cInstruction_3e_20_22_3b_22_29_2b(__nt)));
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
    //   Place = Num (*) [EOF]
    //   Place = Num (*) [";"]
    //
    //   EOF -> Reduce(Place = Num => ActionFn(6);)
    //   ";" -> Reduce(Place = Num => ActionFn(6);)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<u8>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Place(__nt)));
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
    //   Instruction = Operation Place Place (*) [EOF]
    //   Instruction = Operation Place Place (*) [";"]
    //
    //   EOF -> Reduce(Instruction = Operation, Place, Place => ActionFn(2);)
    //   ";" -> Reduce(Instruction = Operation, Place, Place => ActionFn(2);)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Operation>,
        __sym1: &mut Option<Place>,
        __sym2: &mut Option<Place>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
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

    // State 16
    //   Place = Register (*) [EOF]
    //   Place = Register (*) [";"]
    //
    //   EOF -> Reduce(Place = Register => ActionFn(5);)
    //   ";" -> Reduce(Place = Register => ActionFn(5);)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Register>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Place(__nt)));
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
    //   Num = r#"[0-9]+"# (*) [EOF]
    //   Num = r#"[0-9]+"# (*) [";"]
    //
    //   EOF -> Reduce(Num = r#"[0-9]+"# => ActionFn(8);)
    //   ";" -> Reduce(Num = r#"[0-9]+"# => ActionFn(8);)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0, &__lookbehind, &__lookahead);
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

    // State 18
    //   Register = r#"[a-z]+"# (*) [EOF]
    //   Register = r#"[a-z]+"# (*) [";"]
    //
    //   EOF -> Reduce(Register = r#"[a-z]+"# => ActionFn(7);)
    //   ";" -> Reduce(Register = r#"[a-z]+"# => ActionFn(7);)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0, &__lookbehind, &__lookahead);
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
                        '0' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        ';' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        's' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        't' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
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
                        '0' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'a' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        's' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        't' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
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
    v: ::std::vec::Vec<Instruction>,
    e: ::std::option::Option<Instruction>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    match e {
        None => v,
        Some(e) => {
            println!("Got {:?}, {:?}", e, v);
            let mut v = v;
            v.push(e);
            v
        }
    }
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    op: Operation,
    lhs: Place,
    rhs: Place,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Instruction
{
    match op {
        Operation::Mov => Instruction::Mov(lhs, rhs),
        _ => panic!("wrong number of arguments for opcode"),
    }
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    op: Operation,
    arg: Place,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Instruction
{
    match op {
        Operation::Push => Instruction::Push(arg),
        Operation::Pop => Instruction::Pop(arg),
        _ => panic!("wrong number of arguments for opcode"),
    }
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Operation
{
    match __0 {
    "push" => Operation::Push,
    "pop" => Operation::Pop,
    "mov" => Operation::Mov,
    op => panic!("unrecognized opcode {:?}", op),
}
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    r: Register,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Place
{
    Place::Register(r)
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    v: u8,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Place
{
    Place::Value(v)
}

pub fn __action7<
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
    reg => panic!("unrecognized register {:?}", reg),
}
}

pub fn __action8<
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

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: Instruction,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::option::Option<Instruction>
{
    Some(__0)
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::option::Option<Instruction>
{
    None
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Instruction>
{
    vec![]
}

pub fn __action12<
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

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: Instruction,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Instruction
{
    (__0)
}

pub fn __action14<
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

pub fn __action15<
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

pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: Instruction,
    __1: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Instruction>
{
    let __temp0 = __action13(
        input,
        __0,
        __1,
        __lookbehind,
        __lookahead,
    );
    __action14(
        input,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __0: ::std::vec::Vec<Instruction>,
    __1: Instruction,
    __2: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<Instruction>
{
    let __temp0 = __action13(
        input,
        __1,
        __2,
        __lookbehind,
        __lookahead,
    );
    __action15(
        input,
        __0,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    __0: ::std::option::Option<Instruction>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    let __temp0 = __action11(
        input,
        __lookbehind,
        __lookahead,
    );
    __action1(
        input,
        __temp0,
        __0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    __0: ::std::vec::Vec<Instruction>,
    __1: ::std::option::Option<Instruction>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    let __temp0 = __action12(
        input,
        __0,
        __lookbehind,
        __lookahead,
    );
    __action1(
        input,
        __temp0,
        __1,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    __0: Instruction,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    let __temp0 = __action9(
        input,
        __0,
        __lookbehind,
        __lookahead,
    );
    __action18(
        input,
        __temp0,
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
    let __temp0 = __action10(
        input,
        __lookbehind,
        __lookahead,
    );
    __action18(
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
    __1: Instruction,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    let __temp0 = __action9(
        input,
        __1,
        __lookbehind,
        __lookahead,
    );
    __action19(
        input,
        __0,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action23<
    'input,
>(
    input: &'input str,
    __0: ::std::vec::Vec<Instruction>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Vec<Instruction>
{
    let __temp0 = __action10(
        input,
        __lookbehind,
        __lookahead,
    );
    __action19(
        input,
        __0,
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
