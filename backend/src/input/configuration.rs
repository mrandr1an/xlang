use crate::language::{error::SyntaxError, parser::Parser, sexpr::Sexpr, tokenizer::Tokenizer};

pub enum Configuration {
    Compiled(Target),
}

pub enum Target {
    ELF64Exec,
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

    pub fn parse(&self) -> Result<Sexpr<'a>, SyntaxError<'a>> {
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
    use crate::{
        input::configuration::Configuration,
        language::{ast::AST, sexpr::List},
    };

    #[test]
    fn parse() {
        let s = "(defun hello (name) (print name))".to_string();
        let parsed = Resorvable::from((&s, Configuration::Compiled(Target::ELF64Exec))).to_source();
        println!("{:#?}", parsed);
    }

    #[test]
    fn simple_parse() {
        let s = "(print :debug hello)".to_string();
        let parsed = Resorvable::from((&s, Configuration::Compiled(Target::ELF64Exec))).to_source();
        println!("{:#?}", parsed);
    }

    ///Ends at hello
    #[test]
    fn simple_parse2() {
        let s = "(print :debug hello) (goodbye)".to_string();
        let parsed = Resorvable::from((&s, Configuration::Compiled(Target::ELF64Exec))).to_source();
        println!("{:#?}", parsed);
    }

    #[test]
    fn failed_parse() {
        let s = "(print :debug hello".to_string();
        let parsed = Resorvable::from((&s, Configuration::Compiled(Target::ELF64Exec))).to_source();
        println!("{:#?}", parsed);
    }

    #[test]
    fn iter_parse() {
        let s = "(#main (#debug/print \"Hello World\") (#exit))".to_string();
        let parsed = Resorvable::from((&s, Configuration::Compiled(Target::ELF64Exec)))
            .to_source()
            .unwrap();
        match parsed.next() {
            Some(e) => match e {
                Ok(token) => {
                    println!("{:#?}", token)
                }
                Err(list) => match list {
                    List::Packed { car, cdr } => println!("{:#?}", car),
                    List::Void => println!("VOID"),
                },
            },
            None => println!("Nothing to parse"),
        }
    }

    #[test]
    fn new_ast() {
        let s = "(#main (#debug/print \"Hello World\") (#exit))".to_string();
        let ast = AST::new(Resorvable::from((
            &s,
            Configuration::Compiled(Target::ELF64Exec),
        )));
    }
}
