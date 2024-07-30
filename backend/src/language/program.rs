use super::tokenizer::Tokenizer;

pub trait Program<'a> {
    type Error;
    fn tokens(&self) -> Tokenizer<'a>;
}
