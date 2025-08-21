use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    NotFoundAlias,
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub struct Config {
    llms: Option<Llm>,
}

impl Config {
    pub fn exist_alias<T: AsRef<str>>(&self, alias: T) -> bool {
        match &self.llms {
            Some(v) => match &v.model_alias {
                Some(vv) => vv.contains_key(&alias.as_ref().to_string()),
                None => false,
            },
            None => false,
        }
    }

    fn get_alias<T: AsRef<str>>(&self, alias: T) -> Option<Model> {
        match &self.llms {
            Some(v) => Some(v.get_model(alias)),
            None => None,
        }
        .flatten()
    }
}

impl easy_storage::Storeable for Config {}

#[derive(Debug, Getters, Serialize, Deserialize)]
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

    pub fn get_model<T: AsRef<str>>(&self, alias: T) -> Option<Model> {
        let res = match &self.model_alias {
            Some(v) => Some(v.get(&alias.as_ref().to_string())),
            None => None,
        }
        .flatten();
        match res {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

#[derive(Debug, Getters, Serialize, Deserialize, Clone)]
pub struct Model {
    provider: String,
    model: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    base_url: Option<(String, u16)>,
}

impl Model {
    pub fn new<T: AsRef<str>>(
        provider: T,
        model: T,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        base_url: Option<(String, u16)>,
    ) -> Self {
        Self {
            provider: provider.as_ref().to_string(),
            model: model.as_ref().to_string(),
            temperature,
            max_tokens,
            base_url,
        }
    }
}
