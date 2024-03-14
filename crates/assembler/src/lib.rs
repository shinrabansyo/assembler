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
        Inst::from((stringify!($kind), vec![stringify!($rd), stringify!($rs1), stringify!($rs2)]))
    };

    // beq, ...
    ($kind:ident $rd:ident, ($rs1:ident, $rs2:ident) -> $imm:expr) => {
        Inst::from((stringify!($kind), vec![stringify!($rd), stringify!($rs1), stringify!($rs2), stringify!($imm)]))
    };

    // lw, ..., in
    ($kind:ident $rd:ident = $rs1:ident[$imm:expr]) => {
        Inst::from((stringify!($kind), vec![stringify!($rd), stringify!($rs1), stringify!($imm)]))
    };

    // sw, ..., out
    ($kind:ident $rs1:ident[$imm:expr] = $rs2:ident) => {
        Inst::from((stringify!($kind), vec![stringify!($rs1), stringify!($imm), stringify!($rs2)]))
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

impl From<(&str, Vec<&str>)> for Inst {
    #[rustfmt::skip]
    fn from((kind, args): (&str, Vec<&str>)) -> Self {
        let args = args
            .into_iter()
            .map(|arg| arg.replace("r", ""))
            .map(|arg| arg.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        match kind {
            "add" => Inst::Add { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8 },
            "sub" => Inst::Sub { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8 },

            "addi" => Inst::Addi { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] as u32 },
            "subi" => Inst::Subi { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] as u32 },

            "beq" => Inst::Beq { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8, imm: args[3] },
            "bne" => Inst::Bne { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8, imm: args[3] },
            "blt" => Inst::Blt { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8, imm: args[3] },
            "ble" => Inst::Ble { rd: args[0] as u8, rs1: args[1] as u8, rs2: args[2] as u8, imm: args[3] },

            "lw" => Inst::Lw { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] },
            "lh" => Inst::Lh { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] },
            "lb" => Inst::Lb { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] },
            "lhu" => Inst::Lhu { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] },
            "lbu" => Inst::Lbu { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] },

            "sw" => Inst::Sw { rs1: args[0] as u8, imm: args[1], rs2: args[2] as u8 },
            "sh" => Inst::Sh { rs1: args[0] as u8, imm: args[1], rs2: args[2] as u8 },
            "sb" => Inst::Sb { rs1: args[0] as u8, imm: args[1], rs2: args[2] as u8 },

            "in" => Inst::In { rd: args[0] as u8, rs1: args[1] as u8, imm: args[2] },
            "out" => Inst::Out { rs1: args[0] as u8, imm: args[1], rs2: args[2] as u8 },

           _ => unimplemented!(),
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
