pub enum Register {
    ACCA,
    ACCB,
    X,
    PC,
    SP,
    CC
}

pub enum Opcode {
    Push,
    Pop,
    Mov
}

pub enum Expression {
    Register(Register),
    Value(u8),
}

pub enum Instruction {
    Push(Expression),
    Pop(Expression),
    Mov(Expression, Expression)
}
