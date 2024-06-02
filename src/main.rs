use assembler::{assemble, assembly, Inst};
use compiler::compile;
use std::env;
use std::fs;

#[rustfmt::skip]
fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} [path/to/source]", args[0]);
        return;
    }
    
    let source = fs::read_to_string(&args[1]).unwrap();
    
    let lines = source
        .split("\n")
        .map(|line| line.trim())
        .map(|line| line.split("//").collect::<Vec<_>>()[0])
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let programs = lines
        .into_iter()
        .map(|line| Inst::try_from(line))
        .collect::<anyhow::Result<Vec<Inst>>>()
        .unwrap();
    
    println!("{}", assemble(programs));
}
