use std::fmt::Write;

use crate::imem::ir::resolved::Inst;
use crate::dmem::ir::{Data, Command};

pub fn convert(datas: Vec<Data>, insts: Vec<Inst>) -> anyhow::Result<(String, String)> {
    let datas = command_convert(datas)?;
    let inst = inst_convert(insts)?;
    Ok((datas, inst))
}
pub fn command_convert(datas: Vec<Data>) -> anyhow::Result<String> {
    let mut result = String::new();

    for data in datas {
        match data.command {
            Command::Byte1(s) => writeln!(result, "{:0>2X}", s)?,
            Command::Byte2(s) => {
                writeln!(result, "{:0>2X}", (s >>  0) & 0xff)?;
                writeln!(result, "{:0>2X}", (s >>  8) & 0xff)?;
            }
            Command::Byte4(s) => {
                writeln!(result, "{:0>2X}", (s >>  0) & 0xff)?;
                writeln!(result, "{:0>2X}", (s >>  8) & 0xff)?;
                writeln!(result, "{:0>2X}", (s >> 16) & 0xff)?;
                writeln!(result, "{:0>2X}", (s >> 24) & 0xff)?;
            }
            Command::Byte6(s) => {
                writeln!(result, "{:0>2X}", (s >>  0) & 0xff)?;
                writeln!(result, "{:0>2X}", (s >>  8) & 0xff)?;
                writeln!(result, "{:0>2X}", (s >> 16) & 0xff)?;
                writeln!(result, "{:0>2X}", (s >> 24) & 0xff)?;
                writeln!(result, "{:0>2X}", (s >> 32) & 0xff)?;
                writeln!(result, "{:0>2X}", (s >> 40) & 0xff)?;
            },
            Command::Char(s) => writeln!(result, "{:0>2X}", s as u8)?,
            Command::String(s) => {
                for n in s.as_bytes(){
                    writeln!(result, "{:0>2X}", n)?;
                }
                writeln!(result, "{:0>2X}", 0)?;
            },
        }
    }

    Ok(result)
}

pub fn inst_convert(insts: Vec<Inst>) -> anyhow::Result<String> {
    let mut result = String::new();

    for inst in insts {
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
