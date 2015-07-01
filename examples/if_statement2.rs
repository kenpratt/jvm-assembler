extern crate jvm_assembler;

use jvm_assembler::*;

fn main() {
    let mut class = define_class(ACC_PUBLIC, "if_statement2", "java/lang/Object");

    {
        // create main method
        let mut method = class.define_method(ACC_PUBLIC | ACC_STATIC, "main", &[Java::Array(Box::new(Java::Class("java/lang/String")))], &Java::Void);

        // if (args.length > 0) {
        //     System.out.println("Hello with args!");
        //     if (args[0].length() >= 5) {
        //         System.out.println("First arg had at least 5 characters");
        //     } else {
        //         System.out.println("First arg has less than 5 characters");
        //     }
        // } else {
        //     System.out.println("Hello without args!");
        // }
        method.aload0();
        method.array_length();
        method.ifle("outer-else");

        // outer if: true case
        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("Hello with args!");
        method.invoke_virtual("java/io/PrintStream", "println", &[Java::Class("java/lang/String")], &Java::Void);

        // inner if: load the first arg and calculate string length
        method.aload0();
        method.iconst0();
        method.aaload();
        method.invoke_virtual("java/lang/String", "length", &[], &Java::Int);

        // inner if: do comparison against 5
        method.iconst5();
        method.if_icmp_lt("inner-else");

        // inner if: true case
        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("First arg has at least 5 characters");
        method.invoke_virtual("java/io/PrintStream", "println", &[Java::Class("java/lang/String")], &Java::Void);
        method.goto("outer-after");

        // inner if: false case
        method.label("inner-else");
        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("First arg has less than 5 characters");
        method.invoke_virtual("java/io/PrintStream", "println", &[Java::Class("java/lang/String")], &Java::Void);

        // outer if: done true case
        method.goto("outer-after");

        // outer if: false case
        method.label("outer-else");
        method.get_static("java/lang/System", "out", &Java::Class("java/io/PrintStream"));
        method.load_constant("Hello without args!");
        method.invoke_virtual("java/io/PrintStream", "println", &[Java::Class("java/lang/String")], &Java::Void);

        // after outer if
        method.label("outer-after");
        method.do_return();

        // fini!
        method.done();
    }

    let classfile = class.done();
    write_classfile(classfile, "if_statement2.class");
}
