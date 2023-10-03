use crate::configuration::Configuration;

pub struct Runtime<C: Configuration> {
    config: C,
}

impl<C: Configuration> Runtime<C> {
    pub fn new(config: C) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &C {
        &self.config
    }
}
