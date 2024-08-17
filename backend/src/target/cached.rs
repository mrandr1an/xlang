use std::{
    fs::File,
    io::{BufReader, Error as IOError, Write},
};

use super::assembler::assembly::Assembly;

pub struct Cached<'a, A: Assembly<'a>> {
    source: &'a A,
    reader: BufReader<File>,
}

pub enum CachError<'a, A: Assembly<'a>> {
    IOError(IOError),
    AssemblyError(A::Error),
    Mutated,
}

impl<'a, A: Assembly<'a>> From<IOError> for CachError<'a, A> {
    fn from(value: IOError) -> Self {
        CachError::IOError(value)
    }
}

impl<'a, A: Assembly<'a>> Cached<'a, A> {
    pub fn new(path: &str, name: &str, asm: &'a A) -> Result<Self, CachError<'a, A>> {
        let full_path = String::from(path) + name;
        let mut file = File::create_new(full_path)?;
        file.write_all(asm.assemble().map_err(CachError::AssemblyError)?)?;
        Ok(Self {
            source: asm,
            reader: BufReader::new(file),
        })
    }
}
