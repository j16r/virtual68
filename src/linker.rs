use grammar::Program;
use opcode::Opcode;
use ast::{Instruction, Command, Address};

fn opcode(command: &Command) -> Opcode {
    match *command {
        Command::ABA => Opcode::ABA,
        Command::ADC => Opcode::ADC_A_IMM,
        Command::ADD => Opcode::ADD_A_IMM,
        Command::AND => Opcode::AND_A_IMM,
        Command::ASL => Opcode::ASL_A,
        Command::ASR => Opcode::ASR_A,
        Command::BCC => Opcode::BCC_REL,
        Command::BCS => Opcode::BCS_REL,
        Command::BEQ => Opcode::BEQ_REL,
        Command::BGE => Opcode::BGE_REL,
        Command::BGT => Opcode::BGT_REL,
        Command::BHI => Opcode::BHI_REL,
        Command::BIT => Opcode::BIT_A_IMM,
        Command::BLE => Opcode::BLE_REL,
        Command::BLS => Opcode::BLS_REL,
        Command::BLT => Opcode::BLT_REL,
        Command::BMI => Opcode::BMI_REL,
        Command::BNE => Opcode::BNE_REL,
        Command::BPL => Opcode::BPL_REL,
        Command::BRA => Opcode::BRA_REL,
        Command::BSR => Opcode::BSR_REL,
        Command::BVC => Opcode::BVC_REL,
        Command::BVS => Opcode::BVS_REL,
        Command::CBA => Opcode::CBA,
        Command::CLC => Opcode::CLC,
        Command::CLI => Opcode::CLI,
        Command::CLR => Opcode::CLR_A,
        Command::CLV => Opcode::CLV,
        Command::CMP => Opcode::CMP_A_IMM,
        Command::COM => Opcode::COM_A,
        Command::CPX => Opcode::CPX_A_IMM,
        Command::DAA => Opcode::DAA,
        Command::DEC => Opcode::DEC_A,
        Command::DES => Opcode::DES,
        Command::DEX => Opcode::DEX,
        Command::EOR => Opcode::EOR_A_IMM,
        Command::INC => Opcode::INC_A,
        Command::INS => Opcode::INS,
        Command::INX => Opcode::INX,
        Command::JMP => Opcode::JMP_IND,
        Command::JSR => Opcode::JSR_IND,
        Command::LDA => Opcode::LDA_A_IMM,
        Command::LDS => Opcode::LDS_IMM,
        Command::LDX => Opcode::LDX_IMM,
        Command::LSR => Opcode::LSR_A,
        Command::NEG => Opcode::NEG_A,
        Command::NOP => Opcode::NOP,
        Command::ORA => Opcode::ORA_A_IMM,
        Command::PSH => Opcode::PSH_A,
        Command::PUL => Opcode::PUL_A,
        Command::ROL => Opcode::ROL_A,
        Command::ROR => Opcode::ROR_A,
        Command::RTI => Opcode::RTI,
        Command::RTS => Opcode::RTS,
        Command::SBA => Opcode::SBA,
        Command::SBC => Opcode::SBC_A_IMM,
        Command::SEC => Opcode::SEC,
        Command::SEI => Opcode::SEI,
        Command::SEV => Opcode::SEV,
        Command::STA => Opcode::STA_A_DIR,
        Command::STS => Opcode::STS_DIR,
        Command::STX => Opcode::STX_DIR,
        Command::SUB => Opcode::SUB_A_IMM,
        Command::SWI => Opcode::SWI,
        Command::TAB => Opcode::TAB,
        Command::TAP => Opcode::TAP,
        Command::TBA => Opcode::TBA,
        Command::TPA => Opcode::TPA,
        Command::TST => Opcode::TST_A,
        Command::TSX => Opcode::TSX,
        Command::TXS => Opcode::TXS,
        Command::WAI => Opcode::WAI,
    }
}

const OFFSET_OPCODE_B : u8 = 0x10;
const OFFSET_OPCODE_IND : u8 = 0x20;
const OFFSET_OPCODE_EXT : u8 = 0x30;
const OFFSET_OPCODE_B_ADDR : u8 = 0x40;

pub fn assemble(program: &Program) -> Vec<u8> {
    let mut image = Vec::<u8>::new();
    for instruction in program {
        match *instruction {
            Instruction::OperandNone(ref command) => {
                image.push(opcode(command) as u8);
            }
            Instruction::OperandOne(ref command, ref address) => {
                let mut opcode = opcode(command);
                match *address {
                    Address::AccumulatorA => {
                        image.push(opcode as u8);
                    },
                    Address::AccumulatorB => {
                        image.push(opcode as u8 + OFFSET_OPCODE_B);
                    },
                    Address::Indexed(offset) => {
                        image.push(opcode as u8 + OFFSET_OPCODE_IND);
                        image.push(offset);
                    },
                    Address::Extended(offset) => {
                        image.push(opcode as u8 + OFFSET_OPCODE_EXT);
                        image.push(offset as u8);
                        image.push((offset >> 8) as u8);
                    }
                    _ => panic!("unsupported address type for {:?} instruction", command)
                }
            },
            Instruction::OperandTwo(ref command, ref left_address, ref right_address) => {
                let mut opcode = opcode(command);

                let mut base_offset = 0;
                if let Address::AccumulatorB = *left_address {
                    base_offset += OFFSET_OPCODE_B_ADDR;
                }

                match *right_address {
                    Address::Immediate(offset) => {
                        image.push(opcode as u8 + base_offset);
                        image.push(offset);
                    }
                    Address::Direct(offset) => {
                        image.push(opcode as u8 + base_offset + OFFSET_OPCODE_B);
                        image.push(offset);
                    },
                    Address::Indexed(offset) => {
                        image.push(opcode as u8 + base_offset + OFFSET_OPCODE_IND);
                        image.push(offset);
                    },
                    Address::Extended(offset) => {
                        image.push(opcode as u8 + base_offset + OFFSET_OPCODE_EXT);
                        image.push(offset as u8);
                        image.push((offset >> 8) as u8);
                    }
                    _ => panic!("unsupported address type for {:?} instruction", command)
                }
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
    fn test_clr_ind_program_encodes_to_two_bytes() {
        let mut program = Program::new();
        program.push(Instruction::OperandOne(Command::CLR, Address::Indexed(19)));

        let image = assemble(&program);
        assert_eq!(image.len(), 2);
        assert_eq!(image[0], Opcode::CLR_IND as u8);
        assert_eq!(image[1], 19 as u8);
    }

    #[test]
    fn test_clr_ext_program_encodes_to_two_bytes() {
        let mut program = Program::new();
        program.push(Instruction::OperandOne(Command::CLR, Address::Extended(12347)));

        let image = assemble(&program);
        assert_eq!(image.len(), 3);
        assert_eq!(image[0], Opcode::CLR_EXT as u8);
        assert_eq!(image[1], 59 as u8);
        assert_eq!(image[2], 48 as u8);
    }

    #[test]
    fn test_sub_a_imm_program_encodes_to_two_bytes() {
        let mut program = Program::new();
        program.push(Instruction::OperandTwo(Command::SUB, Address::AccumulatorA, Address::Immediate(231)));

        let image = assemble(&program);
        assert_eq!(image.len(), 2);
        assert_eq!(image[0], Opcode::SUB_A_IMM as u8);
        assert_eq!(image[1], 231 as u8);
    }

    #[test]
    fn test_sub_b_imm_program_encodes_to_two_bytes() {
        let mut program = Program::new();
        program.push(Instruction::OperandTwo(Command::SUB, Address::AccumulatorB, Address::Direct(61)));

        let image = assemble(&program);
        assert_eq!(image.len(), 2);
        assert_eq!(image[0], Opcode::SUB_B_DIR as u8);
        assert_eq!(image[1], 61 as u8);
    }
}
