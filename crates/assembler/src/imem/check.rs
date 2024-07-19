use crate::imem::ir::unresolved::{Inst, InstKind, Value};

pub fn check(insts: &[Inst]) -> anyhow::Result<()> {
    check_instlabel(insts)?;
    check_label_usage(insts)?;
    check_value_range(insts)?;
    Ok(()) 
}

// 宣言されていないラベルを呼び出ししていたらエラー
fn check_instlabel(insts: &[Inst]) -> anyhow::Result<()> {
    let inst_labels = insts.iter()
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
        
        if let Value::InstLabel(label) = val {
            if !inst_labels.contains(label) {
                return Err(anyhow::anyhow!("label {} is not found", label));
            }
        }
    }
    Ok(())
}

fn check_label_usage(insts: &[Inst]) -> anyhow::Result<()> {
    for inst in insts {
        let is_collect_usage = match &inst.kind {
            InstKind::Beq { val, .. } => !matches!(val, Value::DataLabel(_)),
            InstKind::Bne { val, .. } => !matches!(val, Value::DataLabel(_)),
            InstKind::Blt { val, .. } => !matches!(val, Value::DataLabel(_)),
            InstKind::Ble { val, .. } => !matches!(val, Value::DataLabel(_)),
            _ => continue,
        };
        if !is_collect_usage {
            return Err(anyhow::anyhow!("Datalabel is not permitted in branch instruction {:?}", inst));
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
            *imm >= ((1 << 24) as i64) || *imm < (-(1 << 24) as i64) 
        } else {
            true
        }
    };

    for inst in insts {
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
