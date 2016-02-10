use ast::Instruction;
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
    pub fn run(&self) {
        for ins in &self.program {
            match ins {
                 &Instruction::Push(ref reg) => print!("push"),
                 &Instruction::Pop(ref reg) => print!("pop"),
                 &Instruction::Mov(ref lhs, ref rhs) => print!("mov"),
            }
        }
    }
}


