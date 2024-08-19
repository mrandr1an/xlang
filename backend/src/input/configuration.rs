use crate::language::{error::SyntaxError, parser::Parser, sexpr::Sexpr, tokenizer::Tokenizer};

#[derive(Debug)]
pub struct Host<'a> {
    config: Config,
    source: Vec<Sexpr<'a>>,
}

impl<'a> Host<'a> {
    fn new<'b>(config: Config, input: &'b String) -> Result<Self, Vec<SyntaxError<'b>>>
    where
        'b: 'a,
    {
        let tokenizer = Tokenizer::new(input);
        let mut parser = Parser::from(tokenizer);
        let mut source = Vec::<Sexpr<'b>>::new();
        let mut errors = Vec::<SyntaxError<'b>>::new();
        while let Some((parsed, new_parser)) = parser.parse() {
            match parsed {
                Ok(expr) => source.push(expr),
                Err(err) => errors.push(err),
            };
            parser = new_parser;
        }

        if errors.is_empty() {
            Ok(Self { config, source })
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug)]
pub enum Config {
    Compiled {
        target: Target,
        format: Format,
        bintype: Binary,
    },
}

#[derive(Debug)]
pub enum Target {
    X8664,
}

#[derive(Debug)]
pub enum Format {
    ELF64,
}

#[derive(Debug)]
pub enum Binary {
    Library,
    Exec,
}

#[cfg(test)]
mod tokenizer_tests {
    use crate::language::{parser::Parser, tokenizer::Tokenizer};

    use super::{Config, Format, Host};

    #[test]
    fn parsing_multiple_sexprs() {
        let s =
            "(hello friend) (maybe (I should give (you) )) (give you a name (Vicky!))".to_string();
        let parser = Parser::from(Tokenizer::new(&s));
        if let Some((parsed, parser)) = parser.parse() {
            println!("{:#?}", parsed);
            if let Some((parsed, _parser)) = parser.parse() {
                println!("{:#?}", parsed);
            }
        } else {
            panic!("None!")
        }
    }

    #[test]
    fn parsing_multiple_sexprs_with_host() {
        let input =
            "(hello friend) (maybe (I should give (you) )) (give you a name (Vicky!))".to_string();

        let config = Config::Compiled {
            target: super::Target::X8664,
            format: Format::ELF64,
            bintype: super::Binary::Exec,
        };

        let source = Host::new(config, &input);
        println!("{:#?}", source)
    }
}
