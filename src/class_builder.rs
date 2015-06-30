use std::collections::HashMap;

use classfile::*;

pub const ACC_PUBLIC: u16 = 0x1;
pub const ACC_STATIC: u16 = 0x8;

pub struct ClassBuilder {
    access_flags: u16,
    this_class_index: u16,
    super_class_index: u16,
    constants: Vec<Constant>,
    methods: Vec<Method>,
}

impl ClassBuilder {
    pub fn new(access_flags: u16, this_class: &str, super_class: &str) -> ClassBuilder {
        let mut builder = ClassBuilder {
            access_flags: access_flags,
            this_class_index: 0,
            super_class_index: 0,
            constants: vec![],
            methods: vec![],
        };
        builder.this_class_index = builder.define_class(this_class);
        builder.super_class_index = builder.define_class(super_class);
        builder
    }

    pub fn define_method(&mut self, access_flags: u16, name: &str, descriptor: &str) -> MethodBuilder {
        MethodBuilder::new(self, access_flags, name, descriptor)
    }

    fn push_constant(&mut self, constant: Constant) -> u16 {
        // TODO check if this constant is exactly equal to anything already defined in constants. If so, return the existing index instead of re-defining it.
        self.constants.push(constant);
        self.constants.len() as u16
    }

    fn define_utf8(&mut self, string: &str) -> u16 {
        self.push_constant(Constant::Utf8(string.to_owned()))
    }

    fn define_class(&mut self, class: &str) -> u16 {
        let name_index = self.define_utf8(class);
        self.push_constant(Constant::Class(name_index))
    }

    fn define_string(&mut self, value: &str) -> u16 {
        let string_index = self.define_utf8(value);
        self.push_constant(Constant::String(string_index))
    }

    fn define_fieldref(&mut self, class: &str, name: &str, descriptor: &str) -> u16 {
        let class_index = self.define_class(class);
        let name_and_type_index = self.define_name_and_type(name, descriptor);
        self.push_constant(Constant::Fieldref(class_index, name_and_type_index))
    }

    fn define_methodref(&mut self, class: &str, name: &str, descriptor: &str) -> u16 {
        let class_index = self.define_class(class);
        let name_and_type_index = self.define_name_and_type(name, descriptor);
        self.push_constant(Constant::Methodref(class_index, name_and_type_index))
    }

    fn define_name_and_type(&mut self, name: &str, descriptor: &str) -> u16 {
        let name_index = self.define_utf8(name);
        let descriptor_index = self.define_utf8(descriptor);
        self.push_constant(Constant::NameAndType(name_index, descriptor_index))
    }

    pub fn done(self) -> Classfile {
        Classfile::new(self.constants, self.access_flags, self.this_class_index, self.super_class_index, self.methods)
    }
}

pub struct MethodBuilder<'a> {
    classfile: &'a mut ClassBuilder,
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    instructions: Vec<(u16, IntermediateInstruction<'a>)>,
    labels: HashMap<String, u16>,
    stack_index: u16,
    curr_stack_depth: u16,
    max_stack_depth: u16,
    stack_frames: Vec<StackMapFrame>,
    last_stack_frame_index: Option<u16>,
}

#[derive(Debug)]
pub enum IntermediateInstruction<'a> {
    Ready(Instruction),
    Waiting(&'a str, Instruction),
}

impl<'a> MethodBuilder<'a> {
    fn new(classfile: &'a mut ClassBuilder, access_flags: u16, name: &str, descriptor: &str) -> MethodBuilder<'a> {
        let name_index = classfile.define_utf8(name);
        let descriptor_index = classfile.define_utf8(descriptor);
        MethodBuilder {
            classfile: classfile,
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            instructions: vec![],
            labels: HashMap::new(),
            stack_index: 0,
            curr_stack_depth: 0,
            max_stack_depth: 0,
            stack_frames: vec![],
            last_stack_frame_index: None,
        }
    }

    pub fn bipush(&mut self, value: i8) {
        self.push_instruction(Instruction::Bipush(value as u8));
        self.increase_stack_depth();
    }

    pub fn load_constant(&mut self, value: &str) {
        let string_index = self.classfile.define_string(value);
        if string_index > ::std::u8::MAX as u16 {
            panic!("Placed a constant in too high of an index: {}", string_index)
        }
        self.push_instruction(Instruction::LoadConstant(string_index as u8));
        self.increase_stack_depth();
    }

    pub fn aload1(&mut self) {
        self.push_instruction(Instruction::Aload1);
        self.increase_stack_depth();
    }

    pub fn aload2(&mut self) {
        self.push_instruction(Instruction::Aload2);
        self.increase_stack_depth();
    }

    pub fn aload3(&mut self) {
        self.push_instruction(Instruction::Aload3);
        self.increase_stack_depth();
    }

    pub fn aload4(&mut self) {
        self.push_instruction(Instruction::Aload4);
        self.increase_stack_depth();
    }

    pub fn iadd(&mut self) {
        self.push_instruction(Instruction::Iadd);
        self.decrease_stack_depth();
    }

    pub fn ifeq(&mut self, label: &'a str) {
        self.delay_instruction(label, Instruction::IfEq(0));
        self.decrease_stack_depth();
    }

    pub fn ifne(&mut self, label: &'a str) {
        self.delay_instruction(label, Instruction::IfNe(0));
        self.decrease_stack_depth();
    }

    pub fn iflt(&mut self, label: &'a str) {
        self.delay_instruction(label, Instruction::IfLt(0));
        self.decrease_stack_depth();
    }

    pub fn ifge(&mut self, label: &'a str) {
        self.delay_instruction(label, Instruction::IfGe(0));
        self.decrease_stack_depth();
    }

    pub fn ifgt(&mut self, label: &'a str) {
        self.delay_instruction(label, Instruction::IfGt(0));
        self.decrease_stack_depth();
    }

    pub fn ifle(&mut self, label: &'a str) {
        self.delay_instruction(label, Instruction::IfLe(0));
        self.decrease_stack_depth();
    }

    pub fn goto(&mut self, label: &'a str) {
        self.delay_instruction(label, Instruction::Goto(0));
    }

    pub fn do_return(&mut self) {
        self.push_instruction(Instruction::Return);
    }

    pub fn get_static(&mut self, class: &str, name: &str, descriptor: &str) {
        let fieldref_index = self.classfile.define_fieldref(class, name, descriptor);
        self.push_instruction(Instruction::GetStatic(fieldref_index));
        self.increase_stack_depth();
    }

    pub fn invoke_virtual(&mut self, class: &str, name: &str, descriptor: &str, n_args: u8, has_result: bool) {
        let methodref_index = self.classfile.define_methodref(class, name, descriptor);
        self.push_instruction(Instruction::InvokeVirtual(methodref_index));
        self.decrease_stack_depth_by(n_args + 1);
        if has_result { self.increase_stack_depth(); }
    }

    pub fn invoke_special(&mut self, class: &str, name: &str, descriptor: &str, n_args: u8, has_result: bool) {
        let methodref_index = self.classfile.define_methodref(class, name, descriptor);
        self.push_instruction(Instruction::InvokeSpecial(methodref_index));
        self.decrease_stack_depth_by(n_args + 1);
        if has_result { self.increase_stack_depth(); }
    }

    pub fn array_length(&mut self) {
        self.push_instruction(Instruction::ArrayLength);
    }

    pub fn label(&mut self, name: &str) {
        self.labels.insert(name.to_owned(), self.stack_index);

        // create a stack map table entry
        let offset = match self.last_stack_frame_index {
            Some(i) => self.stack_index - i - 1,
            None => self.stack_index
        };
        let frame = if offset > ::std::u8::MAX as u16 {
            StackMapFrame::SameFrameExtended(offset)
        } else {
            StackMapFrame::SameFrame(offset as u8)
        };
        self.stack_frames.push(frame);
        self.last_stack_frame_index = Some(self.stack_index);
    }

    fn push_instruction(&mut self, instruction: Instruction) {
        let index = self.stack_index;
        self.stack_index += instruction.size() as u16;
        self.instructions.push((index, IntermediateInstruction::Ready(instruction)));
    }

    fn delay_instruction(&mut self, label: &'a str, instruction: Instruction) {
        let index = self.stack_index;
        self.stack_index += instruction.size() as u16;
        self.instructions.push((index, IntermediateInstruction::Waiting(label, instruction)));
    }

    fn increase_stack_depth(&mut self) {
        self.curr_stack_depth += 1;
        if self.curr_stack_depth > self.max_stack_depth {
            self.max_stack_depth = self.curr_stack_depth;
        }
    }

    fn decrease_stack_depth(&mut self) {
        self.curr_stack_depth -= 1;
    }

    fn decrease_stack_depth_by(&mut self, n: u8) {
        self.curr_stack_depth -= n as u16;
    }

    pub fn done(self) {
        if self.curr_stack_depth != 0 {
            println!("Warning: stack depth at the end of a method should be 0, but is {} instead", self.curr_stack_depth);
        }

        let classfile = self.classfile;
        let labels = self.labels;
        let real_instructions = self.instructions.into_iter().map(|(pos, ir)| match ir {
            IntermediateInstruction::Ready(i) => i,
            IntermediateInstruction::Waiting(l, i) => {
                let label_pos = labels.get(l).unwrap();
                let offset = label_pos - pos;
                fill_offset(i, offset)
            }
        }).collect();

        let stack_map_table_index = classfile.define_utf8("StackMapTable");
        let stack_map_table = Attribute::StackMapTable(stack_map_table_index, self.stack_frames);

        // TODO track max_locals counts instead of hard-coding to 1
        let code_index = classfile.define_utf8("Code");
        let code = Attribute::Code(code_index, self.max_stack_depth, 1, real_instructions, vec![], vec![stack_map_table]);

        let method = Method::new(self.access_flags, self.name_index, self.descriptor_index, vec![code]);
        classfile.methods.push(method);
    }
}

fn fill_offset(instruction: Instruction, offset: u16) -> Instruction {
    match instruction {
        Instruction::IfEq(_) => Instruction::IfEq(offset),
        Instruction::IfNe(_) => Instruction::IfNe(offset),
        Instruction::IfLt(_) => Instruction::IfLt(offset),
        Instruction::IfGe(_) => Instruction::IfGe(offset),
        Instruction::IfGt(_) => Instruction::IfGt(offset),
        Instruction::IfLe(_) => Instruction::IfLe(offset),
        Instruction::Goto(_) => Instruction::Goto(offset),
        _ => panic!("Instruction type doesn't have an offset to fill: {:?}", instruction)
    }
}
