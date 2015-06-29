mod classfile;
mod classfile_builder;

use std::env;
use std::fs::File;
use std::io::Write;

use classfile::*;

pub fn write_classfile(classfile: Classfile, filename: &str) {
    let mut bytes = vec![];
    classfile.serialize(&mut bytes);

    let mut f = File::create(filename).unwrap();
    f.write_all(&bytes).unwrap();
}

pub fn read_classfile(filename: &str) -> Classfile {
    let f = File::open(filename).unwrap();
    Classfile::deserialize(Box::new(f))
}

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
