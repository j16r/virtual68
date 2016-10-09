use grammar::Program;
use opcode::Opcode;
use ast::{Instruction, Command, Address};

pub fn assemble(program: &Program) -> Vec<u8> {
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
            OperandOne(ref command, ref address) => {
                match *command {
                    Command::ADC => {
                        match *address {
                            Address::AccumulatorA => {
                                image.push(Opcode::ADC_A as u8);
                            },
                            _ => panic!("unsupported address type for adc instruction")
                        }
                    },
                    Command::ADD => {
                        match *address {
                            Address::AccumulatorA => {
                                image.push(Opcode::ADD_A as u8);
                            },
                            _ => panic!("unsupported address type for add instruction")
                        }
                    },
                    Command::AND => {
                        match *address {
                            Address::AccumulatorA => {
                                image.push(Opcode::AND_A as u8);
                            },
                            _ => panic!("unsupported address type for and instruction")
                        }
                    },
                    Command::ASL => {
                        match *address {
                            Address::AccumulatorA => {
                                image.push(Opcode::ASL_A as u8);
                            },
                            Address::AccumulatorB => {
                                image.push(Opcode::ASL_B as u8);
                            },
                            Address::Indexed(offset) => {
                                image.push(Opcode::ASL_IND as u8);
                                image.push(offset);
                            },
                            Address::Extended(offset) => {
                                image.push(Opcode::ASL_EXT as u8);
                                image.push(offset as u8);
                                image.push((offset >> 8) as u8);
                            }
                            _ => panic!("unsupported address type for asl instruction")
                        }
                    },
                    Command::ASR => {
                    },
                    Command::BCC => {
                    },
                    Command::BCS => {
                    },
                    Command::BEQ => {
                    },
                    Command::BGE => {
                    },
                    Command::BGT => {
                    },
                    Command::BHI => {
                    },
                    Command::BIT => {
                    },
                    Command::BLE => {
                    },
                    Command::BLS => {
                    },
                    Command::BLT => {
                    },
                    Command::BMI => {
                    },
                    Command::BNE => {
                    },
                    Command::BPL => {
                    },
                    Command::BRA => {
                    },
                    Command::BSR => {
                    },
                    Command::BVC => {
                    },
                    Command::BVS => {
                    },
                    Command::CLR => {
                        match *address {
                            Address::AccumulatorA => {
                                image.push(Opcode::CLR_A as u8);
                            },
                            Address::AccumulatorB => {
                                image.push(Opcode::CLR_B as u8);
                            },
                            Address::Indexed(offset) => {
                                image.push(Opcode::CLR_IND as u8);
                                image.push(offset);
                            },
                            Address::Extended(offset) => {
                                image.push(Opcode::CLR_EXT as u8);
                                image.push(offset as u8);
                                image.push((offset >> 8) as u8);
                            }
                            _ => panic!("unsupported address type for clr instruction")
                        }
                    },
                    Command::CMP => {
                    },
                    Command::COM => {
                    },
                    Command::CPX => {
                    },
                    Command::DEC => {
                    },
                    Command::EOR => {
                    },
                    Command::INC => {
                    },
                    Command::JMP => {
                    },
                    Command::JSR => {
                    },
                    Command::LDA => {
                    },
                    Command::LDS => {
                    },
                    Command::LDX => {
                    },
                    Command::LSR => {
                    },
                    Command::NEG => {
                        match *address {
                            Address::AccumulatorA => {
                                image.push(Opcode::NEG_A as u8);
                            },
                            Address::AccumulatorB => {
                                image.push(Opcode::NEG_B as u8);
                            },
                            Address::Indexed(offset) => {
                                image.push(Opcode::NEG_IND as u8);
                                image.push(offset);
                            },
                            Address::Extended(offset) => {
                                image.push(Opcode::NEG_EXT as u8);
                                image.push(offset as u8);
                                image.push((offset >> 8) as u8);
                            }
                            _ => panic!("unsupported address type for neg instruction")
                        }
                    },
                    Command::ORA => {
                    },
                    Command::PSH => {
                    },
                    Command::PUL => {
                    },
                    Command::ROL => {
                    },
                    Command::ROR => {
                        match *address {
                            Address::AccumulatorA => {
                                image.push(Opcode::ROR_A as u8);
                            },
                            Address::AccumulatorB => {
                                image.push(Opcode::ROR_B as u8);
                            },
                            Address::Indexed(offset) => {
                                image.push(Opcode::ROR_IND as u8);
                                image.push(offset);
                            },
                            Address::Extended(offset) => {
                                image.push(Opcode::ROR_EXT as u8);
                                image.push(offset as u8);
                                image.push((offset >> 8) as u8);
                            }
                            _ => panic!("unsupported address type for ror instruction")
                        }
                    },
                    Command::SBC => {
                    },
                    Command::STS => {
                    },
                    Command::STX => {
                    },
                    Command::SUB => {
                    },
                    Command::TST => {
                    },
                    _ => panic!("invalid command in OperandOne")
                };
            },
            OperandTwo(ref command, ref left_address, ref right_address) => {
                image.push(*command as u8);
            }
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Instruction, Command, Address};
    use grammar::Program;
    use opcode::Opcode;

    #[test]
    fn test_empty_program() {
        let mut program = Program::new();
        let image = assemble(&program);
        assert_eq!(image.len(), 0);
    }

    #[test]
    fn test_nop_program_encodes_to_byte() {
        let mut program = Program::new();
        program.push(Instruction::OperandNone(Command::NOP));

        let image = assemble(&program);
        assert_eq!(image.len(), 1);
        assert_eq!(image[0], Opcode::NOP as u8);
    }

    #[test]
    fn test_ror_program_encodes_to_byte() {
        let mut program = Program::new();
        program.push(Instruction::OperandOne(Command::ROR, Address::AccumulatorB));

        let image = assemble(&program);
        assert_eq!(image.len(), 1);
        assert_eq!(image[0], Opcode::ROR_B as u8);
    }

    #[test]
    fn test_clr_program_encodes_to_two_bytes() {
        let mut program = Program::new();
        program.push(Instruction::OperandOne(Command::CLR, Address::Indexed(19)));

        let image = assemble(&program);
        assert_eq!(image.len(), 2);
        assert_eq!(image[0], Opcode::CLR_IND as u8);
        assert_eq!(image[1], 19 as u8);
    }
}
