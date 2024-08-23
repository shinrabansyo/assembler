use crate::imem::ir::{unresolved, resolved};
use crate::dmem::ir::Data;
use std::collections::HashMap;

pub fn resolve(datas: &[Data], insts: Vec<unresolved::Inst>) -> anyhow::Result<Vec<resolved::Inst>> {
    let mut data_label_map = HashMap::new();
    let mut current_addr = 0;
    for data in datas {
        if data.label.is_some() {
            data_label_map.insert(data.label.clone().unwrap(), current_addr);
        }
        current_addr += data.command.len();
    }

    let mut inst_label_map = HashMap::new();
    for (idx, inst) in insts.iter().enumerate() { 
        if inst.label.is_some() {
            inst_label_map.insert(inst.label.clone().unwrap(), idx*6);
        }
    }

    let calc_diff = |value: &unresolved::Value, pos: i64| -> i32 {
        if let unresolved::Value::InstLabel(label) = value {
            let imm = (*inst_label_map.get(label).unwrap() as i64) - pos * 6;
            imm as i32
        } else if let unresolved::Value::Imm(imm) = value {
            *imm as i32
        } else {
            unreachable!();
        }
    };
    
    let calc_imm = |value: &unresolved::Value| -> u32 {
        if let unresolved::Value::Imm(imm) = value {
            *imm as u32
        } else if let unresolved::Value::InstLabel(label) = value {
            *inst_label_map.get(label).unwrap() as u32
        } else if let unresolved::Value::DataLabel(label) = value {
            *data_label_map.get(label).unwrap() as u32
        } else {
            unreachable!();
        }
    };

    let mut resolved_insts = Vec::new();
    for (idx, inst) in insts.into_iter().enumerate() { 
        let converted = match inst.kind {
            unresolved::InstKind::Add { rd, rs1, rs2 } => resolved::Inst::Add { rd, rs1, rs2 },
            unresolved::InstKind::Sub { rd, rs1, rs2 } => resolved::Inst::Sub { rd, rs1, rs2 },
            unresolved::InstKind::Addi { rd, rs1, val } => resolved::Inst::Addi { rd, rs1, imm: calc_imm(&val) },
            unresolved::InstKind::Subi { rd, rs1, val } => resolved::Inst::Subi { rd, rs1, imm: calc_imm(&val) },
            unresolved::InstKind::Lw { rd, rs1, imm } => resolved::Inst::Lw { rd, rs1, imm },
            unresolved::InstKind::Lh { rd, rs1, imm } => resolved::Inst::Lh { rd, rs1, imm },
            unresolved::InstKind::Lb { rd, rs1, imm } => resolved::Inst::Lb { rd, rs1, imm },
            unresolved::InstKind::Lhu { rd, rs1, imm } => resolved::Inst::Lhu { rd, rs1, imm },
            unresolved::InstKind::Lbu { rd, rs1, imm } => resolved::Inst::Lbu { rd, rs1, imm },
            unresolved::InstKind::Sw { rs1, rs2, imm } => resolved::Inst::Sw { rs1, rs2, imm },
            unresolved::InstKind::Sh { rs1, rs2, imm } => resolved::Inst::Sh { rs1, rs2, imm },
            unresolved::InstKind::Sb { rs1, rs2, imm } => resolved::Inst::Sb { rs1, rs2, imm },
            unresolved::InstKind::In { rd, rs1, imm } => resolved::Inst::In { rd, rs1, imm },
            unresolved::InstKind::Out { rs1, rs2, imm } => resolved::Inst::Out { rs1, rs2, imm },
            unresolved::InstKind::Beq { rd, rs1, rs2, val } => resolved::Inst::Beq { rd, rs1, rs2, imm: calc_diff(&val, idx as i64) },
            unresolved::InstKind::Ble { rd, rs1, rs2, val } => resolved::Inst::Ble { rd, rs1, rs2, imm: calc_diff(&val, idx as i64) },
            unresolved::InstKind::Blt { rd, rs1, rs2, val } => resolved::Inst::Blt { rd, rs1, rs2, imm: calc_diff(&val, idx as i64) },
            unresolved::InstKind::Bne { rd, rs1, rs2, val } => resolved::Inst::Bne { rd, rs1, rs2, imm: calc_diff(&val, idx as i64) },
            unresolved::InstKind::Jal { rd, rs1, imm } => resolved::Inst::Jal { rd, rs1, imm },
        };
        resolved_insts.push(converted);
    }
    Ok(resolved_insts)
}

/*
// Loop:Setup
label    kind
  ↓       ↓
@setup   addi r2 = r0, 97                0
[noname] addi r3 = r0, 123               6

// Spi:CS
@loop    addi r1 = r0, 1                 12
[noname] out r0[4] = r1                  18

// Spi:Send
[noname] out r0[1] = r2                  24

// Spi:CS
[noname] addi r1 = r0, 0                 30
[noname] out r0[4] = r1                  36

// Loop
[noname] addi r2 = r2, 1                 42
[noname] beq r0, (r2, r3) -> setup       48
[noname] beq r0, (r0, r0) -> loop        54

// ↓みたいに書き換えたい
[noname] beq r0, (r2, r3) -> 0-48       48
[noname] beq r0, (r0, r0) -> 12-54      54
*/
