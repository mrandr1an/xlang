use std::fmt::Display;

pub struct Fasm<'a> {
    format: Format,
    rest: Vec<Segment<'a>>,
}

impl<'a> Display for Fasm<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format)?;
        for seg in self.rest.iter() {
            write!(f, "{}", seg)?;
        }
        writeln!(f)
    }
}

pub enum Format {
    ELF64_EXEC,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ELF64_EXEC => writeln!(f, "format ELF64 executable"),
        }
    }
}

pub enum Instruction<'a> {
    Mov {
        left: Operand<'a>,
        right: Operand<'a>,
    },
    Lea {
        left: Operand<'a>,
        right: Operand<'a>,
    },
    Xor {
        left: Operand<'a>,
        right: Operand<'a>,
    },
    Syscall,
}

impl<'a> AsRef<[u8]> for Instruction<'a> {
    fn as_ref(&self) -> &[u8] {
        todo!()
    }
}

impl<'a> Display for Instruction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov { left, right } => {
                writeln!(f, "mov {},{}", left, right)
            }
            Instruction::Xor { left, right } => {
                writeln!(f, "xor {},{}", left, right)
            }
            Instruction::Lea { left, right } => {
                writeln!(f, "lea {},{}", left, right)
            }
            Instruction::Syscall => writeln!(f, "syscall"),
        }
    }
}

pub enum Operand<'a> {
    Reg(Register),
    Str(&'a str),
    Int(usize),
    Label(Label<'a>),
    Pointer(Box<Operand<'a>>),
}

impl<'a> Display for Operand<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => write!(f, "{}", reg),
            Operand::Str(s) => write!(f, "{}", s),
            Operand::Int(num) => write!(f, "{}", num),
            Operand::Label(label) => match label {
                Label::DataDef { name, .. } => write!(f, "{}", name),
                Label::CodeDef { name, .. } => write!(f, "{}", name),
            },
            Operand::Pointer(ptr) => write!(f, "[{}]", ptr),
        }
    }
}

pub enum Register {
    Rdx,
    Rsi,
    Rdi,
    Rax,
}

impl AsRef<[u8]> for Register {
    fn as_ref(&self) -> &[u8] {
        match self {
            Register::Rax => b"rax",
            Register::Rsi => b"rsi",
            Register::Rdi => b"rdi",
            Register::Rdx => b"rdx",
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Rax => write!(f, "rax"),
            Register::Rsi => write!(f, "rsi"),
            Register::Rdi => write!(f, "rdi"),
            Register::Rdx => write!(f, "rdx"),
        }
    }
}

pub enum Label<'a> {
    DataDef {
        data_dir: DataDirective,
        name: &'a str,
    },
    CodeDef {
        name: &'a str,
        instructions: Vec<Instruction<'a>>,
    },
}

impl<'a> Display for Label<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Label::DataDef { data_dir, name } => {
                write!(f, "{} ", name)?;
                write!(f, "{}\n", data_dir)
            }
            Label::CodeDef { name, instructions } => {
                write!(f, "{}", name)?;
                for instruction in instructions {
                    write!(f, "{}", instruction)?;
                }
                writeln!(f)
            }
        }
    }
}

pub struct Segment<'a> {
    name: &'a str,
    executable: bool,
    readable: bool,
    writable: bool,
    labels: Vec<Label<'a>>,
}

impl<'a> Display for Segment<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "segment {}", self.name)?;

        if self.readable {
            write!(f, " readable")?;
        }

        if self.writable {
            write!(f, " writable")?;
        }

        if self.executable {
            write!(f, " executable")?;
        }

        for label in self.labels.iter() {
            write!(f, "{}", label)?;
        }

        writeln!(f)
    }
}

pub enum DataDirective {
    Db,
}

impl AsRef<[u8]> for DataDirective {
    fn as_ref(&self) -> &[u8] {
        b"db"
    }
}

impl Display for DataDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataDirective::Db => write!(f, "db"),
        }
    }
}

#[derive(Debug)]
pub enum FasmError {
    NotCompiled,
    FasmNotFound,
}
