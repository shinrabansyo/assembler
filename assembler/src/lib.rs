mod imem;
mod dmem;

mod check;
mod resolve;
mod convert;

use check::check;
use resolve::resolve;
use convert::convert;

pub fn assemble(program: &str) -> anyhow::Result<(String, String)> {
    // 分割
    let lines = program.lines().collect::<Vec<_>>();
    let sep_pos = lines
        .iter()
        .position(|&line| line == "===")
        .unwrap();

    // 構文解析
    let datas = dmem::parse(&lines[..sep_pos])?;
    let insts = imem::parse(&lines[sep_pos..])?;

    // 意味解析
    check(&datas, &insts)?;

    // コード生成
    let insts = resolve(&datas, insts)?;
    convert(datas, insts)
}
