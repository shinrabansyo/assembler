use std::fmt;
use std::fmt::Display;

enum Inst {
    Add { rd: u8, rs1: u8, rs2: u8, },
}

impl Into<u64> for Inst {
    fn into(self) -> u64 {
        let s: String = match self {
            Inst::Add { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_001_00001", rs2, rs1, rd),
        };
        let s: String = s.replace("_", "");

        u64::from_str_radix(&s, 2).unwrap()
    }
}

impl Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inst = Into::<u64>::into(Inst::Add { rd: 0, rs1: 0, rs2: 0 });
        let inst_bytes = vec![
            (inst >>  0) & 0b11111111,
            (inst >>  8) & 0b11111111,
            (inst >> 16) & 0b11111111,
            (inst >> 24) & 0b11111111,
            (inst >> 32) & 0b11111111,
            (inst >> 40) & 0b11111111,
        ];
        let inst_s = inst_bytes
            .into_iter()
            .map(|e| format!("{:0>2X}", e))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", inst_s)
    }
}

fn main() {
    vec![
        Inst::Add { rd: 0, rs1: 0, rs2: 0 }
    ]
    .into_iter()
    .for_each(|inst| println!("{}", inst));
}

