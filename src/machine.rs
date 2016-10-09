use std::fmt;
use std::ops::{Index, IndexMut};

use opcode::*;
use enum_primitive::FromPrimitive;
use grammar;
use linker;
 
struct Ram {
    // The 6800 had 16bits of addressable ram
    bytes: [u8; 65536],
}

impl fmt::Debug for Ram {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.bytes[..].fmt(formatter)
    }
}

impl Index<u16> for Ram {
    type Output = u8;

    fn index<'a>(&'a self, i: u16) -> &'a u8 {
        self.bytes.index(i as usize)
    }
}

impl IndexMut<u16> for Ram {
    fn index_mut<'a>(&'a mut self, i: u16) -> &'a mut u8 {
        self.bytes.index_mut(i as usize)
    }
}

#[derive(Debug)]
pub struct Machine {
    // Registers
    acca: u8,
    accb: u8,
    ix: u16,
    pc: u16,
    sp: u16,
    cc: u8,
    ram: Ram,
    stop_offset: u16,
}

impl fmt::Display for Machine {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let output = format!(
            "<Machine A:{} ({:#x}) B:{} ({:#x}) IX:{:#x} PC:{:#x} SP:{:#x} CC:{:#x}>",
            self.acca,
            self.acca,
            self.accb,
            self.accb,
            self.ix,
            self.pc,
            self.sp,
            self.cc);
        formatter.write_str(&output)
    }
}

pub fn new() -> Machine {
    Machine{
        acca: 0,
        accb: 0,
        ix: 0,
        pc: 0,
        sp: 65535,
        cc: 0,
        ram: Ram{bytes: [0; 65536]},
        stop_offset: 0,
    }
}

//impl<T: fmt::Debug> fmt::Debug for Machine {
    //fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //self.data[..].fmt(formatter)
    //}
//}

impl Machine {
    pub fn load(&mut self, input: &str) {
        let program = grammar::parse_program(input).unwrap();
        let image = linker::assemble(&program);
        let mut ix = 0;
        for byte in image.iter() {
            self.ram[ix] = *byte;
            ix += 1;
        }
        println!("stop_offset = {:?}", ix);
        self.stop_offset = ix;
    }

    pub fn run(&mut self) {
        loop {
            println!("{}", self);
            if self.ix >= self.stop_offset {
                return
            }

            let opcode = Opcode::from_i32(self.ram[self.ix] as i32).unwrap();
            match opcode {
                Opcode::UNASSIGNED_00 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::NOP => {
                    self.ix += 1;
                },
                Opcode::UNASSIGNED_02 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_03 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_04 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_05 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::TAP => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::TPA => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::INX => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::DEX => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CLV => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SEV => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CLC => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SEC => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CLI => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SEI => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SBA => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CBA => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_12 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_13 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_14 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_15 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::TAB => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::TBA => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_18 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::DAA => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_1A => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::ABA => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_1C => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_1D => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_1E => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_1F => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::BRA_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_21 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::BHI_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BLS_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BCC_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BCS_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BNE_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BEQ_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BVC_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BVS_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BPL_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BMI_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BGE_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BLT_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BGT_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BLE_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::TSX => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::INS => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::PUL_A => {
                    self.acca = self.ram[self.sp];
                    self.sp += 1;
                    self.ix += 1;
                },
                Opcode::PUL_B => {
                    self.accb = self.ram[self.sp];
                    self.sp += 1;
                    self.ix += 1;
                },
                Opcode::DES => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::TXS => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::PSH_A => {
                    self.ram[self.sp] = self.acca;
                    self.sp -= 1;
                    self.ix += 1;
                },
                Opcode::PSH_B => {
                    self.ram[self.sp] = self.accb;
                    self.sp -= 1;
                    self.ix += 1;
                },
                Opcode::UNASSIGNED_38 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::RTS => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_3A => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::RTI => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_3C => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_3D => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::WAI => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SWI => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::NEG_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_41 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_42 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::COM_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LSR_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_45 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::ROR_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ASR_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ASL_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ROL_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::DEC_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_4B => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::INC_A => {
                    self.acca += 1;
                    self.ix += 1;
                },
                Opcode::TST_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_4E => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::CLR_A => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::NEG_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_51 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_52 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::COM_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LSR_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_55 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::ROR_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ASR_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ASL_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ROL_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::DEC_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_5B => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::INC_B => {
                    self.accb += 1;
                    self.ix += 1;
                },
                Opcode::TST_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_5E => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::CLR_B => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::NEG_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_61 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_62 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::COM_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LSR_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_65 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::ROR_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ASR_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ASL_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ROL_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::DEC_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_6B => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::INC_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::TST_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::JMP_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CLR_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::NEG_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_71 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_72 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::COM_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LSR_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_75 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::ROR_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ASR_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ASL_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ROL_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::DEC_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_7B => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::INC_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::TST_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::JMP_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CLR_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SUB_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CMP_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SBC_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_83 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::AND_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BIT_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDA_A_IMM => {
                    self.acca = self.ram[self.ix + 1];
                    self.ix += 2;
                },
                Opcode::UNASSIGNED_87 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::EOR_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADC_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ORA_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADD_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CPX_A_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BSR_REL => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDS_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_8F => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::SUB_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CMP_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SBC_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_93 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::AND_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BIT_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDA_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STA_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::EOR_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADC_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ORA_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADD_A_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CPX_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_9D => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::LDS_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STS_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SUB_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CMP_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SBC_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_A3 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::AND_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BIT_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDA_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STA_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::EOR_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADC_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ORA_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADD_A_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CPX_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::JSR_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDS_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STS_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SUB_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CMP_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SBC_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_B3 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::AND_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BIT_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDA_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STA_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::EOR_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADC_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ORA_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADD_A_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CPX_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::JSR_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDS_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STS_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SUB_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CMP_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SBC_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_C3 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::AND_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BIT_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDA_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_C7 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::EOR_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADC_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ORA_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADD_B_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_CC => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_CD => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::LDX_IMM => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_CF => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::SUB_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CMP_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SBC_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_D3 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::AND_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BIT_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDA_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STA_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::EOR_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADC_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ORA_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADD_B_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_DC => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_DD => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::LDX_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STX_DIR => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SUB_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CMP_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SBC_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_E3 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::AND_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BIT_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDA_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STA_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::EOR_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADC_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ORA_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADD_B_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_EC => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_ED => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::LDX_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STX_IND => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SUB_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::CMP_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::SBC_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_F3 => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::AND_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::BIT_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::LDA_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STA_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADC_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADC_B_EXT_DUP => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ORA_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::ADD_B_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::UNASSIGNED_FC => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::UNASSIGNED_FD => {
                    panic!("unassigned opcode {:?}", opcode);
                },
                Opcode::LDX_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
                Opcode::STX_EXT => {
                    panic!("{:?} opcode not implemented yet", opcode);
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use opcode::Opcode;
    use machine;

    #[test]
    fn run_nop() {
        let mut machine = machine::new();
        machine.ram[0] = Opcode::NOP as u8;
        machine.stop_offset = 1;
        machine.run();
        assert!(machine.ix == 1);
        assert!(machine.sp == 65535);
    }

    #[test]
    fn run_push() {
        let mut machine = machine::new();
        machine.ram[0] = Opcode::PSH_A as u8;
        machine.stop_offset = 1;
        machine.run();
        assert!(machine.ix == 1);
        assert!(machine.sp == 65534);
    }

    #[test]
    fn run_push_pop() {
        let mut machine = machine::new();
        machine.ram[0] = Opcode::PSH_A as u8;
        machine.ram[1] = Opcode::PUL_A as u8;
        machine.stop_offset = 2;
        machine.run();
        assert!(machine.ix == 2);
        assert!(machine.sp == 65535);
    }
}
