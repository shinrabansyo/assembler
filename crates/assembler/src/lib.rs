use std::fmt;
use std::fmt::Display;

pub fn assemble(insts: Vec<Inst>) -> String {
    insts
        .into_iter()
        .map(|inst| format!("{}", inst))
        .collect::<Vec<String>>()
        .join("\n")
}

#[macro_export]
macro_rules! assembly {
    // add, addi, ...
    ($kind:ident $rd:ident = $rs1:expr, $rs2:expr) => {
        Inst::try_from((stringify!($kind), vec![stringify!($rd), stringify!($rs1), stringify!($rs2)])).unwrap()
    };

    // beq, ...
    ($kind:ident $rd:ident, ($rs1:ident, $rs2:ident) -> $imm:expr) => {
        Inst::try_from((stringify!($kind), vec![stringify!($rd), stringify!($rs1), stringify!($rs2), stringify!($imm)])).unwrap()
    };

    // lw, ..., in
    ($kind:ident $rd:ident = $rs1:ident[$imm:expr]) => {
        Inst::try_from((stringify!($kind), vec![stringify!($rd), stringify!($rs1), stringify!($imm)])).unwrap()
    };

    // sw, ..., out
    ($kind:ident $rs1:ident[$imm:expr] = $rs2:ident) => {
        Inst::try_from((stringify!($kind), vec![stringify!($rs1), stringify!($imm), stringify!($rs2)])).unwrap()
    };
}

#[allow(dead_code)]
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

impl TryFrom<(&str, Vec<&str>)> for Inst {
    type Error = anyhow::Error;

    #[rustfmt::skip]
    fn try_from((kind, args): (&str, Vec<&str>)) -> anyhow::Result<Self> {
        let args = args
            .into_iter()
            .map(|arg| arg.replace("r", ""))
            .map(|arg| arg.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        match kind {
            "add" => Ok(Inst::Add { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8 }),
            "sub" => Ok(Inst::Sub { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8 }),

            "addi" => Ok(Inst::Addi { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] as u32 }),
            "subi" => Ok(Inst::Subi { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] as u32 }),

            "beq" => Ok(Inst::Beq { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8, imm: args[3] }),
            "bne" => Ok(Inst::Bne { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8, imm: args[3] }),
            "blt" => Ok(Inst::Blt { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8, imm: args[3] }),
            "ble" => Ok(Inst::Ble { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8, imm: args[3] }),

            "lw" => Ok(Inst::Lw { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] }),
            "lh" => Ok(Inst::Lh { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] }),
            "lb" => Ok(Inst::Lb { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] }),
            "lhu" => Ok(Inst::Lhu { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] }),
            "lbu" => Ok(Inst::Lbu { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] }),

            "sw" => Ok(Inst::Sw { rs1: args[0] as u8, imm: args[1], rs2: args[2] as u8 }),
            "sh" => Ok(Inst::Sh { rs1: args[0] as u8, imm: args[1], rs2: args[2] as u8 }),
            "sb" => Ok(Inst::Sb { rs1: args[0] as u8, imm: args[1], rs2: args[2] as u8 }),

            "in" => Ok(Inst::In { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] }),
            "out" => Ok(Inst::Out { rs1: args[0] as u8, imm: args[1], rs2: args[2] as u8 }),

           _ => Err(anyhow::anyhow!("Invalid instruction: {}", kind)),
        }
    }
}

impl TryFrom<&str> for Inst {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Inst> {
        let (lhs, rhs) = if s.contains("->") {
            // beq, ...
            let splitted_by_arrow = s.split("->").collect::<Vec<_>>();
            (splitted_by_arrow[0], splitted_by_arrow[1])
        } else {
            // add, lw, sw, in, out, ...
            let splitted_by_eq = s.split("=").collect::<Vec<_>>();
            (splitted_by_eq[0], splitted_by_eq[1])
        };
        let lhs = lhs.split_ascii_whitespace().collect::<Vec<_>>();
        let (kind, lhs) = (lhs[0], lhs[1..].concat());

        // beq
        if lhs.contains("(") {
            let rhs = rhs.trim();
            let lhs = lhs
                .split(",")
                .map(|e| e.trim())
                .collect::<Vec<_>>();
            // beq r0, (r0, r0) -> -42
            // beq | r0, (r0, r0) | -42
            // beq | r0, (r0, r0) | -42
            // beq | r0 | (r0 | r0) | -42
            return Inst::try_from((kind, vec![lhs[0], &lhs[1].replace("(", ""), &lhs[2].replace(")", ""), rhs]));
        }

        // sw, ..., out
        if lhs.contains("[") {
            let rhs = rhs.trim();
            let lhs = lhs
                .split("[")
                .map(|e| e.trim())
                .collect::<Vec<_>>();
            // sw r0[4] = r7
            // sw r0[4] | r7
            // sw r0 | 4] | r7
            return Inst::try_from((kind, vec![lhs[0], &lhs[1].replace("]", ""), rhs]));
        }

        // lw, ..., in
        if rhs.contains("[") {
            let lhs = lhs.trim();
            let rhs = rhs
                .split("[")
                .map(|e| e.trim())
                .collect::<Vec<_>>();
            // lw r7 = r0[4]
            // lw | r7 | r0[4]
            // lw | r7 | r0 | 4]
            return Inst::try_from((kind, vec![lhs, rhs[0], &rhs[1].replace("]", "")]));
        }

        // アセンブリ言語例
        // assembly!(out r0[4] = r1),
        // assembly!(addi r1 = r0, 1),                // 00 (00)
        // assembly!(beq r0, (r0, r0) -> -42),
        // assembly!(lw r7 = r0[4]),               // 06 (06)
        // assembly!(sw r0[0] = r7),               // 12 (0C)

        // add, addi, ...
        let lhs = lhs.trim();
        let rhs = rhs
            .split(",")
            .map(|e| e.trim())
            .collect::<Vec<_>>();

        Inst::try_from((kind, vec![lhs, rhs[0], rhs[1]]))
    }
}

impl TryFrom<u64> for Inst {
    type Error = anyhow::Error;

    #[rustfmt::skip]
    fn try_from(inst: u64) -> anyhow::Result<Inst> {
        let opcode     = ((inst >> 0) & 0x1F) as u8;
        let opcode_sub = ((inst >> 5) & 0x07) as u8;
        let rd         = ((inst >> (5+3)) & 0x1F) as u8;
        let rs1_r      = ((inst >> (5+3+5)) & 0x1F) as u8;
        let rs1_i_s    = rs1_r & 0x07;
        let rs2        = rd;
        let imm        = ((inst >> (5+3+5+3)) & 0xFFFFFFFF) as u32;
        let _reserved  = ((inst >> (5+3+5+5+5)) & 0x1FFFFFF) as u32;

        match opcode {
            0x00 => {
                match opcode_sub {
                    0x00 => Ok(Inst::Add { rd: 0, rs1: 0, rs2: 0 }),
                    _ => Err(anyhow::anyhow!("Invalid opcode_sub")),
                }
            },
            0x01 => {
                match opcode_sub {
                    0x01 => Ok(Inst::Add { rd, rs1: rs1_r, rs2 }),
                    0x02 => Ok(Inst::Sub { rd, rs1: rs1_r, rs2 }),
                    _ => Err(anyhow::anyhow!("Invalid opcode_sub")),
                }
            },
            0x02 => {
                match opcode_sub {
                    0x01 => Ok(Inst::Addi { rd, rs1: rs1_i_s, imm }),
                    0x02 => Ok(Inst::Subi { rd, rs1: rs1_i_s, imm }),
                    _ => Err(anyhow::anyhow!("Invalid opcode_sub")),
                }
            },
            0x03 => {
                match opcode_sub {
                    0x00 => Ok(Inst::Beq { rd, rs1: rs1_r, rs2, imm: imm as i32 }),
                    0x01 => Ok(Inst::Bne { rd, rs1: rs1_r, rs2, imm: imm as i32 }),
                    0x02 => Ok(Inst::Blt { rd, rs1: rs1_r, rs2, imm: imm as i32 }),
                    0x03 => Ok(Inst::Ble { rd, rs1: rs1_r, rs2, imm: imm as i32 }),
                    _ => Err(anyhow::anyhow!("Invalid opcode_sub")),
                }
            },
            0x04 => {
                match opcode_sub {
                    0x00 => Ok(Inst::Lw  { rd, rs1: rs1_i_s, imm: imm as i32 }),
                    0x01 => Ok(Inst::Lh  { rd, rs1: rs1_i_s, imm: imm as i32 }),
                    0x02 => Ok(Inst::Lb  { rd, rs1: rs1_i_s, imm: imm as i32 }),
                    0x03 => Ok(Inst::Lhu { rd, rs1: rs1_i_s, imm: imm as i32 }),
                    0x04 => Ok(Inst::Lbu { rd, rs1: rs1_i_s, imm: imm as i32 }),
                    _ => Err(anyhow::anyhow!("Invalid opcode_sub")),
                }
            },
            0x05 => {
                match opcode_sub {
                    0x00 => Ok(Inst::Sw { rs1: rs1_i_s, rs2, imm: imm as i32 }),
                    0x01 => Ok(Inst::Sh { rs1: rs1_i_s, rs2, imm: imm as i32 }),
                    0x02 => Ok(Inst::Sb { rs1: rs1_i_s, rs2, imm: imm as i32 }),
                    _ => Err(anyhow::anyhow!("Invalid opcode_sub")),
                }
            },
            0x06 => {
                match opcode_sub {
                    0x00 => Ok(Inst::In  { rd, rs1: rs1_i_s, imm: imm as i32 }),
                    0x01 => Ok(Inst::Out { rs1: rs1_i_s, rs2, imm: imm as i32 }),
                    _ => Err(anyhow::anyhow!("Invalid opcode_sub")),
                }
            },
            _ => Err(anyhow::anyhow!("Invalid opcode")),
        }
    }
}

impl From<&Inst> for u64 {
    #[rustfmt::skip]
    fn from(inst: &Inst) -> u64 {
        let s: String = match inst {
            Inst::Add { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_001_00001", rs2, rs1, rd),
            Inst::Sub { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_010_00001", rs2, rs1, rd),

            Inst::Addi { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00010", imm, rs1, rd),
            Inst::Subi { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_00010", imm, rs1, rd),

            Inst::Beq { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_000_00011", imm, rs2, rs1, rd),
            Inst::Bne { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_001_00011", imm, rs2, rs1, rd),
            Inst::Blt { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_010_00011", imm, rs2, rs1, rd),
            Inst::Ble { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_011_00011", imm, rs2, rs1, rd),

            Inst::Lw  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_00100", imm, rs1, rd),
            Inst::Lh  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00100", imm, rs1, rd),
            Inst::Lb  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_00100", imm, rs1, rd),
            Inst::Lhu { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_011_00100", imm, rs1, rd),
            Inst::Lbu { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_100_00100", imm, rs1, rd),

            Inst::Sw { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_00101", imm, rs1, rs2),
            Inst::Sh { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00101", imm, rs1, rs2),
            Inst::Sb { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_00101", imm, rs1, rs2),

            Inst::In  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_00110", imm, rs1, rd),
            Inst::Out { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00110", imm, rs1, rs2),
        };
        let s: String = s.replace("_", "");

        u64::from_str_radix(&s, 2).unwrap()
    }
}

impl Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inst = u64::from(self);
        let inst_bytes = vec![
            (inst >> 0) & 0b11111111,
            (inst >> 8) & 0b11111111,
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
