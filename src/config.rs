use crate::{Cli, Error, storage::Storage};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    prompt: Option<Prompt>,
    llm: Option<Llm>,
}

impl<P: AsRef<Path>> Storage<P> for Config {}

#[derive(Debug, Serialize, Deserialize)]
struct Prompt {
    custom_prompt: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Llm {
    default_alias: Option<String>,
    model_alias: HashMap<String, Model>,
}

impl Llm {
    fn get_default_model(&self) -> Option<Model> {
        self.model_alias.get(self.default_alias.as_ref()?).cloned()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub provider: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

impl Model {
    pub fn new<T: AsRef<str>>(
        pro: T,
        model: T,
        temp: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Self {
        Self {
            provider: pro.as_ref().to_string(),
            model: model.as_ref().to_string(),
            temperature: temp,
            max_tokens,
        }
    }
}

impl TryFrom<Cli> for Model {
    type Error = Error;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        match value.model {
            Some(v) => match v.split_once('/') {
                Some(v) => Ok((v.0.to_string(), v.1.to_string())),
                None => Err(Error::InvalidModelFormat(v)),
            },
            None => Err(Error::FailedParseCli),
        }
        .map(|f| Model::new(f.0, f.1, None, None))
    }
}
