use crate::dmem::ir::{Data, Command};

pub fn parse(program: &str) -> anyhow::Result<Vec<Data>> {
    let lines = program
        .split("\n")
        .map(|line| line.trim())
        .map(|line| line.split("//").collect::<Vec<_>>()[0])
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let mut data = Vec::new();
    let mut label = None;
    for line in lines {
        // label
        if line.starts_with("$") {
            label = Some(line[1..].to_string());
        } else {
            let mut line_data = parse_line(line)?;
            if label.is_some() {
                line_data[0].label = label.take();
            }
            data.extend(line_data);
        }
    }

    Ok(data)
}

fn parse_line(line: &str) -> anyhow::Result<Vec<Data>> {
    let splitted_line = line.split_whitespace().collect::<Vec<_>>();
    let (command, args) = splitted_line.split_first().unwrap();
    let command = command.trim();
    let args = args.join(" ");

    let mut data = Vec::new();
    for arg in args.split(",") {
        let arg = arg.trim();
        let inst_command = match command {
            "byte1" => Command::Byte1(parse_u8(arg)?),
            "byte2" => Command::Byte2(parse_u16(arg)?),
            "byte4" => Command::Byte4(parse_u32(arg)?),
            "byte6" => Command::Byte6(parse_u48(arg)?),
            "char" => Command::Char(parse_char(arg)?),
            "string" => Command::String(parse_string(arg)?),
            _ => return Err(anyhow::anyhow!("Invalid command: {}", command)),
        };
        data.push(Data {
            label: None,
            command: inst_command,
        });
    }

    Ok(data)
}

fn parse_u8(num: &str) -> anyhow::Result<u8> {
    let mut num = parse_num_with_radix(num)?;
    if !(i8::MIN as i64 <= num && num <= u8::MAX as i64) {
        return Err(anyhow::anyhow!("Invalid value: {}", num));
    } 
    if num < 0 {
        num = (num + (u8::MAX) as i64 + 1) | 0x80;
    }
    Ok(num as u8)
}

fn parse_u16(num: &str) -> anyhow::Result<u16> {
    let mut num = parse_num_with_radix(num)?;
    if !(i16::MIN as i64 <= num && num <= u16::MAX as i64) {
        return Err(anyhow::anyhow!("Invalid value: {}", num));
    } 
    if num < 0 {
        num = (num + (u16::MAX) as i64 + 1) | 0x8000;
    }
    Ok(num as u16)
}

fn parse_u32(num: &str) -> anyhow::Result<u32> {
    let mut num = parse_num_with_radix(num)?;
    if !(i32::MIN as i64 <= num && num <= u32::MAX as i64) {
        return Err(anyhow::anyhow!("Invalid value: {}", num));
    } 
    if num < 0 {
        num = (num + (u32::MAX) as i64 + 1) | 0x80000000;
    }
    Ok(num as u32)
}

fn parse_u48(num: &str) -> anyhow::Result<u64> {
    let mut num = parse_num_with_radix(num)?;
    let i48_min = -(1 << 47);
    let u48_max = (1 << 48) - 1;
    if !(i48_min <= num && num <= u48_max) {
        return Err(anyhow::anyhow!("Invalid value: {}", num));
    } 
    if num < 0 {
        num = (num + u48_max + 1) | 0x800000000000;
    }
    Ok(num as u64)
}

fn parse_num_with_radix(num_s: &str) -> anyhow::Result<i64> {
    // 0x 0X: 16, 0b 0B: 2, nothing: 10
    let num = if (num_s.starts_with("0x") || num_s.starts_with("0X")) && num_s.len() > 2 {
        i64::from_str_radix(&num_s[2..], 16)
    } else if (num_s.starts_with("0b") || num_s.starts_with("0B")) && num_s.len() > 2 {
        i64::from_str_radix(&num_s[2..], 2)
    } else {
        i64::from_str_radix(&num_s, 10)
    };

    if let Ok(num) = num {
        Ok(num)
    } else {
        Err(anyhow::anyhow!("Invalid value: {}", num_s))
    }
}

fn parse_char(ch: &str) -> anyhow::Result<char> {
    let chars = ch.chars().collect::<Vec<_>>();
    // シングルクォーテーションで囲まれていることを検査
    if chars[0] != '\'' || chars[2] != '\'' {
        return Err(anyhow::anyhow!("Unexpected identifier(expect: \"'\"): {}", ch));
    }
    Ok(chars[1])
}

fn parse_string(string: &str) -> anyhow::Result<String> {
    let chars = string.chars().collect::<Vec<_>>();
    // ダブルクォーテーションで囲まれていることを検査
    if chars[0] != '\"' || chars[chars.len()-1] != '\"' {
        return Err(anyhow::anyhow!("Unexpected identifier(expect: '\"'): {}", string));
    }
    Ok(chars[1..chars.len()-1].iter().collect())
}
