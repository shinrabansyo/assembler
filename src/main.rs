use std::env;
use std::fs;

use assembler::assemble;

#[rustfmt::skip]
fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} [path/to/source]", args[0]);
        return;
    }

    let source = fs::read_to_string(&args[1]).unwrap();

    println!("{}", assemble(&source).unwrap());
}
