use std::fmt;

use classfile::*;

impl fmt::Display for Classfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Magic: 0x{:X}", self.magic));
        try!(writeln!(f, "Minor version: {}", self.minor_version));
        try!(writeln!(f, "Major version: {}", self.major_version));
        try!(writeln!(f, "Constant pool:"));
        let constant_pool_with_indices: Vec<(u16, &Constant)> = self.constant_pool.iter().enumerate().map(|(i, v)| (i as u16 + 1, v)).collect();
        try!(constant_pool_with_indices.pretty_println(f, 2));
        try!(writeln!(f, "Access flags: 0x{:X}", self.access_flags));
        try!(writeln!(f, "This class: {}", self.this_class));
        try!(writeln!(f, "Super class: {}", self.super_class));
        try!(writeln!(f, "Interfaces:"));
        try!(self.interfaces.pretty_println(f, 2));
        try!(writeln!(f, "Fields:"));
        try!(self.fields.pretty_println(f, 2));
        try!(writeln!(f, "Methods:"));
        try!(self.methods.pretty_println(f, 2));
        try!(writeln!(f, "Attributes:"));
        Ok(())
    }
}

trait PrettyPrint {
    fn pretty_print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result;

    fn pretty_println(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        try!(self.pretty_print(f, indent));
        write!(f, "\n")
    }

    fn pretty_print_preln(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        try!(write!(f, "\n"));
        self.pretty_print(f, indent)
    }
}

impl<T: PrettyPrint> PrettyPrint for Vec<T> {
    fn pretty_print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let mut count = 0;
        for item in self {
            try!(write!(f, "{0:1$}", "", indent));
            try!(item.pretty_print(f, indent + 2));
            count += 1;
            if count < self.len() {
                try!(write!(f, "\n"));
            }
        }
        Ok(())
    }

    fn pretty_println(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        if self.len() > 0 {
            try!(self.pretty_print(f, indent));
            write!(f, "\n")
        } else {
            Ok(())
        }
    }


    fn pretty_print_preln(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        if self.len() > 0 {
            try!(write!(f, "\n"));
            self.pretty_print(f, indent)
        } else {
            Ok(())
        }
    }
}

impl<T: PrettyPrint, U: PrettyPrint> PrettyPrint for (T, U) {
    fn pretty_print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let (ref t, ref u) = *self;
        try!(t.pretty_print(f, indent));
        try!(write!(f, ": "));
        try!(u.pretty_print(f, indent));
        Ok(())
    }
}

impl PrettyPrint for u8 {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:2}", self)
    }
}

impl PrettyPrint for u16 {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:2}", self)
    }
}

impl PrettyPrint for u32 {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:2}", self)
    }
}

impl<'a> PrettyPrint for &'a Constant {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PrettyPrint for Interface {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PrettyPrint for Field {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PrettyPrint for Method {
    fn pretty_print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        try!(write!(f, "Method(access_flags: 0x{:X}, name_index: {}, descriptor_index: {})\n", self.access_flags, self.name_index, self.descriptor_index));
        try!(write!(f, "{0:1$}Attributes:", "", indent));
        try!(self.attributes.pretty_print_preln(f, indent + 2));
        Ok(())
    }
}

impl PrettyPrint for Attribute {
    fn pretty_print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        match *self {
            Attribute::Code(_, max_stack, max_locals, ref code, ref exception_table, ref attributes) => {
                try!(write!(f, "Code(max_stack: {}, max_locals: {})\n", max_stack, max_locals));
                try!(write!(f, "{0:1$}Instructions:\n", "", indent));
                try!(code.pretty_println(f, indent + 2));
                try!(write!(f, "{0:1$}Exception table:\n", "", indent));
                try!(exception_table.pretty_println(f, indent + 2));
                try!(write!(f, "{0:1$}Attributes:", "", indent));
                try!(attributes.pretty_print_preln(f, indent + 2));
                Ok(())
            },
            Attribute::LineNumberTable(_, ref entries) => {
                try!(write!(f, "LineNumberTable:"));
                try!(entries.pretty_print_preln(f, indent));
                Ok(())
            },
            Attribute::SourceFile(_, index) => {
                try!(write!(f, "SourceFile(index: {}):", index));
                Ok(())
            }
            Attribute::StackMapTable(_, ref entries) => {
                try!(write!(f, "StackMapTable:"));
                try!(entries.pretty_print_preln(f, indent));
                Ok(())
            },
        }
    }
}

impl PrettyPrint for ExceptionTableEntry {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PrettyPrint for LineNumberTableEntry {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "start_pc: {:2}, line_number: {:2}", self.start_pc, self.line_number)
    }
}

impl PrettyPrint for StackMapFrame {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PrettyPrint for VerificationType {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PrettyPrint for Instruction {
    fn pretty_print(&self, f: &mut fmt::Formatter, _indent: usize) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
