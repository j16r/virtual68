use combine::{alpha_num, char, choice, letter, newline, optional, sep_end_by, try, ParseError,
              Parser, ParserExt, many1, skip_many1};

use ast::{Address, Command, Instruction};

pub type Program = Vec<Instruction>;

fn is_decimal(input: &str) -> bool {
    input.chars().all(|ch| ch.is_digit(10))
}

pub fn parse_program(input: &str) -> Result<Program, &str> {
    let opcode = || {
        many1(letter()).map(|token: String| {
            let command = match token.as_ref() {
                "aba" => Command::ABA,
                "adc" => Command::ADC,
                "add" => Command::ADD,
                "and" => Command::AND,
                "asl" => Command::ASL,
                "asr" => Command::ASR,
                "bcc" => Command::BCC,
                "bcs" => Command::BCS,
                "beq" => Command::BEQ,
                "bge" => Command::BGE,
                "bgt" => Command::BGT,
                "bhi" => Command::BHI,
                "bit" => Command::BIT,
                "ble" => Command::BLE,
                "bls" => Command::BLS,
                "blt" => Command::BLT,
                "bmi" => Command::BMI,
                "bne" => Command::BNE,
                "bpl" => Command::BPL,
                "bra" => Command::BRA,
                "bsr" => Command::BSR,
                "bvc" => Command::BVC,
                "bvs" => Command::BVS,
                "cba" => Command::CBA,
                "clc" => Command::CLC,
                "cli" => Command::CLI,
                "clr" => Command::CLR,
                "clv" => Command::CLV,
                "cmp" => Command::CMP,
                "com" => Command::COM,
                "cpx" => Command::CPX,
                "daa" => Command::DAA,
                "dec" => Command::DEC,
                "des" => Command::DES,
                "dex" => Command::DEX,
                "eor" => Command::EOR,
                "inc" => Command::INC,
                "ins" => Command::INS,
                "inx" => Command::INX,
                "jmp" => Command::JMP,
                "jsr" => Command::JSR,
                "lda" => Command::LDA,
                "lds" => Command::LDS,
                "ldx" => Command::LDX,
                "lsr" => Command::LSR,
                "neg" => Command::NEG,
                "nop" => Command::NOP,
                "ora" => Command::ORA,
                "psh" => Command::PSH,
                "pul" => Command::PUL,
                "rol" => Command::ROL,
                "ror" => Command::ROR,
                "rti" => Command::RTI,
                "rts" => Command::RTS,
                "sba" => Command::SBA,
                "sbc" => Command::SBC,
                "sec" => Command::SEC,
                "sei" => Command::SEI,
                "sev" => Command::SEV,
                "sta" => Command::STA,
                "sts" => Command::STS,
                "stx" => Command::STX,
                "sub" => Command::SUB,
                "swi" => Command::SWI,
                "tab" => Command::TAB,
                "tap" => Command::TAP,
                "tba" => Command::TBA,
                "tpa" => Command::TPA,
                "tst" => Command::TST,
                "tsx" => Command::TSX,
                "txs" => Command::TXS,
                "wai" => Command::WAI,
                //_ => return Err(format!("unrecognized opcode '{}'", token))
                _ => panic!("unrecognized opcode '{}'", token),
            };
            command
        })
    };

    let address = || {
        (optional(choice([char(':'), char('@')])), many1(alpha_num())).map(
            |(prefix, token): (Option<char>, String)| match token.as_ref() {
                "a" => Address::AccumulatorA,
                "b" => Address::AccumulatorB,
                _ => {
                    let value = token.parse().unwrap();
                    match prefix {
                        Some(':') => {
                            if value > 255 {
                                Address::Extended(value)
                            } else {
                                Address::Direct(value as u8)
                            }
                        }
                        Some('@') => Address::Indexed(value as u8),
                        Some(_) => unreachable!(),
                        None => Address::Immediate(value as u8),
                    }
                }
            },
        )
    };

    let separator = || skip_many1(choice([char(' '), char('\t')]));

    let instruction_two_operands = (opcode(), separator(), address(), separator(), address()).map(
        |(opcode, _, left_address, _, right_address)| {
            Instruction::OperandTwo(opcode, left_address, right_address)
        },
    );
    let instruction_one_operand = (opcode(), separator(), address())
        .map(|(opcode, _, address)| Instruction::OperandOne(opcode, address));
    let instruction_no_operand = (opcode()).map(|opcode: Command| Instruction::OperandNone(opcode));

    let instruction =
        try(instruction_two_operands).or(try(instruction_one_operand).or(instruction_no_operand));

    let mut program = sep_end_by(instruction, newline());

    let result: Result<(Vec<Instruction>, &str), ParseError<&str>> = program.parse(input);
    match result {
        Ok((instructions, _remaining_input)) => Ok(instructions),
        Err(error) => panic!("parser error {}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Address, Command, Instruction};

    #[test]
    fn test_empty_input() {
        let input = "";
        let program_result = parse_program(input);
        assert!(program_result.is_ok());
    }

    #[test]
    fn test_nop_input() {
        let input = "nop";
        let program_result = parse_program(input);
        assert_eq!(
            program_result,
            Ok(vec![Instruction::OperandNone(Command::NOP)])
        );
    }

    #[test]
    fn test_nop_trailing_input() {
        let input = "nop\n";
        let program_result = parse_program(input);
        assert_eq!(
            program_result,
            Ok(vec![Instruction::OperandNone(Command::NOP)])
        );
    }

    #[test]
    fn test_psh_a_input() {
        let input = "psh a";
        let program_result = parse_program(input);
        assert_eq!(
            program_result,
            Ok(vec![
                Instruction::OperandOne(Command::PSH, Address::AccumulatorA),
            ])
        );
    }

    #[test]
    fn test_sta_a_imm_input() {
        let input = "sta a 10";
        let program_result = parse_program(input);
        assert_eq!(
            program_result,
            Ok(vec![
                Instruction::OperandTwo(
                    Command::STA,
                    Address::AccumulatorA,
                    Address::Immediate(10),
                ),
            ])
        );
    }

    #[test]
    fn test_sta_a_ind_input() {
        let input = "sta a @11";
        let program_result = parse_program(input);
        assert_eq!(
            program_result,
            Ok(vec![
                Instruction::OperandTwo(Command::STA, Address::AccumulatorA, Address::Indexed(11)),
            ])
        );
    }
}
