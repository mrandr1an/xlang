pub struct Fasm {}

pub enum Instruction {
    Mov { left: Operand, right: Operand },
}

pub enum Operand {
    Reg(),
    Str(),
    Int(),
    Pointer(Box<Operand>),
}

pub enum Register {}
