use crate::input::configuration::Resorvable;

use super::sexpr::Sexpr;

pub struct AST<'a> {
    config: Resorvable<'a>,
    source: Vec<Sexpr<'a>>,
}

impl<'a> AST<'a> {
    pub fn new(config: Resorvable<'a>) -> Self {
        Self {
            config,
            source: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        match self.config.parse() {
            Ok(expr) => self.source.push(expr),
            Err(err) => panic!("{:#?}", err),
        }
    }
}
