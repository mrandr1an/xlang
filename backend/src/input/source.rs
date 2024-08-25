use super::configuration::Configuration;

pub struct Source<T> {
    config: Configuration,
    abstract_code: T,
}

impl<T> Source<T> {
    pub fn new<I: AsRef<str>>(config: Configuration, input: I) -> Self {
        todo!()
    }
}
