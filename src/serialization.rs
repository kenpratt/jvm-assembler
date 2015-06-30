use std::io::Read;

use classfile::*;

impl Classfile {
    pub fn serialize(self, buf: &mut Vec<u8>) {
        self.magic.serialize(buf);
        self.minor_version.serialize(buf);
        self.major_version.serialize(buf);
        self.constant_pool.serialize(buf);
        self.access_flags.serialize(buf);
        self.this_class.serialize(buf);
        self.super_class.serialize(buf);
        self.interfaces.serialize(buf);
        self.fields.serialize(buf);
        self.methods.serialize(buf);
        self.attributes.serialize(buf);
    }

    pub fn deserialize(stream: Box<Read>) -> Classfile {
        let mut buf = &mut Deserializer::new(Box::new(stream.bytes().map(|r| r.unwrap())));
        let mut c = Classfile {
            magic: 0,
            minor_version: 0,
            major_version: 0,
            constant_pool: vec![],
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        };
        c.magic = u32::deserialize(buf, &c);
        c.minor_version = u16::deserialize(buf, &c);
        c.major_version = u16::deserialize(buf, &c);
        c.constant_pool = Vec::deserialize(buf, &c);
        c.access_flags = u16::deserialize(buf, &c);
        c.this_class = u16::deserialize(buf, &c);
        c.super_class = u16::deserialize(buf, &c);
        c.interfaces = Vec::deserialize(buf, &c);
        c.fields = Vec::deserialize(buf, &c);
        c.methods = Vec::deserialize(buf, &c);
        c.attributes = Vec::deserialize(buf, &c);
        c
    }
}

struct Deserializer {
    stream: Box<Iterator<Item=u8>>,
    bytes_taken: u32,
}

impl Deserializer {
    fn new(stream: Box<Iterator<Item=u8>>) -> Deserializer {
        Deserializer { stream: stream, bytes_taken: 0 }
    }

    fn take_byte(&mut self) -> u8 {
        let v = self.take_bytes(1);
        v[0]
    }

    fn take_bytes(&mut self, n: u32) -> Vec<u8> {
        self.bytes_taken += n;
        (&mut self.stream).take(n as usize).collect()
    }
}

trait Serializable {
    fn serialize(self, &mut Vec<u8>);
    fn deserialize(&mut Deserializer, &Classfile) -> Self;
}

impl Serializable for u8 {
    fn serialize(self, buf: &mut Vec<u8>) {
        buf.push(self)
    }

    fn deserialize(buf: &mut Deserializer, _classfile: &Classfile) -> u8 {
        buf.take_byte()
    }
}

impl Serializable for u16 {
    fn serialize(self, buf: &mut Vec<u8>) {
        buf.push((self >> 8) as u8);
        buf.push(self as u8);
    }

    fn deserialize(buf: &mut Deserializer, _classfile: &Classfile) -> u16 {
        let v = buf.take_bytes(2);
        ((v[0] as u16) << 8) + (v[1] as u16)
    }
}

impl Serializable for u32 {
    fn serialize(self, buf: &mut Vec<u8>) {
        buf.push((self >> 24) as u8);
        buf.push((self >> 16) as u8);
        buf.push((self >> 8) as u8);
        buf.push(self as u8);
    }

    fn deserialize(buf: &mut Deserializer, _classfile: &Classfile) -> u32 {
        let v = buf.take_bytes(4);
        ((v[0] as u32) << 24) + ((v[1] as u32) << 16) + ((v[2] as u32) << 8) + (v[3] as u32)
    }
}

impl Serializable for String {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u16).serialize(buf);
        for b in self.as_bytes() {
            b.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> String {
        let len = u16::deserialize(buf, classfile);
        let v = buf.take_bytes(len as u32);
        String::from_utf8(v).unwrap()
    }
}

impl Serializable for Vec<u8> {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u32).serialize(buf); // byte vectors use a 4-byte length prefix, not 2-byte
        for b in self.into_iter() {
            b.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<u8> {
        let len = u32::deserialize(buf, classfile); // byte vectors use a 4-byte length prefix, not 2-byte
        buf.take_bytes(len)
    }
}

impl Serializable for Vec<Constant> {
    fn serialize(self, buf: &mut Vec<u8>) {
        ((self.len() + 1) as u16).serialize(buf); // IMPORTANT: constant_pool_length is len + 1
        for constant in self.into_iter() {
            constant.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<Constant> {
        let len = u16::deserialize(buf, classfile) - 1; // IMPORTANT: constant_pool_length is len + 1
        (0..len).into_iter().map(|_| Constant::deserialize(buf, classfile)).collect()
    }
}

impl Serializable for Vec<Interface> {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u16).serialize(buf);
        for constant in self.into_iter() {
            constant.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<Interface> {
        let len = u16::deserialize(buf, classfile);
        (0..len).into_iter().map(|_| Interface::deserialize(buf, classfile)).collect()
    }
}

impl Serializable for Vec<Field> {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u16).serialize(buf);
        for constant in self.into_iter() {
            constant.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<Field> {
        let len = u16::deserialize(buf, classfile);
        (0..len).into_iter().map(|_| Field::deserialize(buf, classfile)).collect()
    }
}

impl Serializable for Vec<Method> {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u16).serialize(buf);
        for constant in self.into_iter() {
            constant.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<Method> {
        let len = u16::deserialize(buf, classfile);
        (0..len).into_iter().map(|_| Method::deserialize(buf, classfile)).collect()
    }
}

impl Serializable for Vec<Attribute> {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u16).serialize(buf);
        for constant in self.into_iter() {
            constant.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<Attribute> {
        let len = u16::deserialize(buf, classfile);
        (0..len).into_iter().map(|_| Attribute::deserialize(buf, classfile)).collect()
    }
}

impl Serializable for Vec<ExceptionTableEntry> {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u16).serialize(buf);
        for constant in self.into_iter() {
            constant.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<ExceptionTableEntry> {
        let len = u16::deserialize(buf, classfile);
        (0..len).into_iter().map(|_| ExceptionTableEntry::deserialize(buf, classfile)).collect()
    }
}

impl Serializable for Vec<LineNumberTableEntry> {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u16).serialize(buf);
        for constant in self.into_iter() {
            constant.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<LineNumberTableEntry> {
        let len = u16::deserialize(buf, classfile);
        (0..len).into_iter().map(|_| LineNumberTableEntry::deserialize(buf, classfile)).collect()
    }
}

impl Serializable for Vec<StackMapFrame> {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u16).serialize(buf);
        for constant in self.into_iter() {
            constant.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<StackMapFrame> {
        let len = u16::deserialize(buf, classfile);
        (0..len).into_iter().map(|_| StackMapFrame::deserialize(buf, classfile)).collect()
    }
}

impl Serializable for Vec<VerificationType> {
    fn serialize(self, buf: &mut Vec<u8>) {
        (self.len() as u16).serialize(buf);
        for constant in self.into_iter() {
            constant.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<VerificationType> {
        let len = u16::deserialize(buf, classfile);
        (0..len).into_iter().map(|_| VerificationType::deserialize(buf, classfile)).collect()
    }
}

impl Serializable for Vec<Instruction> {
    fn serialize(self, buf: &mut Vec<u8>) {
        let mut code = vec![];
        for inst in self.into_iter() {
            inst.serialize(&mut code);
        }
        code.serialize(buf);
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Vec<Instruction> {
        let code: Vec<u8> = Vec::deserialize(buf, classfile);
        let code_len = code.len() as u32;
        let mut code_buf = &mut Deserializer::new(Box::new(code.into_iter()));
        let mut out = vec![];
        while code_buf.bytes_taken < code_len {
            out.push(Instruction::deserialize(code_buf, classfile));
        }
        out
    }
}

impl Serializable for Constant {
    fn serialize(self, buf: &mut Vec<u8>) {
        match self {
            Constant::Utf8(string) => {
                (1 as u8).serialize(buf);
                string.serialize(buf);
            },
            Constant::Class(name_index) => {
                (7 as u8).serialize(buf);
                name_index.serialize(buf);
            },
            Constant::String(string_index) => {
                (8 as u8).serialize(buf);
                string_index.serialize(buf);
            },
            Constant::Fieldref(class_index, name_and_type_index) => {
                (9 as u8).serialize(buf);
                class_index.serialize(buf);
                name_and_type_index.serialize(buf);
            },
            Constant::Methodref(class_index, name_and_type_index) => {
                (10 as u8).serialize(buf);
                class_index.serialize(buf);
                name_and_type_index.serialize(buf);
            },
            Constant::NameAndType(name_index, descriptor_index) => {
                (12 as u8).serialize(buf);
                name_index.serialize(buf);
                descriptor_index.serialize(buf);
            },
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Constant {
        let code = u8::deserialize(buf, classfile);
        match code {
            1 => Constant::Utf8(String::deserialize(buf, classfile)),
            7 => Constant::Class(u16::deserialize(buf, classfile)),
            8 => Constant::String(u16::deserialize(buf, classfile)),
            9 => Constant::Fieldref(u16::deserialize(buf, classfile), u16::deserialize(buf, classfile)),
            10 => Constant::Methodref(u16::deserialize(buf, classfile), u16::deserialize(buf, classfile)),
            12 => Constant::NameAndType(u16::deserialize(buf, classfile), u16::deserialize(buf, classfile)),
            _ => panic!("Don't know how to deserialize Constant of type: {}", code)
        }
    }
}

impl Serializable for Interface {
    fn serialize(self, _buf: &mut Vec<u8>) {
        panic!("TODO implement Interface::serialize")
    }

    fn deserialize(_buf: &mut Deserializer, _classfile: &Classfile) -> Interface {
        panic!("TODO implement Interface::deserialize")
    }
}

impl Serializable for Field {
    fn serialize(self, _buf: &mut Vec<u8>) {
        panic!("TODO implement Field::serialize")
    }

    fn deserialize(_buf: &mut Deserializer, _classfile: &Classfile) -> Field {
        panic!("TODO implement Field::deserialize")
    }
}

impl Serializable for Method {
    fn serialize(self, buf: &mut Vec<u8>) {
        self.access_flags.serialize(buf);
        self.name_index.serialize(buf);
        self.descriptor_index.serialize(buf);
        self.attributes.serialize(buf);
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Method {
        Method {
            access_flags: u16::deserialize(buf, classfile),
            name_index: u16::deserialize(buf, classfile),
            descriptor_index: u16::deserialize(buf, classfile),
            attributes: Vec::deserialize(buf, classfile),
        }
    }
}

impl Serializable for Attribute {
    fn serialize(self, buf: &mut Vec<u8>) {
        // generate a temporary buffer holding the attribute "body"
        let mut attribute_body = vec![];
        let mut attribute_name_index;

        {
            let mut body_buf = &mut attribute_body;
            match self {
                Attribute::Code(name_index, max_stack, max_locals, code, exception_table, attributes) => {
                    attribute_name_index = name_index;

                    max_stack.serialize(body_buf);
                    max_locals.serialize(body_buf);
                    code.serialize(body_buf);
                    exception_table.serialize(body_buf);
                    attributes.serialize(body_buf);
                },
                Attribute::LineNumberTable(name_index, entries) => {
                    attribute_name_index = name_index;
                    entries.serialize(body_buf);
                },
                Attribute::SourceFile(name_index, sourcefile_index) => {
                    attribute_name_index = name_index;
                    sourcefile_index.serialize(body_buf);
                },
                Attribute::StackMapTable(name_index, entries) => {
                    attribute_name_index = name_index;
                    entries.serialize(body_buf);
                },
            }
        }

        // append the attribute body to the real buffer
        attribute_name_index.serialize(buf);
        attribute_body.serialize(buf);
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Attribute {
        let attribute_name_index = u16::deserialize(buf, classfile);
        let attribute_name = classfile.lookup_string(attribute_name_index);

        let attribute_body: Vec<u8> = Vec::deserialize(buf, classfile);
        let mut buf2 = &mut Deserializer::new(Box::new(attribute_body.into_iter()));

        match attribute_name {
            "Code" => {
                let max_stack = u16::deserialize(buf2, classfile);
                let max_locals = u16::deserialize(buf2, classfile);
                let code = Vec::deserialize(buf2, classfile);
                let exception_table = Vec::deserialize(buf2, classfile);
                let attributes = Vec::deserialize(buf2, classfile);
                Attribute::Code(attribute_name_index, max_stack, max_locals, code, exception_table, attributes)
            },
            "LineNumberTable" => {
                let entries = Vec::deserialize(buf2, classfile);
                Attribute::LineNumberTable(attribute_name_index, entries)
            },
            "SourceFile" => {
                let sourcefile_index = u16::deserialize(buf2, classfile);
                Attribute::SourceFile(attribute_name_index, sourcefile_index)
            },
            "StackMapTable" => {
                let entries = Vec::deserialize(buf2, classfile);
                Attribute::StackMapTable(attribute_name_index, entries)
            },
            _ => panic!("TODO implement Attribute::deserialize for attribute type: {:?}", attribute_name)

        }
    }
}

impl Serializable for ExceptionTableEntry {
    fn serialize(self, _buf: &mut Vec<u8>) {
        panic!("TODO implement ExceptionTableEntry::serialize")
    }

    fn deserialize(_buf: &mut Deserializer, _classfile: &Classfile) -> ExceptionTableEntry {
        panic!("TODO implement ExceptionTableEntry::deserialize")
    }
}

impl Serializable for LineNumberTableEntry {
    fn serialize(self, buf: &mut Vec<u8>) {
        self.start_pc.serialize(buf);
        self.line_number.serialize(buf);
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> LineNumberTableEntry {
        LineNumberTableEntry {
            start_pc: u16::deserialize(buf, classfile),
            line_number: u16::deserialize(buf, classfile),
        }
    }
}

impl Serializable for StackMapFrame {
    fn serialize(self, buf: &mut Vec<u8>) {
        match self {
            StackMapFrame::SameFrame(offset_delta) => {
                let frame_type = offset_delta;
                frame_type.serialize(buf);
            },
            StackMapFrame::SameLocals1StackItemFrame(offset_delta, verification_type) => {
                let frame_type = offset_delta + 64;
                frame_type.serialize(buf);
                verification_type.serialize(buf);
            },
            StackMapFrame::SameLocals1StackItemFrameExtended(offset_delta, verification_type) => {
                let frame_type: u8 = 247;
                frame_type.serialize(buf);
                offset_delta.serialize(buf);
                verification_type.serialize(buf);
            },
            StackMapFrame::ChopFrame(k, offset_delta) => {
                let frame_type = 251 - k;
                frame_type.serialize(buf);
                offset_delta.serialize(buf);
            },
            StackMapFrame::SameFrameExtended(offset_delta) => {
                let frame_type: u8 = 251;
                frame_type.serialize(buf);
                offset_delta.serialize(buf);
            },
            StackMapFrame::AppendFrame(k, offset_delta, locals) => {
                let frame_type = 251 + k;
                frame_type.serialize(buf);
                offset_delta.serialize(buf);
                for local in locals {
                    local.serialize(buf);
                }
            },
            StackMapFrame::FullFrame(offset_delta, locals, stack_items) => {
                let frame_type: u8 = 255;
                frame_type.serialize(buf);
                offset_delta.serialize(buf);
                locals.serialize(buf);
                stack_items.serialize(buf);
            },
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> StackMapFrame {
        let frame_type = u8::deserialize(buf, classfile);
        match frame_type {
            0...63 => {
                let offset_delta = frame_type;
                StackMapFrame::SameFrame(offset_delta)
            },
            64...127 => {
                let offset_delta = frame_type - 64;
                let verification_type = VerificationType::deserialize(buf, classfile);
                StackMapFrame::SameLocals1StackItemFrame(offset_delta, verification_type)
            },
            247 => {
                let offset_delta = u16::deserialize(buf, classfile);
                let verification_type = VerificationType::deserialize(buf, classfile);
                StackMapFrame::SameLocals1StackItemFrameExtended(offset_delta, verification_type)
            },
            248...250 => {
                let k = 251 - frame_type;
                let offset_delta = u16::deserialize(buf, classfile);
                StackMapFrame::ChopFrame(k, offset_delta)
            },
            251 => {
                let offset_delta = u16::deserialize(buf, classfile);
                StackMapFrame::SameFrameExtended(offset_delta)
            },
            252...254 => {
                let k = frame_type - 251;
                let offset_delta = u16::deserialize(buf, classfile);
                let locals = (0..k).into_iter().map(|_| VerificationType::deserialize(buf, classfile)).collect();
                StackMapFrame::AppendFrame(k, offset_delta, locals)
            },
            255 => {
                let offset_delta = u16::deserialize(buf, classfile);
                let locals = Vec::deserialize(buf, classfile);
                let stack_items = Vec::deserialize(buf, classfile);
                StackMapFrame::FullFrame(offset_delta, locals, stack_items)
            },
            _ => panic!("Invalid StackMapFrame type: {}", frame_type)
        }
    }
}

impl Serializable for VerificationType {
    fn serialize(self, buf: &mut Vec<u8>) {
        match self {
            VerificationType::Top => {
                (0 as u8).serialize(buf);
            },
            VerificationType::Integer => {
                (1 as u8).serialize(buf);
            },
            VerificationType::Float => {
                (2 as u8).serialize(buf);
            },
            VerificationType::Long => {
                (3 as u8).serialize(buf);
            },
            VerificationType::Double => {
                (4 as u8).serialize(buf);
            },
            VerificationType::Null => {
                (5 as u8).serialize(buf);
            },
            VerificationType::UninitializedThis => {
                (6 as u8).serialize(buf);
            },
            VerificationType::Object(cpool_index) => {
                (7 as u8).serialize(buf);
                cpool_index.serialize(buf);
            },
            VerificationType::Uninitialized(offset) => {
                (8 as u8).serialize(buf);
                offset.serialize(buf);
            },
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> VerificationType {
        let verification_type = u8::deserialize(buf, classfile);
        match verification_type {
            0 => {
                VerificationType::Top
            },
            1 => {
                VerificationType::Integer
            },
            2 => {
                VerificationType::Float
            },
            3 => {
                VerificationType::Long
            },
            4 => {
                VerificationType::Double
            },
            5 => {
                VerificationType::Null
            },
            6 => {
                VerificationType::UninitializedThis
            },
            7 => {
                let cpool_index = u16::deserialize(buf, classfile);
                VerificationType::Object(cpool_index)
            },
            8 => {
                let offset = u16::deserialize(buf, classfile);
                VerificationType::Uninitialized(offset)
            },
            _ => panic!("Invalid VerificationType: {}", verification_type)
        }
    }
}

impl Serializable for Instruction {
    fn serialize(self, buf: &mut Vec<u8>) {
        match self {
            Instruction::IconstM1 => {
                (0x2 as u8).serialize(buf);
            },
            Instruction::Iconst0 => {
                (0x3 as u8).serialize(buf);
            },
            Instruction::Iconst1 => {
                (0x4 as u8).serialize(buf);
            },
            Instruction::Iconst2 => {
                (0x5 as u8).serialize(buf);
            },
            Instruction::Iconst3 => {
                (0x6 as u8).serialize(buf);
            },
            Instruction::Iconst4 => {
                (0x7 as u8).serialize(buf);
            },
            Instruction::Iconst5 => {
                (0x8 as u8).serialize(buf);
            },
            Instruction::Bipush(val) => {
                (0x10 as u8).serialize(buf);
                val.serialize(buf);
            },
            Instruction::LoadConstant(index) => {
                (0x12 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::Aload0 => {
                (0x2A as u8).serialize(buf);
            },
            Instruction::Aload1 => {
                (0x2B as u8).serialize(buf);
            },
            Instruction::Aload2 => {
                (0x2C as u8).serialize(buf);
            },
            Instruction::Aload3 => {
                (0x2D as u8).serialize(buf);
            },
            Instruction::Aaload => {
                (0x32 as u8).serialize(buf);
            },
            Instruction::IfEq(index) => {
                (0x99 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfNe(index) => {
                (0x9A as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfLt(index) => {
                (0x9B as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfGe(index) => {
                (0x9C as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfGt(index) => {
                (0x9D as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfLe(index) => {
                (0x9E as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfIcmpEq(index) => {
                (0x9F as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfIcmpNe(index) => {
                (0xA0 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfIcmpLt(index) => {
                (0xA1 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfIcmpGe(index) => {
                (0xA2 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfIcmpGt(index) => {
                (0xA3 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::IfIcmpLe(index) => {
                (0xA4 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::Goto(index) => {
                (0xA7 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::Iadd => {
                (0x60 as u8).serialize(buf);
            },
            Instruction::Return => {
                (0xB1 as u8).serialize(buf);
            },
            Instruction::GetStatic(index) => {
                (0xB2 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::InvokeVirtual(index) => {
                (0xB6 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::InvokeSpecial(index) => {
                (0xB7 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::InvokeStatic(index) => {
                (0xB8 as u8).serialize(buf);
                index.serialize(buf);
            },
            Instruction::ArrayLength => {
                (0xBE as u8).serialize(buf);
            },
        }
    }

    fn deserialize(buf: &mut Deserializer, classfile: &Classfile) -> Instruction {
        let code = u8::deserialize(buf, classfile);
        match code {
            0x02 => Instruction::IconstM1,
            0x03 => Instruction::Iconst0,
            0x04 => Instruction::Iconst1,
            0x05 => Instruction::Iconst2,
            0x06 => Instruction::Iconst3,
            0x07 => Instruction::Iconst4,
            0x08 => Instruction::Iconst5,
            0x10 => Instruction::Bipush(u8::deserialize(buf, classfile)),
            0x12 => Instruction::LoadConstant(u8::deserialize(buf, classfile)),
            0x2A => Instruction::Aload0,
            0x2B => Instruction::Aload1,
            0x2C => Instruction::Aload2,
            0x2D => Instruction::Aload3,
            0x32 => Instruction::Aaload,
            0x99 => Instruction::IfEq(u16::deserialize(buf, classfile)),
            0x9A => Instruction::IfNe(u16::deserialize(buf, classfile)),
            0x9B => Instruction::IfLt(u16::deserialize(buf, classfile)),
            0x9C => Instruction::IfGe(u16::deserialize(buf, classfile)),
            0x9D => Instruction::IfGt(u16::deserialize(buf, classfile)),
            0x9E => Instruction::IfLe(u16::deserialize(buf, classfile)),
            0x9F => Instruction::IfIcmpEq(u16::deserialize(buf, classfile)),
            0xA0 => Instruction::IfIcmpNe(u16::deserialize(buf, classfile)),
            0xA1 => Instruction::IfIcmpLt(u16::deserialize(buf, classfile)),
            0xA2 => Instruction::IfIcmpGe(u16::deserialize(buf, classfile)),
            0xA3 => Instruction::IfIcmpGt(u16::deserialize(buf, classfile)),
            0xA4 => Instruction::IfIcmpLe(u16::deserialize(buf, classfile)),
            0xA7 => Instruction::Goto(u16::deserialize(buf, classfile)),
            0x60 => Instruction::Iadd,
            0xB1 => Instruction::Return,
            0xB2 => Instruction::GetStatic(u16::deserialize(buf, classfile)),
            0xB6 => Instruction::InvokeVirtual(u16::deserialize(buf, classfile)),
            0xB7 => Instruction::InvokeSpecial(u16::deserialize(buf, classfile)),
            0xB8 => Instruction::InvokeStatic(u16::deserialize(buf, classfile)),
            0xBE => Instruction::ArrayLength,
            _ => panic!("Don't know how to deserialize Instruction of type: 0x{:X}", code)
        }

    }
}
