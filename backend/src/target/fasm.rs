pub struct Fasm {}

pub enum Instruction<'a> {
    Mov {
        left: Operand<'a>,
        right: Operand<'a>,
    },
    Lea {
        left: Operand<'a>,
        right: Operand<'a>,
    },
    Syscall,
}

pub enum Operand<'a> {
    Reg(Register),
    Str(&'a str),
    Int(usize),
    Label(Label<'a>),
    Pointer(Box<Operand<'a>>),
}

pub enum Register {
    Rdx,
    Rsi,
    Rdi,
    Rax,
}

pub enum Label<'a> {
    DataDef {
        data_dir: DataDirective,
        name: &'a str,
    },
    CodeDef {
        name: &'a str,
    },
}

pub struct Segment {}

pub enum DataDirective {
    Db,
}
