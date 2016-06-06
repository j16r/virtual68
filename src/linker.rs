use ast::Instruction;
use grammar;

pub fn assemble(program: &Vec<Instruction>) -> Vec<u8> {
    let mut image = Vec::<u8>::new();
    for instruction in program {
        use ast::Instruction::*;
        use opcode::Opcode::*;

        match *instruction {
            OperandNone(ref op) => {
                image.push(*op as u8);
            }
            OperandA(ref op) => {
                image.push(*op as u8);
            },
            OperandB(ref op) => {
                image.push(*op as u8);
            },
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
