use std::fmt::Display;

#[derive(Clone)]
pub enum TypeInstance {
    Int,
    Char,
    Short,
    Long,
    LongLong,
    Func(Box<TypeInstance>, String, Option<Vec<TypeInstance>>),
    Ptr(Box<TypeInstance>),
    Void,
}

impl Display for TypeInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeInstance::Char => write!(f, "char"),
            TypeInstance::Int => write!(f, "int"),
            TypeInstance::Short => write!(f, "short"),
            TypeInstance::Long => write!(f, "long"),
            TypeInstance::LongLong => write!(f, "long long"),
            TypeInstance::Void => write!(f, "void"),
            TypeInstance::Func(_type, name, params) => {
                write!(f, "{} {}", _type, name)?;
                if let Some(params) = params {
                    for param in params {
                        write!(f, "{},", param)?;
                    }
                } else {
                    write!(f, "()")?;
                }
                writeln!(f, "")
            }
            TypeInstance::Ptr(_type) => {
                write!(f, "*{}", _type)
            }
        }
    }
}
