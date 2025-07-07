mod storage;

use std::path::Path;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use storage::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    llm: Llm,
    custom_prompt: CustomPrompt,
}

impl<P: AsRef<Path>> Storage<P> for Config {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Llm {
    models: Vec<Option<ModelInfo>>,
    default_model: ModelInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  CustomPrompt {
    commit_message: String,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ModelInfo {
    provider: Provider,
    model: String,
    api_key: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    base_url: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Provider {
    Ollama,
    Anthropic,
    Deepseek,
    Gemini,
    OpenAI,
}
