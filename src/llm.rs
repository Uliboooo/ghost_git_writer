use llm_api_rs::{
    Anthropic, ChatCompletionRequest, ChatMessage, Gemini, LlmApiError, LlmProvider, OpenAI,
};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use tokio::runtime::Runtime;

use crate::config::{ModelConfig, ModelInfo, Provider};

pub enum ServiceModel {
    Ollama(String),
    Anthropic(String),
    Deepseek(String),
    Gemini(String),
    OpenAI(String),
}

impl ServiceModel {
    pub fn new<T: AsRef<str>>(service: T, model_name: T) -> Self {
        match service.as_ref() {
            "ollama" => Self::Ollama(model_name.as_ref().to_string()),
            "anthropic" => Self::Anthropic(model_name.as_ref().to_string()),
            "deepseek" => Self::Deepseek(model_name.as_ref().to_string()),
            "gemini" => Self::Gemini(model_name.as_ref().to_string()),
            "openai" => Self::OpenAI(model_name.as_ref().to_string()),
            _ => Self::Ollama(model_name.as_ref().to_string()),
        }
    }
}

#[derive(Debug)]
pub enum LlmError {
    Ollama(ollama_rs::error::OllamaError),
    Other(LlmApiError),
}

pub fn call_llms<T: AsRef<str>>(pmt: T, model_info: ModelInfo) -> Result<String, LlmError> {
    let pmt = pmt.as_ref().to_string();

    let model = model_info.1;
    let api_key = model_info.2.api_key().unwrap().clone();
    let temperature= *model_info.2.temperature();
    let max_tokens = *model_info.2.max_tokens();

    // let api_key = model_info.api_key().unwrap().as_str().to_string();
    // // let model = model_info.model().to_string();
    // let temperature = model_info.temperature().as_ref().map(|f| *f);
    // let max_tokens = model_info.max_tokens().as_ref().map(|f| *f);

    let rt = Runtime::new().unwrap();
    match model_info.0 {
        Provider::Ollama => rt.block_on(ollama(pmt, model)),
        Provider::Anthropic => rt.block_on(anthopic(api_key, model, pmt, temperature, max_tokens)),
        Provider::Deepseek => rt.block_on(deep_seek(api_key, model, pmt, temperature, max_tokens)),
        Provider::Gemini => rt.block_on(gemini(api_key, model, pmt, temperature, max_tokens)),
        Provider::OpenAI => rt.block_on(openai(api_key, model, pmt, temperature, max_tokens)),
    }
}

async fn ollama(pmt: String, model: String) -> Result<String, LlmError> {
    let ollama = Ollama::default();

    let res = ollama.generate(GenerationRequest::new(model, pmt)).await;
    match res {
        Ok(v) => Ok(v.response.to_string()),
        Err(e) => Err(LlmError::Ollama(e)),
    }
}

async fn anthopic(
    api_key: String,
    model: String,
    pmt: String,
    tmp: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String, LlmError> {
    let client = Anthropic::new(api_key.to_string());

    let req = ChatCompletionRequest {
        model,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: pmt,
        }],
        temperature: tmp,
        max_tokens,
    };
    client
        .chat_completion(req)
        .await
        .map(|f| {
            f.choices
                .iter()
                .map(|f| f.message.content.to_string())
                .collect::<String>()
        })
        .map_err(LlmError::Other)
}

async fn gemini(
    api_key: String,
    model: String,
    pmt: String,
    tmp: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String, LlmError> {
    let client = Gemini::new(api_key.to_string());
    let req = ChatCompletionRequest {
        model,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: pmt,
        }],
        temperature: tmp,
        max_tokens,
    };
    client
        .chat_completion(req)
        .await
        .map(|f| {
            f.choices
                .iter()
                .map(|f| f.message.content.to_string())
                .collect::<String>()
        })
        .map_err(LlmError::Other)
}

async fn openai(
    api_key: String,
    model: String,
    pmt: String,
    tmp: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String, LlmError> {
    let client = OpenAI::new(api_key.to_string());
    let req = ChatCompletionRequest {
        model,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: pmt,
        }],
        temperature: tmp,
        max_tokens,
    };
    client
        .chat_completion(req)
        .await
        .map(|f| {
            f.choices
                .iter()
                .map(|f| f.message.content.to_string())
                .collect::<String>()
        })
        .map_err(LlmError::Other)
}

async fn deep_seek(
    api_key: String,
    model: String,
    pmt: String,
    tmp: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String, LlmError> {
    let client = OpenAI::new(api_key.to_string());
    let req = ChatCompletionRequest {
        model,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: pmt,
        }],
        temperature: tmp,
        max_tokens,
    };
    client
        .chat_completion(req)
        .await
        .map(|f| {
            f.choices
                .iter()
                .map(|f| f.message.content.to_string())
                .collect::<String>()
        })
        .map_err(LlmError::Other)
}
