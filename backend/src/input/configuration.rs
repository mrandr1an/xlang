pub enum Configuration {
    Compiled {
        platform: Platform,
        bintype: Type,
        target: Target,
        asm: Assembler,
    },
}

pub enum Platform {
    Linux,
}

pub enum Type {
    Executable,
}

pub enum Target {
    ELF64,
}

pub enum Assembler {
    Fasm,
}
