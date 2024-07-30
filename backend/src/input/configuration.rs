use crate::language::{program::Program, tokenizer::Tokenizer};

pub enum Configuration {
    Stdin,
    Compiled,
    Jit,
}

pub struct Resorvable<'a> {
    pub input: &'a String,
    config: Configuration,
}

impl<'a> From<(&'a String, Configuration)> for Resorvable<'a> {
    fn from(value: (&'a String, Configuration)) -> Self {
        Self {
            input: value.0,
            config: value.1,
        }
    }
}

impl<'a> Program<'a> for Resorvable<'a> {
    type Error = String;
    fn tokens(&self) -> crate::language::tokenizer::Tokenizer<'a> {
        Tokenizer::new(self.input)
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;
    use crate::input::configuration::Configuration;

    #[test]
    fn tokenize() {
        let s = "(hello (friend) (maybe) i should give you a (name) )".to_string();
        let mut t = Resorvable::from((&s, Configuration::Compiled)).tokens();
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
        println!("{:#?}", t.next().unwrap());
    }
}
