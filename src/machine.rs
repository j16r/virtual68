use ast::{Register, Operation, Instruction, Place};
use grammar;

pub struct Machine {
    // The actual 6800 registers are packed a lot tighter than this!
    registers: [u16; 6], // size_of::<Register>()],
    // The 6800 had 16bit of addressable ram
    ram: [u8; 65536],
    // The program is kept separate to the ram, which is unlike a real 6800, but is much easier to
    // use
    program: Vec<Instruction>,
}

pub fn new(input: &str) -> Machine {
    let program = grammar::parse_Program(input).unwrap();
    Machine{
        registers: [0; 6],
        ram: [0; 65536],
        program: program,
    }
}

impl Machine {
    pub fn run(&mut self) {
        for ins in &self.program {
            match ins {
                 &Instruction::Push(ref reg) => {
                     println!("push({:?})", reg);
                     match reg {
                        &Place::Register(ref r) => self.ram[0] = self.registers[0] as u8,
                        &Place::Value(ref v) => self.ram[0] = *v,
                     }
                     self.registers[1] += 1;
                 },
                 &Instruction::Pop(ref reg) => println!("pop"),
                 &Instruction::Mov(ref lhs, ref rhs) => println!("mov"),
            }
        }
    }
}


