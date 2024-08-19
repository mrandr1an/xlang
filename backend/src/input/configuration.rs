use crate::language::parser::Parser;

pub struct HostSourceTree {
    config: Config,
}

impl HostSourceTree {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

pub enum Config {
    Compiled {
        target: Target,
        format: Format,
        bintype: Binary,
    },
}

pub enum Target {
    X8664,
}

pub enum Format {
    ELF64,
}

pub enum Binary {
    Library,
    Exec,
}

#[cfg(test)]
mod tokenizer_tests {
    use crate::language::{parser::Parser, tokenizer::Tokenizer};

    #[test]
    fn parsing_multiple_sexprs() {
        let s =
            "(hello friend) (maybe (I should give (you) )) (give you a name (Vicky!))".to_string();
        let mut parser = Parser::from(Tokenizer::new(&s));
        if let Some((parsed, parser)) = parser.parse() {
            println!("{:#?}", parsed);
            if let Some((parsed, _parser)) = parser.parse() {
                println!("{:#?}", parsed);
            }
        } else {
            panic!("None!")
        }
    }
}
