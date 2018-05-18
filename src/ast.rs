#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Command {
    ABA,
    ADC,
    ADD,
    AND,
    ASL,
    ASR,
    BCC,
    BCS,
    BEQ,
    BGE,
    BGT,
    BHI,
    BIT,
    BLE,
    BLS,
    BLT,
    BMI,
    BNE,
    BPL,
    BRA,
    BSR,
    BVC,
    BVS,
    CBA,
    CLC,
    CLI,
    CLR,
    CLV,
    CMP,
    COM,
    CPX,
    DAA,
    DEC,
    DES,
    DEX,
    EOR,
    INC,
    INS,
    INX,
    JMP,
    JSR,
    LDA,
    LDS,
    LDX,
    LSR,
    NEG,
    NOP,
    ORA,
    PSH,
    PUL,
    ROL,
    ROR,
    RTI,
    RTS,
    SBA,
    SBC,
    SEC,
    SEI,
    SEV,
    STA,
    STS,
    STX,
    SUB,
    SWI,
    TAB,
    TAP,
    TBA,
    TPA,
    TST,
    TSX,
    TXS,
    WAI,
}

#[derive(Debug, PartialEq)]
pub enum Address {
    AccumulatorA,
    AccumulatorB,
    Immediate(u8),
    Indexed(u8),
    Direct(u8),
    Extended(u16),
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    OperandNone(Command),
    OperandOne(Command, Address),
    OperandTwo(Command, Address, Address),
}
