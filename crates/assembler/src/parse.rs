use crate::ir::unresolved::Inst;
use crate::ir::unresolved::InstKind;

pub fn parse(program: &str) -> anyhow::Result<Vec<Inst>> {
    // program
    // 1: addi r1 = r0, 1\n
    // 2: beq r0, (r0, r0) -> -42\n
    // 3: lw r7 = r0[4]\n
    // 4: ...

    let lines = program
        .split("\n")
        .map(|line| line.trim())
        .map(|line| line.split("//").collect::<Vec<_>>()[0])
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let mut insts = Vec::new();
    let mut label = None;
    for line in lines {
        // label
        if line.starts_with("@") {
            label = Some(line[1..].to_string());
        } else {
            let mut inst = parse_line(line)?;
            if label.is_some() {
                inst.label = label.take();
            }
            insts.push(inst)
        }
    }
    Ok(insts)
}

fn parse_line(line: &str) -> anyhow::Result<Inst> {
    let (lhs, rhs) = if line.contains("->") {
        // beq, ...
        let splitted_by_arrow = line.split("->").collect::<Vec<_>>();
        (splitted_by_arrow[0], splitted_by_arrow[1])
    } else {
        // add, lw, sw, in, out, ...
        let splitted_by_eq = line.split("=").collect::<Vec<_>>();
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
        return parse_inst(kind, vec![lhs[0], &lhs[1].replace("(", ""), &lhs[2].replace(")", ""), rhs]);
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
        return parse_inst(kind, vec![lhs[0], &lhs[1].replace("]", ""), rhs]);
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
        return parse_inst(kind, vec![lhs, rhs[0], &rhs[1].replace("]", "")]);
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

    parse_inst(kind, vec![lhs, rhs[0], rhs[1]])
}

fn parse_inst(kind: &str, args: Vec<&str>) -> anyhow::Result<Inst> {
    enum ArgEither {
        Num(i32),
        String(String),
    }

    impl ArgEither {
        fn u8(&self) -> u8 {
            match self {
                ArgEither::Num(num) => *num as u8,
                ArgEither::String(s) => panic!("Unexpected string: {}", s),
            }
        }
        
        fn i32(&self) -> i32 {
            match self {
                ArgEither::Num(num) => *num,
                ArgEither::String(s) => panic!("Unexpected string: {}", s),
            }
        }
        
        fn u32(&self) -> u32 {
            match self {
                ArgEither::Num(num) => *num as u32,
                ArgEither::String(s) => panic!("Unexpected string: {}", s),
            }
        }

        fn string(&self) -> String {
            match self {
                ArgEither::Num(num) => panic!("Unexpected number: {}", num),
                ArgEither::String(s) => s.clone(),
            }
        }
    }

    let args = args
        .into_iter()
        .map(|arg| arg.replace("r", ""))
        .map(|arg| {
            match arg.parse::<i32>() {
                Ok(num) => ArgEither::Num(num),
                Err(_) => ArgEither::String(arg),
            }
        })
        .collect::<Vec<_>>();

    let inst_kind = match kind {
        "add" => Ok(InstKind::Add { rd: args[0].u8(), rs1: args[1].u8(), rs2: args[2].u8() }),
        "sub" => Ok(InstKind::Sub { rd: args[0].u8(), rs1: args[1].u8(), rs2: args[2].u8() }),

        "addi" => Ok(InstKind::Addi { rd: args[0].u8(), rs1: args[1].u8(), imm: args[2].u32() }),
        "subi" => Ok(InstKind::Subi { rd: args[0].u8(), rs1: args[1].u8(), imm: args[2].u32() }),

        "beq" => Ok(InstKind::Beq { rd: args[0].u8(), rs1: args[1].u8(), rs2: args[2].u8(), label: args[3].string() }),
        "bne" => Ok(InstKind::Bne { rd: args[0].u8(), rs1: args[1].u8(), rs2: args[2].u8(), label: args[3].string() }),
        "blt" => Ok(InstKind::Blt { rd: args[0].u8(), rs1: args[1].u8(), rs2: args[2].u8(), label: args[3].string() }),
        "ble" => Ok(InstKind::Ble { rd: args[0].u8(), rs1: args[1].u8(), rs2: args[2].u8(), label: args[3].string() }),

        "lw" => Ok(InstKind::Lw { rd: args[0].u8(), rs1: args[1].u8(), imm: args[2].i32() }),
        "lh" => Ok(InstKind::Lh { rd: args[0].u8(), rs1: args[1].u8(), imm: args[2].i32() }),
        "lb" => Ok(InstKind::Lb { rd: args[0].u8(), rs1: args[1].u8(), imm: args[2].i32() }),
        "lhu" => Ok(InstKind::Lhu { rd: args[0].u8(), rs1: args[1].u8(), imm: args[2].i32() }),
        "lbu" => Ok(InstKind::Lbu { rd: args[0].u8(), rs1: args[1].u8(), imm: args[2].i32() }),

        "sw" => Ok(InstKind::Sw { rs1: args[0].u8(), imm: args[1].i32(), rs2: args[2].u8() }),
        "sh" => Ok(InstKind::Sh { rs1: args[0].u8(), imm: args[1].i32(), rs2: args[2].u8() }),
        "sb" => Ok(InstKind::Sb { rs1: args[0].u8(), imm: args[1].i32(), rs2: args[2].u8() }),

        "in" => Ok(InstKind::In { rd: args[0].u8(), rs1: args[1].u8(), imm: args[2].i32() }),
        "out" => Ok(InstKind::Out { rs1: args[0].u8(), imm: args[1].i32(), rs2: args[2].u8() }),

       _ => Err(anyhow::anyhow!("Invalid instruction: {}", kind)),
    }?;

    Ok(Inst {
        kind: inst_kind,
        label: None,
    })
}
