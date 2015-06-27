mod classfile;
mod classfile_builder;

use std::io::Write;
use std::fs::File;

use classfile::*;
use classfile_builder::*;

fn print_hello_world() -> Classfile {
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

    let instructions = vec![
        Instruction::GetStatic(10),
        Instruction::LoadConstant(1),
        Instruction::InvokeVirtual(16),
        Instruction::Return,
        ];

    let methods = vec![
        Method::new(0x9, 3, 4, vec![Attribute::Code(17, 2, 1, instructions, vec![], vec![])]),
        ];

    Classfile::new(constants, 0x1, 19, 21, methods)
}

fn print_integer() -> Classfile {
    let constants = vec![
        Constant::Utf8("main".to_owned()),
        Constant::Utf8("([Ljava/lang/String;)V".to_owned()),
        Constant::Utf8("java/lang/System".to_owned()),
        Constant::Class(3),
        Constant::Utf8("out".to_owned()),
        Constant::Utf8("Ljava/io/PrintStream;".to_owned()),
        Constant::NameAndType(5, 6),
        Constant::Fieldref(4, 7),
        Constant::Utf8("java/io/PrintStream".to_owned()),
        Constant::Class(9),
        Constant::Utf8("println".to_owned()),
        Constant::Utf8("(I)V".to_owned()),
        Constant::NameAndType(11, 12),
        Constant::Methodref(10, 13),
        Constant::Utf8("Code".to_owned()),
        Constant::Utf8("hello".to_owned()),
        Constant::Class(16),
        Constant::Utf8("java/lang/Object".to_owned()),
        Constant::Class(18),
        ];

    let instructions = vec![
        Instruction::GetStatic(8),
        Instruction::Bipush(42),
        Instruction::InvokeVirtual(14),
        Instruction::Return,
        ];

    let methods = vec![
        Method::new(0x9, 1, 2, vec![Attribute::Code(15, 2, 1, instructions, vec![], vec![])]),
        ];

    Classfile::new(constants, 0x1, 17, 19, methods)
}

fn print_addition_result() -> Classfile {
    let constants = vec![
        Constant::Utf8("main".to_owned()),
        Constant::Utf8("([Ljava/lang/String;)V".to_owned()),
        Constant::Utf8("java/lang/System".to_owned()),
        Constant::Class(3),
        Constant::Utf8("out".to_owned()),
        Constant::Utf8("Ljava/io/PrintStream;".to_owned()),
        Constant::NameAndType(5, 6),
        Constant::Fieldref(4, 7),
        Constant::Utf8("java/io/PrintStream".to_owned()),
        Constant::Class(9),
        Constant::Utf8("println".to_owned()),
        Constant::Utf8("(I)V".to_owned()),
        Constant::NameAndType(11, 12),
        Constant::Methodref(10, 13),
        Constant::Utf8("Code".to_owned()),
        Constant::Utf8("hello".to_owned()),
        Constant::Class(16),
        Constant::Utf8("java/lang/Object".to_owned()),
        Constant::Class(18),
        ];

    let instructions = vec![
        Instruction::GetStatic(8),
        Instruction::Bipush(11),
        Instruction::Bipush(37),
        Instruction::Iadd,
        Instruction::Bipush(42),
        Instruction::Iadd,
        Instruction::InvokeVirtual(14),
        Instruction::Return,
        ];

    let methods = vec![
        Method::new(0x9, 1, 2, vec![Attribute::Code(15, 3, 1, instructions, vec![], vec![])]),
        ];

    Classfile::new(constants, 0x1, 17, 19, methods)
}

fn print_addition_result2() -> Classfile {
    let mut classfile = ClassfileBuilder::new(ACC_PUBLIC, "hello", "java/lang/Object");

    {
        let mut method = classfile.define_method(ACC_PUBLIC | ACC_STATIC, "main", "([Ljava/lang/String;)V");
        method.get_static("java/lang/System", "out", "Ljava/io/PrintStream;");
        method.bipush(11);
        method.bipush(37);
        method.iadd();
        method.bipush(42);
        method.iadd();
        method.invoke_virtual("java/io/PrintStream", "println", "(I)V");
        method.do_return();
        method.done();
    }

    classfile.done()
}

fn main() {
    // let f = print_hello_world();
    // let f = print_integer();
    // let f = print_addition_result();
    let f = print_addition_result2();

    println!("classfile: {:?}", f);
    let mut bytes = vec![];
    f.serialize(&mut bytes);
    println!("serialized:");
    for b in bytes.iter() {
        print!("{:X} ", b);
    }
    println!("");

    let mut f = File::create("hello.class").unwrap();
    f.write_all(&bytes).unwrap();
}
