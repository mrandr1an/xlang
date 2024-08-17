pub trait Assembly<'a> {
    type Error;
    fn assemble(&self) -> Result<&[u8], Self::Error>;
}
