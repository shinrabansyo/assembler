mod dmem;
mod imem;

mod check;
mod convert;
mod resolve;

use check::check;
use convert::convert;
use resolve::resolve;

pub fn assemble(program: &str, chunk_size: usize) -> anyhow::Result<(String, String)> {
    // 分割
    let lines = program.lines().collect::<Vec<_>>();
    let sep_pos = lines.iter().position(|&line| line == "===").unwrap();

    // 構文解析
    let datas = dmem::parse(&lines[..sep_pos])?;
    let insts = imem::parse(&lines[(sep_pos + 1)..])?;

    // 意味解析
    check(&datas, &insts)?;

    // コード生成
    let insts = resolve(&datas, insts)?;
    convert(datas, insts, chunk_size)
}
