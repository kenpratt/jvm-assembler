use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Java<'a> {
    Boolean, // Z
    Byte,    // B
    Char,    // C
    Short,   // S
    Int,     // I
    Long,    // J
    Float,   // F
    Double,  // D
    Void,    // V
    Class(&'a str), // Lclassname;
    Array(Box<Java<'a>>), // [type
}

impl<'a> fmt::Display for Java<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Java::Boolean      => write!(f, "Z"),
            Java::Byte         => write!(f, "B"),
            Java::Char         => write!(f, "C"),
            Java::Short        => write!(f, "S"),
            Java::Int          => write!(f, "I"),
            Java::Long         => write!(f, "J"),
            Java::Float        => write!(f, "F"),
            Java::Double       => write!(f, "D"),
            Java::Void         => write!(f, "V"),
            Java::Class(ref s) => write!(f, "L{};", s),
            Java::Array(ref t) => write!(f, "[{}", t),
        }
    }
}

pub fn method_signature(argument_types: &[Java], return_type: &Java) -> String {
    let mut args = "".to_owned();
    for t in argument_types {
        args.push_str(&format!("{}", t));
    }
    format!("({}){}", args, return_type)
}
