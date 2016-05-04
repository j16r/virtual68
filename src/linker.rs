use ast::Instruction;
use grammar;

pub fn assemble(program: &Vec<Instruction>) -> Vec<u8> {
    let mut image = Vec::<u8>::new();
    for instruction in program {
        use ast::Instruction::*;
        use ast::Register::*;
        use ast::Place::*;
        use opcode::Opcode::*;

        match *instruction {
            Nop => {
                image.push(NOP as u8);
            }
            Push(Register(ACCA)) => {
                image.push(PSH_A as u8);
            },
            Push(Register(ACCB)) => {
                image.push(PSH_B as u8);
            },
            _ => {}
        }
    }
    image
}

#[test]
fn test_empty_program() {
    let input = "";
    let program = grammar::parse_Program(input).unwrap();
    let image = assemble(&program);
    assert_eq!(image.len(), 0);
}

#[test]
fn test_nop_program() {
    let input = "nop";
    let program = grammar::parse_Program(input).unwrap();
    let image = assemble(&program);
    assert_eq!(image.len(), 1);
}
