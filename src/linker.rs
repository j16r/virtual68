use opcode::Opcode;
use ast::{Instruction, Command};

pub fn assemble(program: &Vec<Instruction>) -> Vec<u8> {
    let mut image = Vec::<u8>::new();
    for instruction in program {
        use ast::Instruction::*;

        match *instruction {
            OperandNone(ref command) => {
                let opcode = match *command {
                    Command::ABA => Opcode::ABA,
                    Command::CBA => Opcode::CBA,
                    Command::CLC => Opcode::CLC,
                    Command::CLI => Opcode::CLI,
                    Command::CLV => Opcode::CLV,
                    Command::DAA => Opcode::DAA,
                    Command::DES => Opcode::DES,
                    Command::DEX => Opcode::DEX,
                    Command::INS => Opcode::INS,
                    Command::INX => Opcode::INX,
                    Command::NEG => Opcode::NEG,
                    Command::NOP => Opcode::NOP,
                    Command::RTI => Opcode::RTI,
                    Command::RTS => Opcode::RTS,
                    Command::SBA => Opcode::SBA,
                    Command::SEC => Opcode::SEC,
                    Command::SEI => Opcode::SEI,
                    Command::SEV => Opcode::SEV,
                    Command::SWI => Opcode::SWI,
                    Command::TAB => Opcode::TAB,
                    Command::TAP => Opcode::TAP,
                    Command::TBA => Opcode::TBA,
                    Command::TPA => Opcode::TPA,
                    Command::TSX => Opcode::TSX,
                    Command::TXS => Opcode::TXS,
                    Command::WAI => Opcode::WAI,
                    _ => panic!("invalid command in OperandNone")
                };
                image.push(opcode as u8);
            }
            OperandOne(ref op, ref address) => {
                image.push(*op as u8);
            },
            OperandTwo(ref op, ref left_address, ref right_address) => {
                image.push(*op as u8);
            }
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;
    use grammar;

    #[test]
    fn test_empty_program() {
        let input = "";
        let program = grammar::parse_program(input).unwrap();
        let image = assemble(&program);
        assert_eq!(image.len(), 0);
    }

    #[test]
    fn test_nop_program() {
        let input = "nop";
        let program = grammar::parse_program(input).unwrap();
        let image = assemble(&program);
        assert_eq!(image.len(), 1);
    }
}
