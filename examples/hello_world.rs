extern crate jvm_assembler;

use jvm_assembler::*;

fn main() {
    let mut class = define_class(ACC_PUBLIC, "hello_world", "java/lang/Object");

    {
        // create main method
        let mut method = class.define_method(ACC_PUBLIC | ACC_STATIC, "main", "([Ljava/lang/String;)V");

        // push PrintStream object and string to print onto the stack, and then call println function
        method.get_static("java/lang/System", "out", "Ljava/io/PrintStream;");
        method.load_constant("Hello, World!");
        method.invoke_virtual("java/io/PrintStream", "println", "(Ljava/lang/Object;)V", 1, false);

        // add return statement
        method.do_return();

        // fini!
        method.done();
    }

    let classfile = class.done();
    write_classfile(classfile, "hello_world.class");
}
