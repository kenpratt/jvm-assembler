extern crate jvm_assembler;

use jvm_assembler::*;

fn main() {
    let mut class = define_class(ACC_PUBLIC, "if_statement", "java/lang/Object");

    {
        // create main method
        let mut method = class.define_method(ACC_PUBLIC | ACC_STATIC, "main", "([Ljava/lang/String;)V");

        // if (args.length > 0) {
        //     System.out.println("Hello with args!");
        // } else {
        //     System.out.println("Hello without args!");
        // }
        method.aload0();
        method.array_length();
        method.ifle("false");

        // true case
        method.get_static("java/lang/System", "out", "Ljava/io/PrintStream;");
        method.load_constant("Hello with args!");
        method.invoke_virtual("java/io/PrintStream", "println", "(Ljava/lang/Object;)V", 1, false);
        method.goto("after");

        // false case
        method.label("false");
        method.get_static("java/lang/System", "out", "Ljava/io/PrintStream;");
        method.load_constant("Hello without args!");
        method.invoke_virtual("java/io/PrintStream", "println", "(Ljava/lang/Object;)V", 1, false);

        // after
        method.label("after");
        method.do_return();

        // fini!
        method.done();
    }

    let classfile = class.done();
    write_classfile(classfile, "if_statement.class");
}
