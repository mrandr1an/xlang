use crate::language::{error::SyntaxError, parser::Parser, sexpr::Sexpr, tokenizer::Tokenizer};

pub enum Configuration {
    Compiled,
}

pub struct Resorvable<'a> {
    pub input: &'a String,
    config: Configuration,
}

impl<'a> Resorvable<'a> {
    fn to_source(&self) -> Result<Sexpr<'a>, SyntaxError<'a>> {
        let tokenizer = Tokenizer::new(self.input);
        let mut parser = Parser::from(tokenizer);
        parser.parse()
    }
}

impl<'a> From<(&'a String, Configuration)> for Resorvable<'a> {
    fn from(value: (&'a String, Configuration)) -> Self {
        Self {
            input: value.0,
            config: value.1,
        }
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;
    use crate::{input::configuration::Configuration, language::parser::Parser};

    #[test]
    fn parse() {
        let s = "(defun hello (name) (print name))".to_string();
        let parsed = Resorvable::from((&s, Configuration::Compiled)).to_source();
        println!("{:#?}", parsed);
    }

    #[test]
    fn simple_parse() {
        let s = "(print :debug hello)".to_string();
        let parsed = Resorvable::from((&s, Configuration::Compiled)).to_source();
        println!("{:#?}", parsed);
    }

    #[test]
    fn failed_parse() {
        let s = "(print :debug hello".to_string();
        let parsed = Resorvable::from((&s, Configuration::Compiled)).to_source();
        println!("{:#?}", parsed);
    }
}
