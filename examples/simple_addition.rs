extern crate jvm_assembler;

use jvm_assembler::*;

fn main() {
    let mut class = define_class(ACC_PUBLIC, "simple_addition", "java/lang/Object");

    {
        // create main method
        let mut method = class.define_method(ACC_PUBLIC | ACC_STATIC, "main", "([Ljava/lang/String;)V");

        // push PrintStream object onto the stack for later use
        method.get_static("java/lang/System", "out", "Ljava/io/PrintStream;");

        // execute 11 + 37 + 42
        method.bipush(11);
        method.bipush(37);
        method.iadd();
        method.bipush(42);
        method.iadd();

        // print the result
        method.invoke_virtual("java/io/PrintStream", "println", "(I)V");

        // add return statement
        method.do_return();

        // fini!
        method.done();
    }

    let classfile = class.done();
    write_classfile(classfile, "simple_addition.class");
}
