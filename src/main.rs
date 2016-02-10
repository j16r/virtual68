mod ast;

use std::path::Path;
use std::fs::File;
use ast::Instruction;

struct Machine {
    // The actual 6800 registers are packed a lot tighter than this!
    registers: [u16; 6], // size_of::<Register>()],
    // The 6800 had 16bit of addressable ram
    ram: [u8; 65536],
    // The program is kept separate to the ram, which is unlike a real 6800, but is much easier to
    // use
    program: Vec<Instruction>,
}

fn main() {
    let args: Vec<_> = ::std::env::args().collect();
    let filename = &args[1];
    let path = &Path::new(&filename);
    let file = File::open(path).unwrap();

    let machine = Machine{
        registers: [0; 6],
        ram: [0; 65536],
        program: Vec::new(),
    };
}
