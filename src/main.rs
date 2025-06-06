use std::env;
use std::io::Write;
use std::fs::File;
use std::fs;

use sb_assembler::assemble;

#[rustfmt::skip]
fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 4 {
        println!("Usage: {} [path/to/source] <data.hex> <inst.hex>", args[0]);
        return;
    }

    let source = fs::read_to_string(&args[1]).unwrap();
    let (datas, insts) = assemble(&source).unwrap();

    let file_data_path = &args[2];
    File::create(file_data_path).unwrap().write(datas.as_bytes()).unwrap();

    let file_inst_path = &args[3];
    File::create(file_inst_path).unwrap().write(insts.as_bytes()).unwrap();
}
