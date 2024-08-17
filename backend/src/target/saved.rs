use std::{
    fs::{self, File},
    io::{BufReader, Error as IOError},
    ops::BitAndAssign,
};

use super::assembler::assembly::Assembly;

pub struct Saved<'a, A: Assembly<'a>> {
    source: &'a A,
    reader: BufReader<File>,
}

pub enum SaveError<'a, A: Assembly<'a>> {
    IO(IOError),
    AsmError(A::Error),
}

impl<'a, A: Assembly<'a>> Saved<'a, A> {
    fn new(path: &'a str, name: &'a str) -> Result<Self, SaveError<'a, A>> {
        // fs::create_dir(path)?;
        todo!()
    }
}
