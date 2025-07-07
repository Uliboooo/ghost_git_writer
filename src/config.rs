mod storage;

use std::path::Path;
use serde::{Deserialize, Serialize};
use storage::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    llm: Llm,
}

impl<P: AsRef<Path>> Storage<P> for Config {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Llm {
    models: Vec<Option<ModelInfo>>,
    default_model: ModelInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    provider: String,
    api_key: String,
    base_url: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    model: String,
    api_key: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

