extern crate jvm_assembler;

use jvm_assembler::*;

fn main() {
    let mut class = define_class(ACC_PUBLIC, "static_methods", "java/lang/Object");

    {
        let mut method = class.define_method(ACC_PUBLIC | ACC_STATIC, "main", &[Java::Array(Box::new(Java::Class("java/lang/String")))], &Java::Void);
        method.invoke_static("static_methods", "hello_world", &[], &Java::Void);
        method.load_constant("Rust");
        method.invoke_static("static_methods", "hello_someone", &[Java::Class("java/lang/String")], &Java::Void);
        method.do_return();
        method.done();
    }

    {
        let mut method = class.define_method(ACC_STATIC, "hello_world", &[], &Java::Void);
        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("Hello, World!");
        method.invoke_virtual("java/io/PrintStream", "println", &[Java::Class("java/lang/String")], &Java::Void);
        method.do_return();
        method.done();
    }

    {
        let mut method = class.define_method(ACC_STATIC, "hello_someone", &[Java::Class("java/lang/String")], &Java::Void);
        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("Hello, ");
        method.invoke_virtual("java/io/PrintStream", "print", &[Java::Class("java/lang/String")], &Java::Void);

        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.aload0();
        method.invoke_virtual("java/io/PrintStream", "print", &[Java::Class("java/lang/String")], &Java::Void);

        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("!");
        method.invoke_virtual("java/io/PrintStream", "println", &[Java::Class("java/lang/String")], &Java::Void);

        method.do_return();
        method.done();
    }

    let classfile = class.done();
    write_classfile(classfile, "static_methods.class");
}
