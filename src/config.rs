use crate::storage::Storage;
use std::{collections::HashMap, path::Path};
use serde::{Deserialize, Serialize};

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
    pub fn get_default_model(&self) -> Option<&Model> {
        self.model_alias.get(self.default_alias.as_ref()?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Model {
    provider: String,
    model: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

