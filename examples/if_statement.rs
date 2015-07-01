extern crate jvm_assembler;

use jvm_assembler::*;

fn main() {
    let mut class = define_class(ACC_PUBLIC, "if_statement", "java/lang/Object");

    {
        // create main method
        let mut method = class.define_method(ACC_PUBLIC | ACC_STATIC, "main", &[Java::Array(Box::new(Java::Class("java/lang/String")))], &Java::Void);

        // if (args.length > 0) {
        //     System.out.println("Hello with args!");
        // } else {
        //     System.out.println("Hello without args!");
        // }
        method.aload0();
        method.array_length();
        method.ifle("false");

        // true case
        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("Hello with args!");
        method.invoke_virtual("java/io/PrintStream", "println", &[Java::Class("java/lang/String")], &Java::Void);
        method.goto("after");

        // false case
        method.label("false");
        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("Hello without args!");
        method.invoke_virtual("java/io/PrintStream", "println", &[Java::Class("java/lang/String")], &Java::Void);

        // after
        method.label("after");
        method.do_return();

        // fini!
        method.done();
    }

    let classfile = class.done();
    write_classfile(classfile, "if_statement.class");
}
