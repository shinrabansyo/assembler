use crate::dmem::ir::Data;
use crate::imem::ir::unresolved::{Inst, InstKind, Value};

pub fn check(datas: &[Data], insts: &[Inst]) -> anyhow::Result<()> {
    check_label_exists(datas, insts)?;
    check_label_usage(insts)?;
    check_reg_range(insts)?;
    check_value_range(insts)?;

    Ok(())
}

// 宣言されていないラベルを呼び出ししていたらエラー
fn check_label_exists(datas: &[Data], insts: &[Inst]) -> anyhow::Result<()> {
    let data_labels = datas
        .iter()
        .filter(|data| data.label.is_some())
        .map(|data| data.label.clone().unwrap())
        .collect::<Vec<String>>();

    let inst_labels = insts
        .iter()
        .filter(|inst| inst.label.is_some())
        .map(|inst| inst.label.clone().unwrap())
        .collect::<Vec<String>>();

    for inst in insts {
        // val に 呼び出してる val のリストを入れてる
        let val = match &inst.kind {
            InstKind::Addi { val, .. } => val,
            InstKind::Subi { val, .. } => val,
            InstKind::Beq { val, .. } => val,
            InstKind::Bne { val, .. } => val,
            InstKind::Blt { val, .. } => val,
            InstKind::Ble { val, .. } => val,
            _ => continue,
        };

        match val {
            Value::DataLabel(label) => {
                if !data_labels.contains(label) {
                    return Err(anyhow::anyhow!("label {} is not found", label));
                }
            }
            Value::InstLabel(label) => {
                if !inst_labels.contains(label) {
                    return Err(anyhow::anyhow!("label {} is not found", label));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn check_label_usage(insts: &[Inst]) -> anyhow::Result<()> {
    for inst in insts {
        #[rustfmt::skip]
        let is_collect_usage = match &inst.kind {
            InstKind::Beq { val, .. } => !matches!(val, Value::DataLabel(_)),
            InstKind::Bne { val, .. } => !matches!(val, Value::DataLabel(_)),
            InstKind::Blt { val, .. } => !matches!(val, Value::DataLabel(_)),
            InstKind::Ble { val, .. } => !matches!(val, Value::DataLabel(_)),
            _ => continue,
        };
        if !is_collect_usage {
            return Err(anyhow::anyhow!(
                "Datalabel is not permitted in branch instruction {:?}",
                inst
            ));
        }
    }
    Ok(())
}

fn check_reg_range(insts: &[Inst]) -> anyhow::Result<()> {
    // I形式：rd: 0-31, rs1: 0-7
    // R/B/S形式: rd/rs1/rs2: 0-31

    let check_i_type = |rd: &u8, rs1: &u8| -> bool { *rd <= 31 && *rs1 <= 7 };
    let check_s_type = |rs1: &u8, rs2: &u8| -> bool { *rs1 <= 7 && *rs2 <= 31 };
    let check_other_type =
        |rd: &u8, rs1: &u8, rs2: &u8| -> bool { *rd <= 31 && *rs1 <= 31 && *rs2 <= 31 };

    for inst in insts {
        #[rustfmt::skip]
        let is_correct_reg = match &inst.kind {
            // I-type
            InstKind::Addi { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Subi { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Jal  { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Lw   { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Lh   { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Lb   { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Lhu  { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Lbu  { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::In   { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Andi { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Ori { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Xori { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Srli { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Srai { rd, rs1, .. } => check_i_type(rd, rs1),
            InstKind::Slli { rd, rs1, .. } => check_i_type(rd, rs1),

            // S-type
            InstKind::Sw  { rs1, rs2, .. } => check_s_type(rs1, rs2),
            InstKind::Sh  { rs1, rs2, .. } => check_s_type(rs1, rs2),
            InstKind::Sb  { rs1, rs2, .. } => check_s_type(rs1, rs2),
            InstKind::Out { rs1, rs2, .. } => check_s_type(rs1, rs2),
            InstKind::Isb { rs1, rs2, .. } => check_s_type(rs1, rs2),

            // R-type
            InstKind::Add { rd, rs1, rs2 } => check_other_type(rd, rs1, rs2),
            InstKind::Sub { rd, rs1, rs2 } => check_other_type(rd, rs1, rs2),
            InstKind::And { rd, rs1, rs2 } => check_other_type(rd, rs1, rs2),
            InstKind::Or { rd, rs1, rs2 } => check_other_type(rd, rs1, rs2),
            InstKind::Xor { rd, rs1, rs2 } => check_other_type(rd, rs1, rs2),
            InstKind::Srl { rd, rs1, rs2 } => check_other_type(rd, rs1, rs2),
            InstKind::Sra { rd, rs1, rs2 } => check_other_type(rd, rs1, rs2),
            InstKind::Sll { rd, rs1, rs2 } => check_other_type(rd, rs1, rs2),

            // B-type
            InstKind::Beq { rd, rs1, rs2, .. } => check_other_type(rd, rs1, rs2),
            InstKind::Bne { rd, rs1, rs2, .. } => check_other_type(rd, rs1, rs2),
            InstKind::Blt { rd, rs1, rs2, .. } => check_other_type(rd, rs1, rs2),
            InstKind::Ble { rd, rs1, rs2, .. } => check_other_type(rd, rs1, rs2),
        };
        if !is_correct_reg {
            return Err(anyhow::anyhow!("Invalid register usage: {:?}", inst));
        }
    }
    Ok(())
}

fn check_value_range(insts: &[Inst]) -> anyhow::Result<()> {
    let is_imm_u32 = |val: &Value| -> bool {
        if let Value::Imm(imm) = val {
            *imm >= u32::MIN as i64 && *imm <= u32::MAX as i64
        } else {
            true
        }
    };

    let is_imm_i25 = |val: &Value| -> bool {
        if let Value::Imm(imm) = val {
            *imm >= (-(1 << 24) as i64) || *imm < ((1 << 24) as i64)
        } else {
            true
        }
    };

    for inst in insts {
        #[rustfmt::skip]
        let is_correct_imm = match &inst.kind {
            InstKind::Addi { val, .. } => is_imm_u32(val),
            InstKind::Subi { val, .. } => is_imm_u32(val),
            InstKind::Beq { val, .. } => is_imm_i25(val),
            InstKind::Bne { val, .. } => is_imm_i25(val),
            InstKind::Blt { val, .. } => is_imm_i25(val),
            InstKind::Ble { val, .. } => is_imm_i25(val),
            _ => continue,
        };
        if !is_correct_imm {
            return Err(anyhow::anyhow!("Imm is overflow: {:?}", inst));
        }
    }
    Ok(())
}
