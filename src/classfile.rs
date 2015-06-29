const CAFEBABE: u32 = 0xCAFEBABE;
const MAJOR_VERSION: u16 = 52;
const MINOR_VERSION: u16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Classfile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Constant>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<Interface>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    String(u16),
    Utf8(String),
    Class(u16),
    NameAndType(u16, u16),
    Fieldref(u16, u16),
    Methodref(u16, u16),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Interface;

#[derive(Clone, Debug, PartialEq)]
pub struct Field;

#[derive(Clone, Debug, PartialEq)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    Code(u16, u16, u16, Vec<Instruction>, Vec<ExceptionTableEntry>, Vec<Attribute>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExceptionTableEntry;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    GetStatic(u16),
    LoadConstant(u8),
    InvokeVirtual(u16),
    Bipush(u8),
    Iadd,
    Return,
}

impl Classfile {
    pub fn new(constants: Vec<Constant>, access_flags: u16, this_class: u16, super_class: u16, methods: Vec<Method>) -> Classfile {
        Classfile {
            magic: CAFEBABE,
            minor_version: MINOR_VERSION,
            major_version: MAJOR_VERSION,
            constant_pool: constants,
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces: vec![],
            fields: vec![],
            methods: methods,
            attributes: vec![],
        }
    }

    pub fn lookup_constant(&self, index: u16) -> &Constant {
        &self.constant_pool[index as usize - 1]
    }

    pub fn lookup_string(&self, index: u16) -> &str {
        let val = self.lookup_constant(index);
        match *val {
            Constant::Utf8(ref str) => str,
            _ => panic!("Wanted string, found {:?}", val)
        }
    }
}

impl Method {
    pub fn new(access_flags: u16, name_index: u16, descriptor_index: u16, attributes: Vec<Attribute>) -> Method {
        Method {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes: attributes,
        }
    }
}
