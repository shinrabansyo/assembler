#[derive(Debug)]
pub struct Inst {
    pub kind: InstKind,
    pub label: Option<String>,
}

// addi rd = rs1, @label
// addi rd = rs1, $label
// addi rd = rs1, 0x10
#[derive(Debug)]
pub enum InstKind {
    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },

    Addi { rd: u8, rs1: u8, val: Value },
    Subi { rd: u8, rs1: u8, val: Value },

    Beq { rd: u8, rs1: u8, rs2: u8, val: Value },
    Bne { rd: u8, rs1: u8, rs2: u8, val: Value },
    Blt { rd: u8, rs1: u8, rs2: u8, val: Value },
    Ble { rd: u8, rs1: u8, rs2: u8, val: Value },
    Jal { rd: u8, rs1: u8, imm: i32 },

    Lw { rd: u8, rs1: u8, imm: i32 },
    Lh { rd: u8, rs1: u8, imm: i32 },
    Lb { rd: u8, rs1: u8, imm: i32 },
    Lhu { rd: u8, rs1: u8, imm: i32 },
    Lbu { rd: u8, rs1: u8, imm: i32 },

    Sw { rs1: u8, rs2: u8, imm: i32 },
    Sh { rs1: u8, rs2: u8, imm: i32 },
    Sb { rs1: u8, rs2: u8, imm: i32 },

    In { rd: u8, rs1: u8, imm: i32 },
    Out { rs1: u8, rs2: u8, imm: i32 },
    
    And { rd: u8, rs1: u8, rs2: u8 },
    Or { rd: u8, rs1: u8, rs2: u8 },
    Xor { rd: u8, rs1: u8, rs2: u8 },
    Srl { rd: u8, rs1: u8, rs2: u8 },
    Sra { rd: u8, rs1: u8, rs2: u8 },
    Sll { rd: u8, rs1: u8, rs2: u8 },

    Andi { rd: u8, rs1: u8, val: Value },
    Ori { rd: u8, rs1: u8, val: Value },
    Xori { rd: u8, rs1: u8, val: Value },
    Srli { rd: u8, rs1: u8, val: Value },
    Srai { rd: u8, rs1: u8, val: Value },
    Slli { rd: u8, rs1: u8, val: Value },
}

#[derive(Debug)]
pub enum Value {
    DataLabel(String),
    InstLabel(String),
    Imm(i64),
}
