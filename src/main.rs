mod classfile;

use std::io::Write;
use std::fs::File;

use classfile::{Attribute, Classfile, Constant, Method, Serialize};

fn main() {
    let constants = vec![
        Constant::String(2),
        Constant::Utf8("Hello World!".to_owned()),
        Constant::Utf8("main".to_owned()),
        Constant::Utf8("([Ljava/lang/String;)V".to_owned()),
        Constant::Utf8("java/lang/System".to_owned()),
        Constant::Class(5),
        Constant::Utf8("out".to_owned()),
        Constant::Utf8("Ljava/io/PrintStream;".to_owned()),
        Constant::NameAndType(7, 8),
        Constant::Fieldref(6, 9),
        Constant::Utf8("java/io/PrintStream".to_owned()),
        Constant::Class(11),
        Constant::Utf8("println".to_owned()),
        Constant::Utf8("(Ljava/lang/Object;)V".to_owned()),
        Constant::NameAndType(13, 14),
        Constant::Methodref(12, 15),
        Constant::Utf8("Code".to_owned()),
        Constant::Utf8("hello".to_owned()),
        Constant::Class(18),
        Constant::Utf8("java/lang/Object".to_owned()),
        Constant::Class(20),
        ];

    let methods = vec![
        Method::new(3, 4, vec![Attribute::Code(17, 10, 10, vec![0xB2, 0x00, 0x0A, 0x12, 0x01, 0xB6, 0x00, 0x10, 0xB1], vec![], vec![])]),
        ];

    let f = Classfile::new(constants, 19, 21, methods);

    println!("classfile: {:?}", f);
    let mut bytes = vec![];
    f.serialize(&mut bytes);
    println!("serialized:");
    for b in bytes.iter() {
        print!("{:X} ", b);
    }
    println!("");

    let mut f = File::create("hello2.class").unwrap();
    f.write_all(&bytes).unwrap();
}
