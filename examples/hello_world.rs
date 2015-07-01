extern crate jvm_assembler;

use jvm_assembler::*;

fn main() {
    let mut class = define_class(ACC_PUBLIC, "hello_world", "java/lang/Object");

    {
        // create main method
        let mut method = class.define_method(ACC_PUBLIC | ACC_STATIC, "main", &[Java::Array(Box::new(Java::Class("java/lang/String")))], &Java::Void);

        // push PrintStream object and string to print onto the stack, and then call println function
        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("Hello, World!");
        method.invoke_virtual("java/io/PrintStream", "println", &[Java::Class("java/lang/String")], &Java::Void);

        // add return statement
        method.do_return();

        // fini!
        method.done();
    }

    let classfile = class.done();
    write_classfile(classfile, "hello_world.class");
}
