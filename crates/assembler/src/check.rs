use crate::ir::unresolved::{Inst, InstKind};

pub fn check(insts: &[Inst]) -> anyhow::Result<()> {
    let labels = insts.iter()
        .filter(|inst| inst.label.is_some())
        .map(|inst| inst.label.clone().unwrap())
        .collect::<Vec<String>>();

    for inst in insts {
        let label = match &inst.kind {
            InstKind::Beq { label, .. } => label,
            InstKind::Bne { label, .. } => label,
            InstKind::Blt { label, .. } => label,
            InstKind::Ble { label, .. } => label,
            _ => continue,
        };
        
        if !labels.contains(label) {
            return Err(anyhow::anyhow!("label {} is not found", label));
        }
    }
    Ok(())
}
