use std::ops::{Index, IndexMut};
use opcode::*;
use enum_primitive::FromPrimitive;
use grammar;
use linker;

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
    stop_offset: u16,
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

impl Machine {
    pub fn load(&mut self, input: &str) {
        let program = grammar::parse_Program(input).unwrap();
        let image = linker::assemble(&program);
        let mut ix = 0;
        for byte in image.iter() {
            self.ram[ix] = *byte;
            ix += 1;
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.ix >= self.stop_offset {
                return
            }

            let opcode = self.ram[self.ix];

            match Opcode::from_i32(opcode as i32).unwrap() {
                Opcode::NOP => {
                    self.ix += 1;
                },
                Opcode::TAP => {
                },
                Opcode::TPA => {
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
                _ => {
                    panic!("unhandled opcode");
                }
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
