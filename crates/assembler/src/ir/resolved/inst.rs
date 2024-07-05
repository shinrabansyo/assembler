pub enum Inst {
    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },

    Addi { rd: u8, rs1: u8, imm: u32 },
    Subi { rd: u8, rs1: u8, imm: u32 },

    Beq { rd: u8, rs1: u8, rs2: u8, imm: i32 },
    Bne { rd: u8, rs1: u8, rs2: u8, imm: i32 },
    Blt { rd: u8, rs1: u8, rs2: u8, imm: i32 },
    Ble { rd: u8, rs1: u8, rs2: u8, imm: i32 },

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
}


