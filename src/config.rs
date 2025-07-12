pub mod storage;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
pub use storage::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub llm: Llm,
    pub custom_prompt: CustomPrompt,
}

impl<P: AsRef<Path>> Storage<P> for Config {}

pub type ModelName = String;
pub type ModelInfo = (Provider, ModelName, ModelConfig);

#[derive(Debug, Serialize, Deserialize)]
pub struct Llm {
    pub llm_models: HashMap<Provider, HashMap<String, ModelConfig>>,
    ///                           👇model name
    pub default_model: (Provider, String, ModelConfig),
}

impl Llm {
    fn get_config(&self, provider: Provider, model: &str) -> Option<&ModelConfig> {
        self.llm_models.get(&provider).map(|f| {
            f.get(model)
        }).flatten()
    }
}

#[derive(Debug, Serialize, Deserialize, Getters, PartialEq)]
pub struct ModelConfig {
    // model: String,
    api_key: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    base_url: Option<String>,
}

impl ModelConfig {
    pub fn new(
        // model: String,
        api_key: Option<String>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        base_url: Option<String>,
    ) -> Self {
        ModelConfig {
            // model,
            api_key,
            temperature,
            max_tokens,
            base_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Provider {
    Ollama,
    Anthropic,
    Deepseek,
    Gemini,
    OpenAI,
}

impl<T: AsRef<str>> From<T> for Provider {
    fn from(value: T) -> Self {
        match value.as_ref().to_lowercase().as_str() {
            "ollama" => Provider::Ollama,
            "anthropic" => Provider::Anthropic,
            "deepseek" => Provider::Deepseek,
            "gemini" => Provider::Gemini,
            _ => Provider::OpenAI,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomPrompt {
    commit_message: String,
}

