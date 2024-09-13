mod imem;
mod dmem;

mod check;
mod resolve;
mod convert;

use check::check;
use resolve::resolve;
use convert::convert;

pub fn assemble(program: &str) -> anyhow::Result<(String, String)> { 
    let lines = program.lines();
    let sep_pos = lines.clone().position(|line| line == "===").unwrap();

    let lines = lines.collect::<Vec<_>>();
    let dmem_section = &lines[..sep_pos].join("\n");
    let imem_section = &lines[sep_pos+1..].join("\n");

    // dmem のパース
    let datas = dmem::parse(&dmem_section)?;

    // imem のパース
    let insts = imem::parse(&imem_section)?;
    
    // 合流
    check(&datas, &insts)?;
    let insts = resolve(&datas, insts)?;
    convert(datas, insts)
}
