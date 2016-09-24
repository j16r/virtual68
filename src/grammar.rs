use pest::prelude::*;

use ast::Instruction;
use ast::Instruction::*;

impl_rdp! {
    grammar! {
      expression = _{ paren ~ expression? }
      paren      =  { ["("] ~ expression? ~ [")"] }
    }

    process! {
        program(&self) -> Vec<Instruction> {
        }
    }
}

pub fn parse_program(input: &str) -> &Vec<Instruction> {
    Rdp::new(StringInput::new(input)).program()
}
