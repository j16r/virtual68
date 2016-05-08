use num::FromPrimitive;

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Opcode {
    UNASSIGNED_00,
    NOP,
    UNASSIGNED_02,
    UNASSIGNED_03,
    UNASSIGNED_04,
    UNASSIGNED_05,
    TAP,
    TPA,
    INX,
    DEX,
    CLV,
    SEV,
    CLC,
    SEC,
    CLI,
    SEI,
    SBA,
    CBA,
    UNASSIGNED_12,
    UNASSIGNED_13,
    UNASSIGNED_14,
    UNASSIGNED_15,
    TAB,
    TBA,
    UNASSIGNED_18,
    DAA,
    UNASSIGNED_1A,
    ABA,
    UNASSIGNED_1C,
    UNASSIGNED_1D,
    UNASSIGNED_1E,
    UNASSIGNED_1F,
    BRA_IND,
    UNASSIGNED_21,
    BHI_IND,
    BLS_IND,
    BCC_IND,
    BCS_IND,
    BNE_IND,
    BEQ_IND,
    BVC_IND,
    BVS_IND,
    BPL_IND,
    BMI_IND,
    BGE_IND,
    BLT_IND,
    BGT_IND,
    BLE_IND,
    TSX,
    INS,
    PUL_A,
    PUL_B,
    DES,
    TXS,
    PSH_A,
    PSH_B,
    UNASSIGNED_38,
    RTS,
    UNASSIGNED_3A,
    RTI,
    UNASSIGNED_3C,
    UNASSIGNED_3D,
    WAI,
    SWI,
    NEG,
    UNASSIGNED_41,
    UNASSIGNED_42,
    COM_A,
    LSR_A,
    UNASSIGNED_45,
    ROR_A,
    ASR_A,
    ASL_A,
    ROL_A,
    DEC_A,
    UNASSIGNED_4B,
    INC_A,
    TST_A,
    UNASSIGNED_4E,
    CLR_A,
    NEG_B,
    UNASSIGNED_51,
    UNASSIGNED_52,
    COM_B,
    LSR_B,
    UNASSIGNED_55,
    ROR_B,
    ASR_B,
    ASL_B,
    ROL_B,
    DEC_B,
    UNASSIGNED_5B,
    INC_B,
    TST_B,
    UNASSIGNED_5E,
    CLR_B,
    NEG_IND,
    UNASSIGNED_61,
    UNASSIGNED_62,
    COM_IND,
    LSR_IND,
    UNASSIGNED_65,
    ROR_IND,
    ASR_IND,
    ASL_IND,
    ROL_IND,
    DEC_IND,
    UNASSIGNED_6B,
    INC_IND,
    TST_IND,
    JMP_IND,
    CLR_IND,
    NEG_EXT,
    UNASSIGNED_71,
    UNASSIGNED_72,
    COM_EXT,
    LSR_EXT,
    UNASSIGNED_75,
    ROR_EXT,
    ASR_EXT,
    ASL_EXT,
    ROL_EXT,
    DEC_EXT,
    UNASSIGNED_7B,
    INC_EXT,
    TST_EXT,
    JMP_EXT,
    CLR_EXT,
    SUB_A,
    CMP_A,
    SBC_A,
    UNASSIGNED_82,
    AND_A,
    BIT_A,
    LDA_A,
    UNASSIGNED_87,
    EOR_A,
    ADC_A,
    ORA_A,
    ADD_A,
    CPX_A,
    BSR_REL,
    LDS_IMM,
    SUB_A_DIR,
    CMP_A_DIR,
    SBC_A_DIR,
    UNASSIGNED_93,
    AND_A_DIR,
    BIT_A_DIR,
    LDA_A_DIR,
    STA_A_DIR,
    DOR_A_DIR,
    ADC_A_DIR,
    ORA_A_DIR,
    ADD_A_DIR,
    CPX_DIR,
    UNASSIGNED_9D,
    LDS_DIR,
    STS_DIR,
    SUB_A_IND,
    CMP_A_IND,
    SBC_A_IND,
    UNASSIGNED_A3,
    AND_A_IND,
    BIT_A_IND,
    LDA_A_IND,
    STA_A_IND,
    DOR_A_IND,
    ADC_A_IND,
    ORA_A_IND,
    ADD_A_IND,
    CPX_IND,
    JSR_IND,
    LDS_IND,
    STS_IND,
    SUB_A_EXT,
    CMP_A_EXT,
    SBC_A_EXT,
    UNASSIGNED_B3,
    AND_A_EXT,
    BIT_A_EXT,
    LDA_A_EXT,
    STA_A_EXT,
    EOR_A_EXT,
    ADC_A_EXT,
    ORA_A_EXT,
    ADD_A_EXT,
    CPX_EXT,
    JSR_EXT,
    LDS_EXT,
    STS_EXT,
    SUB_B_IMM,
    CMP_B_IMM,
    SBC_B_IMM,
    UNASSIGNED_C3,
    AND_B_IMM,
    BIT_B_IMM,
    LDA_B_IMM,
    UNASSIGNED_C7,
    EOR_B_IMM,
    ADC_B_IMM,
    ORA_B_IMM,
    ADD_B_IMM,
    UNASSIGNED_CC,
    UNASSIGNED_CD,
    LDX_IMM,
    UNASSIGNED_CF,
    SUB_B_DIR,
    CMP_B_DIR,
    SBC_B_DIR,
    UNASSIGNED_D3,
    AND_B_DIR,
    BIT_B_DIR,
    LDA_B_DIR,
    STA_B_DIR,
    EOR_B_DIR,
    ADC_B_DIR,
    ORA_B_DIR,
    ADD_B_DIR,
    UNASSIGNED_DC,
    UNASSIGNED_DD,
    LDX_DIR,
    STX_DIR,
    SUB_B_IND,
    CMP_B_IND,
    SBC_B_IND,
    UNASSIGNED_E3,
    AND_B_IND,
    BIT_B_IND,
    LDA_B_IND,
    STA_B_IND,
    EOR_B_IND,
    AOC_B_IND,
    ORA_B_IND,
    ADD_B_IND,
    UNASSIGNED_EC,
    UNASSIGNED_ED,
    LDX_IND,
    STX_IND,
    SUB_B_EXT,
    CMP_B_EX,
    SBC_B_EXT,
    UNASSIGNED_F3,
    AND_B_EXT,
    BIT_B_EXT,
    LDA_B_EXT,
    STA_B_EXT,
    ADC_B_EXT,
    ORA_B_EXT,
    ADD_B_EXT,
    UNASSIGNED_FC,
    UNASSIGNED_FD,
    LDX_EXT,
    STX_EXT,
}
}
