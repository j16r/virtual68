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
    Register,
    Value(u8),
}

pub enum Instruction {
    push(Register),
    pop(Register),
    mov(Expression, Expression)
}
