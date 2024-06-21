use crate::ir::{unresolved, resolved};
use crate::ir::unresolved::InstKind;
use std::collections::HashMap;

pub fn resolve(insts: Vec<unresolved::Inst>) -> anyhow::Result<Vec<resolved::Inst>> {
    let mut label_map = HashMap::new();
    for (idx, inst) in insts.iter().enumerate() {
        // let label = match &inst.kind {
        //     InstKind::Beq { label, .. } => label,
        //     InstKind::Bne { label, .. } => label,
        //     InstKind::Blt { label, .. } => label,
        //     InstKind::Ble { label, .. } => label,
        //     _ => continue,
        // };
        if inst.label.is_some() {
            label_map.insert(inst.label.clone().unwrap(), idx*6);
        }
    }



    unimplemented!()
}


/*
let mut map = ...;
map.insert(K, V);

.is_some()
.is_none()
*/


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
