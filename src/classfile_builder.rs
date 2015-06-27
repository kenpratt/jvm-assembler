use classfile::*;

pub const ACC_PUBLIC: u16 = 0x1;
pub const ACC_STATIC: u16 = 0x8;

pub struct ClassfileBuilder {
    access_flags: u16,
    this_class: &'static str,
    super_class: &'static str,
    constants: Vec<Constant>,
    methods: Vec<Method>,
}

pub struct MethodBuilder<'a> {
    classfile: &'a mut ClassfileBuilder,
    access_flags: u16,
    name: &'static str,
    descriptor: &'static str,
    instructions: Vec<Instruction>,
}

impl ClassfileBuilder {
    pub fn new(access_flags: u16, this_class: &'static str, super_class: &'static str) -> ClassfileBuilder {
        ClassfileBuilder {
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            constants: vec![],
            methods: vec![],
        }
    }

    pub fn define_method(&mut self, access_flags: u16, name: &'static str, descriptor: &'static str) -> MethodBuilder {
        MethodBuilder::new(self, access_flags, name, descriptor)
    }

    fn push_constant(&mut self, constant: Constant) -> u16 {
        self.constants.push(constant);
        self.constants.len() as u16
    }

    fn define_utf8(&mut self, string: &'static str) -> u16 {
        self.push_constant(Constant::Utf8(string.to_owned()))
    }

    fn define_class(&mut self, class: &'static str) -> u16 {
        let name_index = self.define_utf8(class);
        self.push_constant(Constant::Class(name_index))
    }

    fn define_name_and_type(&mut self, name: &'static str, descriptor: &'static str) -> u16 {
        let name_index = self.define_utf8(name);
        let descriptor_index = self.define_utf8(descriptor);
        self.push_constant(Constant::NameAndType(name_index, descriptor_index))
    }

    fn define_fieldref(&mut self, class: &'static str, name: &'static str, descriptor: &'static str) -> u16 {
        let class_index = self.define_class(class);
        let name_and_type_index = self.define_name_and_type(name, descriptor);
        self.push_constant(Constant::Fieldref(class_index, name_and_type_index))
    }

    fn define_methodref(&mut self, class: &'static str, name: &'static str, descriptor: &'static str) -> u16 {
        let class_index = self.define_class(class);
        let name_and_type_index = self.define_name_and_type(name, descriptor);
        self.push_constant(Constant::Methodref(class_index, name_and_type_index))
    }

    pub fn done(mut self) -> Classfile {
        let this_class_index = self.define_class(self.this_class);
        let super_class_index = self.define_class(self.super_class);
        Classfile::new(self.constants, self.access_flags, this_class_index, super_class_index, self.methods)
    }
}

impl<'a> MethodBuilder<'a> {
    fn new(classfile: &'a mut ClassfileBuilder, access_flags: u16, name: &'static str, descriptor: &'static str) -> MethodBuilder<'a> {
        MethodBuilder {
            classfile: classfile,
            access_flags: access_flags,
            name: name,
            descriptor: descriptor,
            instructions: vec![],
        }
    }

    pub fn get_static(&mut self, class: &'static str, name: &'static str, descriptor: &'static str) {
        let fieldref_index = self.classfile.define_fieldref(class, name, descriptor);
        self.instructions.push(Instruction::GetStatic(fieldref_index));
        self.increase_stack_depth();
    }

    pub fn invoke_virtual(&mut self, class: &'static str, name: &'static str, descriptor: &'static str) {
        let methodref_index = self.classfile.define_methodref(class, name, descriptor);
        self.instructions.push(Instruction::InvokeVirtual(methodref_index));
        self.decrease_stack_depth();
    }

    pub fn bipush(&mut self, value: u8) {
        self.instructions.push(Instruction::Bipush(value));
        self.increase_stack_depth();
    }

    pub fn iadd(&mut self) {
        self.instructions.push(Instruction::Iadd);
        self.decrease_stack_depth();
    }

    pub fn do_return(&mut self) {
        self.instructions.push(Instruction::Return);
    }

    fn increase_stack_depth(&mut self) {
        // TODO implement maximum stack size tracking
    }

    fn decrease_stack_depth(&mut self) {
        // TODO implement maximum stack size tracking
    }

    pub fn done(mut self) {
        let classfile = self.classfile;
        let name_index = classfile.define_utf8(self.name);
        let descriptor_index = classfile.define_utf8(self.descriptor);
        let code_index = classfile.define_utf8("Code");
        let method = Method::new(self.access_flags, name_index, descriptor_index, vec![Attribute::Code(code_index, 3, 2, self.instructions, vec![], vec![])]);
        classfile.methods.push(method);
    }
}
