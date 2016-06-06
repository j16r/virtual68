use opcode::Opcode;

#[derive(Debug)]
pub enum Instruction {
    OperandNone(Opcode),
    OperandA(Opcode),
    OperandB(Opcode),
}
