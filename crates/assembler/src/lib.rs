mod imem;
mod dmem;

mod check;
mod resolve;
mod convert;

use check::check;
use resolve::resolve;
use convert::convert;

pub fn assemble(program: &str) -> anyhow::Result<String> { 
    let lines = program.lines();
    let sep_pos = lines.clone().position(|line| line == "===").unwrap();

    let lines = lines.collect::<Vec<_>>();
    let dmem_section = &lines[..sep_pos].join("\n");
    let imem_section = &lines[sep_pos+1..].join("\n");

    // dmem のパース

    // imem のパース
    let insts = imem::parse(&imem_section)?;
    
    // 合流
    check(&insts)?;
    let insts = resolve(insts)?;
    convert(insts)
}
