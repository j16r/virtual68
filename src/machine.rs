use std::ops::{Index, IndexMut};
use ast::{Register, Operation, Instruction, Place};
use opcode::*;
use enum_primitive::FromPrimitive;
use grammar;

struct Ram {
    // The 6800 had 16bits of addressable ram
    bytes: [u8; 65536],
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

pub struct Machine {
    // Registers
    acca: u8,
    accb: u8,
    ix: u16,
    pc: u16,
    sp: u16,
    cc: u8,
    ram: Ram,
}

pub fn new(input: &str) -> Machine {
    let program = grammar::parse_Program(input).unwrap();
    Machine{
        acca: 0,
        accb: 0,
        ix: 0,
        pc: 0,
        sp: 0,
        cc: 0,
        ram: Ram{bytes: [0; 65536]},
    }
}

impl Machine {
    pub fn run(&mut self) {
        let opcode = self.ram[self.ix];

        match Opcode::from_i32(opcode as i32).unwrap() {
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
            Opcode::PSH_A => {
                self.ram[self.sp] = self.acca;
                self.sp += 1;
                self.ix += 1;
            },
            Opcode::PSH_B => {
                self.ram[self.sp] = self.accb;
                self.sp += 1;
                self.ix += 1;
            },
            _ => {
                panic!("unhandled opcode");
            }
        }
    }
}


