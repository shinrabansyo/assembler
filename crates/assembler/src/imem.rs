mod parse;
mod check;
mod resolve;
mod convert;
mod ir;

use parse::parse;
use check::check;
use resolve::resolve;
use convert::convert;

pub fn assemble(program: &str) -> anyhow::Result<String> {
    let insts = parse(program)?;
    check(&insts)?;
    let insts = resolve(insts)?;
    convert(insts)
}
