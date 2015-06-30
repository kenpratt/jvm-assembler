extern crate jvm_assembler;

use jvm_assembler::*;

fn main() {
    let mut class = define_class(ACC_PUBLIC, "static_methods", "java/lang/Object");

    {
        let mut method = class.define_method(ACC_PUBLIC | ACC_STATIC, "main", "([Ljava/lang/String;)V");
        method.invoke_static("static_methods", "hello_world", "()V", 0, false);
        method.load_constant("Rust");
        method.invoke_static("static_methods", "hello_someone", "(Ljava/lang/String;)V", 1, false);
        method.do_return();
        method.done();
    }

    {
        let mut method = class.define_method(ACC_STATIC, "hello_world", "()V");
        method.get_static("java/lang/System", "out", "Ljava/io/PrintStream;");
        method.load_constant("Hello, World!");
        method.invoke_virtual("java/io/PrintStream", "println", "(Ljava/lang/String;)V", 1, false);
        method.do_return();
        method.done();
    }

    {
        let mut method = class.define_method(ACC_STATIC, "hello_someone", "(Ljava/lang/String;)V");
        method.get_static("java/lang/System", "out", "Ljava/io/PrintStream;");
        method.load_constant("Hello, ");
        method.invoke_virtual("java/io/PrintStream", "print", "(Ljava/lang/String;)V", 1, false);

        method.get_static("java/lang/System", "out", "Ljava/io/PrintStream;");
        method.aload0();
        method.invoke_virtual("java/io/PrintStream", "print", "(Ljava/lang/String;)V", 1, false);

        method.get_static("java/lang/System", "out", "Ljava/io/PrintStream;");
        method.load_constant("!");
        method.invoke_virtual("java/io/PrintStream", "println", "(Ljava/lang/String;)V", 1, false);

        method.do_return();
        method.done();
    }

    let classfile = class.done();
    write_classfile(classfile, "static_methods.class");
}
