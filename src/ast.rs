#[derive(Debug)]
pub enum Register {
    ACCA,
    ACCB,
    X,
    PC,
    SP,
    CC
}

#[derive(Debug)]
pub enum Operation {
    Push,
    Pop,
    Mov
}

#[derive(Debug)]
pub enum Place {
    Register(Register),
    Value(u8),
}

#[derive(Debug)]
pub enum Instruction {
    Push(Place),
    Pop(Place),
    Mov(Place, Place)
}
