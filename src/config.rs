use derive_getters::Getters;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {}

#[derive(Debug, Getters)]
pub struct Config {
    llms: Option<Llm>,
}

#[derive(Debug, Getters)]
pub struct Llm {
    default_alias: Option<String>,
    model_alias: Option<HashMap<String, Model>>,
}

impl Llm {
    /// check to exist alias
    pub fn exist_alias<T: AsRef<str>>(&self, alias: T) -> bool {
        match self.model_alias() {
            Some(v) => v.contains_key(&alias.as_ref().to_string()),
            None => false,
        }
    }

    pub fn exist_default_alias(&self) -> bool {
        // self.exist_alias(*self.default_alias())
        match self.default_alias() {
            Some(v) => self.exist_alias(v),
            None => false,
        }
    }
}

#[derive(Debug, Getters)]
pub struct Model {
    provider: String,
    model: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    base_url: Option<String>,
}
