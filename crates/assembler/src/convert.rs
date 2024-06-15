use std::fmt::Write;

use crate::ir::unresolved::Inst;
use crate::ir::unresolved::InstKind;

pub fn convert(insts: Vec<Inst>) -> anyhow::Result<String> {
    let mut result = String::new();

    for inst in insts {
        let s: String = match inst.kind {
            InstKind::Add { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_001_00001", rs2, rs1, rd),
            InstKind::Sub { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_010_00001", rs2, rs1, rd),
    
            InstKind::Addi { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00010", imm, rs1, rd),
            InstKind::Subi { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_00010", imm, rs1, rd),
    
            // InstKind::Beq { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_000_00011", imm, rs2, rs1, rd),
            // InstKind::Bne { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_001_00011", imm, rs2, rs1, rd),
            // InstKind::Blt { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_010_00011", imm, rs2, rs1, rd),
            // InstKind::Ble { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_011_00011", imm, rs2, rs1, rd),
            // ラベルは一時的にゼロ埋めで対応
            InstKind::Beq { rd, rs1, rs2, .. } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_000_00011", 0, rs2, rs1, rd),
            InstKind::Bne { rd, rs1, rs2, .. } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_001_00011", 0, rs2, rs1, rd),
            InstKind::Blt { rd, rs1, rs2, .. } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_010_00011", 0, rs2, rs1, rd),
            InstKind::Ble { rd, rs1, rs2, .. } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_011_00011", 0, rs2, rs1, rd),
    
            InstKind::Lw  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_00100", imm, rs1, rd),
            InstKind::Lh  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00100", imm, rs1, rd),
            InstKind::Lb  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_00100", imm, rs1, rd),
            InstKind::Lhu { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_011_00100", imm, rs1, rd),
            InstKind::Lbu { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_100_00100", imm, rs1, rd),
    
            InstKind::Sw { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_00101", imm, rs1, rs2),
            InstKind::Sh { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00101", imm, rs1, rs2),
            InstKind::Sb { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_00101", imm, rs1, rs2),
    
            InstKind::In  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_00110", imm, rs1, rd),
            InstKind::Out { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00110", imm, rs1, rs2),
        };
        let s: String = s.replace("_", "");

        let inst_u64 = u64::from_str_radix(&s, 2).unwrap();
        let inst_bytes = vec![
            (inst_u64 >> 0) & 0b11111111,
            (inst_u64 >> 8) & 0b11111111,
            (inst_u64 >> 16) & 0b11111111,
            (inst_u64 >> 24) & 0b11111111,
            (inst_u64 >> 32) & 0b11111111,
            (inst_u64 >> 40) & 0b11111111,
        ];
        let inst_s = inst_bytes
            .into_iter()
            .map(|e| format!("{:0>2X}", e))
            .collect::<Vec<String>>()
            .join("\n");

        writeln!(result, "{}", inst_s)?;
    }

    Ok(result)
}
