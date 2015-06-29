extern crate jvm_assembler;

use std::env;

use jvm_assembler::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("You must provide 3 arguments to {}", &args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_ref() {
        "read" => print!("{}", read_classfile(filename)),
        _ => panic!("Unknown command: {}", command)
    }
}
