use crate::dmem::ir::{Command, Data};
use crate::imem::ir::resolved::Inst;

pub fn convert(
    datas: Vec<Data>,
    insts: Vec<Inst>,
    chunk_size: usize,
) -> anyhow::Result<(String, String)> {
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
                bytes.push((s >> 0) as u8);
                bytes.push((s >> 8) as u8);
            }
            Command::Byte4(s) => {
                bytes.push((s >> 0) as u8);
                bytes.push((s >> 8) as u8);
                bytes.push((s >> 16) as u8);
                bytes.push((s >> 24) as u8);
            }
            Command::Byte6(s) => {
                bytes.push((s >> 0) as u8);
                bytes.push((s >> 8) as u8);
                bytes.push((s >> 16) as u8);
                bytes.push((s >> 24) as u8);
                bytes.push((s >> 32) as u8);
                bytes.push((s >> 40) as u8);
            }
            Command::Char(s) => bytes.push(s as u8),
            Command::String(s) => {
                for n in s.as_bytes() {
                    bytes.push(*n);
                }
                bytes.push(0);
            }
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
            Inst::Addi { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_000_000_{:0>32b}", rd, rs1, imm),
            Inst::Subi { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_000_001_{:0>32b}", rd, rs1, imm),
            Inst::Andi { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_000_010_{:0>32b}", rd, rs1, imm),
            Inst::Ori  { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_000_011_{:0>32b}", rd, rs1, imm),
            Inst::Xori { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_000_100_{:0>32b}", rd, rs1, imm),
            Inst::Srli { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_000_101_{:0>32b}", rd, rs1, imm),
            Inst::Srai { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_000_110_{:0>32b}", rd, rs1, imm),
            Inst::Slli { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_000_111_{:0>32b}", rd, rs1, imm),

            Inst::Add { rd, rs1, rs2 } => format!("{:0>5b}_{:0>5b}_001_000_{:0>5b}_000_00000000_00000000_00000000", rd, rs1, rs2),
            Inst::Sub { rd, rs1, rs2 } => format!("{:0>5b}_{:0>5b}_001_001_{:0>5b}_000_00000000_00000000_00000000", rd, rs1, rs2),
            Inst::And { rd, rs1, rs2 } => format!("{:0>5b}_{:0>5b}_001_010_{:0>5b}_000_00000000_00000000_00000000", rd, rs1, rs2),
            Inst::Or  { rd, rs1, rs2 } => format!("{:0>5b}_{:0>5b}_001_011_{:0>5b}_000_00000000_00000000_00000000", rd, rs1, rs2),
            Inst::Xor { rd, rs1, rs2 } => format!("{:0>5b}_{:0>5b}_001_100_{:0>5b}_000_00000000_00000000_00000000", rd, rs1, rs2),
            Inst::Srl { rd, rs1, rs2 } => format!("{:0>5b}_{:0>5b}_001_101_{:0>5b}_000_00000000_00000000_00000000", rd, rs1, rs2),
            Inst::Sra { rd, rs1, rs2 } => format!("{:0>5b}_{:0>5b}_001_110_{:0>5b}_000_00000000_00000000_00000000", rd, rs1, rs2),
            Inst::Sll { rd, rs1, rs2 } => format!("{:0>5b}_{:0>5b}_001_111_{:0>5b}_000_00000000_00000000_00000000", rd, rs1, rs2),

            Inst::Lw  { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_010_000_{:0>32b}", rd, rs1, imm),
            Inst::Lh  { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_010_001_{:0>32b}", rd, rs1, imm),
            Inst::Lb  { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_010_010_{:0>32b}", rd, rs1, imm),
            Inst::Lhu { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_010_011_{:0>32b}", rd, rs1, imm),
            Inst::Lbu { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_010_100_{:0>32b}", rd, rs1, imm),
            Inst::Ilb { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_010_101_{:0>32b}", rd, rs1, imm),
            Inst::In  { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_010_110_{:0>32b}", rd, rs1, imm),

            Inst::Sw  { rs1, rs2, imm } => format!("{:0>5b}_{:0>5b}_011_000_{:0>32b}", rs2, rs1, imm),
            Inst::Sh  { rs1, rs2, imm } => format!("{:0>5b}_{:0>5b}_011_001_{:0>32b}", rs2, rs1, imm),
            Inst::Sb  { rs1, rs2, imm } => format!("{:0>5b}_{:0>5b}_011_010_{:0>32b}", rs2, rs1, imm),

            Inst::Isb { rs1, rs2, imm } => format!("{:0>5b}_{:0>5b}_011_101_{:0>32b}", rs2, rs1, imm),
            Inst::Out { rs1, rs2, imm } => format!("{:0>5b}_{:0>5b}_011_110_{:0>32b}", rs2, rs1, imm),

            Inst::Beq { rd, rs1, rs2, imm } => format!("{:0>5b}_{:0>5b}_100_000_{:0>5b}_{:0>27b}", rd, rs1, rs2, imm),
            Inst::Bne { rd, rs1, rs2, imm } => format!("{:0>5b}_{:0>5b}_100_001_{:0>5b}_{:0>27b}", rd, rs1, rs2, imm),
            Inst::Blt { rd, rs1, rs2, imm } => format!("{:0>5b}_{:0>5b}_100_010_{:0>5b}_{:0>27b}", rd, rs1, rs2, imm),
            Inst::Ble { rd, rs1, rs2, imm } => format!("{:0>5b}_{:0>5b}_100_011_{:0>5b}_{:0>27b}", rd, rs1, rs2, imm),
            Inst::Jal { rd, rs1, imm } => format!("{:0>5b}_{:0>5b}_100_100_{:0>32b}", rd, rs1, imm),
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
