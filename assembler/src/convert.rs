use crate::imem::ir::resolved::Inst;
use crate::dmem::ir::{Data, Command};

pub fn convert(datas: Vec<Data>, insts: Vec<Inst>, chunk_size: usize) -> anyhow::Result<(String, String)> {
    let datas = command_convert(datas, chunk_size)?;
    let inst = inst_convert(insts, chunk_size)?;
    Ok((datas, inst))
}
pub fn command_convert(datas: Vec<Data>, chunk_size: usize) -> anyhow::Result<String> {
    let mut bytes: Vec<u8> = Vec::new();

    for data in datas {
        match data.command {
            Command::Byte1(s) => bytes.push(s),
            Command::Byte2(s) => {
                bytes.push((s >>  0) as u8);
                bytes.push((s >>  8) as u8);
            }
            Command::Byte4(s) => {
                bytes.push((s >>  0) as u8);
                bytes.push((s >>  8) as u8);
                bytes.push((s >> 16) as u8);
                bytes.push((s >> 24) as u8);
            }
            Command::Byte6(s) => {
                bytes.push((s >>  0) as u8);
                bytes.push((s >>  8) as u8);
                bytes.push((s >> 16) as u8);
                bytes.push((s >> 24) as u8);
                bytes.push((s >> 32) as u8);
                bytes.push((s >> 40) as u8);
            },
            Command::Char(s) => bytes.push(s as u8),
            Command::String(s) => {
                for n in s.as_bytes(){
                    bytes.push(*n);
                }
                bytes.push(0);
            },
        }
    }

    // chunk_size に満たない場合は 0 で埋める
    if bytes.len() % chunk_size != 0 {
        for _ in 0..(chunk_size - (bytes.len() % chunk_size)) {
            bytes.push(0);
        }
    }

    // chunk_size ごとに区切って、リトルエンディアンで出力
    let result = bytes
        .chunks(chunk_size)
        .map(|chunk| {
            chunk
                .iter()
                .rev()
                .map(|e| format!("{:0>2X}", e))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(result)
}

pub fn inst_convert(insts: Vec<Inst>, chunk_size: usize) -> anyhow::Result<String> {
    let mut bytes = Vec::new();

    for inst in insts {
        #[rustfmt::skip]
        let s: String = match inst {
            Inst::Add { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_001_00001", rs2, rs1, rd),
            Inst::Sub { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_010_00001", rs2, rs1, rd),
    
            Inst::Addi { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00010", imm, rs1, rd),
            Inst::Subi { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_00010", imm, rs1, rd),

            
            Inst::Beq { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_000_00011", imm, rs2, rs1, rd),
            Inst::Bne { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_001_00011", imm, rs2, rs1, rd),
            Inst::Blt { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_010_00011", imm, rs2, rs1, rd),
            Inst::Ble { rd, rs1, rs2, imm } => format!("{:0>25b}_{:0>5b}_{:0>5b}_{:0>5b}_011_00011", imm, rs2, rs1, rd),
            Inst::Jal { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_100_00011", imm, rs1, rd),
    
            Inst::Lw  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_00100", imm, rs1, rd),
            Inst::Lh  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00100", imm, rs1, rd),
            Inst::Lb  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_00100", imm, rs1, rd),
            Inst::Lhu { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_011_00100", imm, rs1, rd),
            Inst::Lbu { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_100_00100", imm, rs1, rd),
    
            Inst::Sw  { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_00101", imm, rs1, rs2),
            Inst::Sh  { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00101", imm, rs1, rs2),
            Inst::Sb  { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_00101", imm, rs1, rs2),
            Inst::Isb { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_011_00101", imm, rs1, rs2),
    
            Inst::In  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_00110", imm, rs1, rd),
            Inst::Out { rs1, rs2, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_00110", imm, rs1, rs2),

            Inst::And { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_000_00111", rs2, rs1, rd),
            Inst::Or  { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_001_00111", rs2, rs1, rd),
            Inst::Xor { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_010_00111", rs2, rs1, rd),
            Inst::Srl { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_011_00111", rs2, rs1, rd),
            Inst::Sra { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_100_00111", rs2, rs1, rd),
            Inst::Sll { rd, rs1, rs2 } => format!("00000000_00000000_00000000_0_{:0>5b}_{:0>5b}_{:0>5b}_101_00111", rs2, rs1, rd),

            Inst::Andi { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_000_01000", imm, rs1, rd),
            Inst::Ori  { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_001_01000", imm, rs1, rd),
            Inst::Xori { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_010_01000", imm, rs1, rd),
            Inst::Srli { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_011_01000", imm, rs1, rd),
            Inst::Srai { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_100_01000", imm, rs1, rd),
            Inst::Slli { rd, rs1, imm } => format!("{:0>32b}_{:0>3b}_{:0>5b}_101_01000", imm, rs1, rd),
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
        bytes.extend_from_slice(&inst_bytes);


    }
    // chunk_size に満たない場合は 0 で埋める
    if bytes.len() % chunk_size != 0 {
        for _ in 0..(chunk_size - (bytes.len() % chunk_size)) {
            bytes.push(0);
        }
    }

    // chunk_size ごとに区切って、リトルエンディアンで出力
    let result = bytes
        .chunks(chunk_size)
        .map(|chunk| {
            chunk
                .iter()
                .rev()
                .map(|e| format!("{:0>2X}", e))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(result)
}
