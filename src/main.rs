use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

use sb_assembler::assemble;

#[rustfmt::skip]
fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 4 {
        println!("Usage: {} [path/to/source] <data.hex> <inst.hex> [<chunk_size>]", args[0]);
        return;
    }

    let source = fs::read_to_string(&args[1]).unwrap();
    let chunk_size = if args.len() >= 5 {
        args[4].parse().unwrap_or(1)
    } else {
        1
    };
    let (datas, insts) = assemble(&source, chunk_size).unwrap();

    let file_data_path = &args[2];
    File::create(file_data_path).unwrap().write(datas.as_bytes()).unwrap();

    let file_inst_path = &args[3];
    File::create(file_inst_path).unwrap().write(insts.as_bytes()).unwrap();
}
