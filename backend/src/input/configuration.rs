#[derive(Debug)]
pub enum Configuration {
    Compiled {
        platform: Platform,
        bintype: Type,
        target: Target,
        asm: Assembler,
    },
}

#[derive(Debug)]
pub enum Platform {
    Linux,
}

#[derive(Debug)]
pub enum Type {
    Executable,
}

#[derive(Debug)]
pub enum Target {
    ELF64,
}

#[derive(Debug)]
pub enum Assembler {
    Fasm,
}
