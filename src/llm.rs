use crate::cli_helper::Spinner;
use crate::config;
use derive_getters::Getters;
use llm_api_rs::{
    self, LlmProvider,
    core::{ChatCompletionRequest, ChatMessage},
    providers::{Anthropic, DeepSeek, Gemini, OpenAI},
};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use reqwest::header::HeaderMap;
use serde_json::json;
use std::fmt::Display;

const ANTHROPIC_DEFAULT_MAX_TOKENS: u32 = 1000;

#[derive(Debug)]
pub enum Error {
    NotSuppoeredProvider,
    FailedGetAPIKey,
    FailedGetBaseURL,
    ChatCompletion(String),
    // CliHelper(String),
    OllamaE(ollama_rs::error::OllamaError),
    Conf(config::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotSuppoeredProvider => write!(f, "not supported provider"),
            Error::FailedGetAPIKey => write!(f, "failed to get api key"),
            Error::FailedGetBaseURL => write!(f, "failed to get base url"),
            Error::ChatCompletion(e) => write!(f, "chat completion error: {}", e),
            // Error::CliHelper(e) => write!(f, "cli helper error: {}", e),
            Error::OllamaE(e) => write!(f, "ollama error: {}", e),
            Error::Conf(error) => write!(f, "config error {error}"),
        }
    }
}

// impl From<cli_helper::Error> for Error {
//     fn from(e: cli_helper::Error) -> Self {
//         match e {
//             cli_helper::Error::Io(io_err) => Error::CliHelper(io_err.to_string()),
//         }
//     }
// }
impl From<ollama_rs::error::OllamaError> for Error {
    fn from(value: ollama_rs::error::OllamaError) -> Self {
        Self::OllamaE(value)
    }
}

impl From<config::Error> for Error {
    fn from(value: config::Error) -> Self {
        Self::Conf(value)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Provider {
    Ollama,
    OpenAI,
    Gemini,
    Anthropic,
    DeepSeek,
}

impl TryFrom<&str> for Provider {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "ollama" => Ok(Self::Ollama),
            "openai" => Ok(Self::OpenAI),
            "gemini" => Ok(Self::Gemini),
            "anthropic" => Ok(Self::Anthropic),
            "deepseek" => Ok(Self::DeepSeek),
            _ => Err(Error::NotSuppoeredProvider),
        }
    }
}

impl Provider {
    fn is<T: AsRef<str>>(&self, prov: T) -> Result<bool, Error> {
        let p = Provider::try_from(prov.as_ref())?;
        Ok(self == &p)
    }
}

#[derive(Debug, Getters, Clone)]
pub struct LlmReqInfo {
    provider: Provider,
    model: String,
    api_key: Option<String>,
    temp: Option<f32>,
    max_tokens: Option<u32>,
    base_url: Option<(String, u16)>,
}

impl LlmReqInfo {
    pub fn resolve_api_key(&self) -> Result<String, Error> {
        Ok(match self.api_key.clone() {
            Some(key) => key,
            None => {
                if self.provider().is("ollama")? {
                    return Err(Error::FailedGetAPIKey);
                } else {
                    String::new()
                }
            }
        })
    }

    pub fn new(
        provider: Provider,
        model: String,
        api_key: Option<String>,
        temp: Option<f32>,
        max_tokens: Option<u32>,
        base_url: Option<(String, u16)>,
    ) -> Self {
        Self {
            provider,
            model,
            api_key,
            temp,
            max_tokens,
            base_url,
        }
    }

    pub fn new_with_api(model: config::Model, api_key: Option<String>) -> Result<Self, Error> {
        let prov = Provider::try_from(model.provider().as_str())?;
        let base = model.resolve_base_url()?;
        Ok(Self::new(
            prov,
            model.model().clone(),
            api_key,
            *model.temperature(),
            *model.max_tokens(),
            base,
        ))
    }
}

/// Build the raw HTTP URL, headers, and JSON body for a provider's chat completion endpoint.
fn build_raw_request(
    llm_info: &LlmReqInfo,
    api: &str,
    prompt: &str,
) -> Result<(String, HeaderMap, serde_json::Value), Error> {
    let mut headers = HeaderMap::new();
    let url;
    let body;

    match llm_info.provider() {
        Provider::OpenAI => {
            url = "https://api.openai.com/v1/chat/completions".to_string();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {api}")
                    .parse()
                    .map_err(|_| Error::FailedGetAPIKey)?,
            );
            body = json!({
                "model": llm_info.model(),
                "messages": [{"role": "user", "content": prompt}],
                "temperature": llm_info.temp(),
                "max_tokens": llm_info.max_tokens(),
            });
        }
        Provider::Gemini => {
            url = format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
                llm_info.model(),
                api
            );
            body = json!({
                "contents": [{"role": "user", "parts": [{"text": prompt}]}],
                "generation_config": {
                    "temperature": llm_info.temp(),
                    "maxOutputTokens": llm_info.max_tokens(),
                }
            });
        }
        Provider::Anthropic => {
            url = "https://api.anthropic.com/v1/messages".to_string();
            headers.insert(
                reqwest::header::HeaderName::from_static("x-api-key"),
                api.parse().map_err(|_| Error::FailedGetAPIKey)?,
            );
            headers.insert(
                reqwest::header::HeaderName::from_static("anthropic-version"),
                "2023-06-01".parse().map_err(|_| Error::ChatCompletion("header".to_string()))?,
            );
            body = json!({
                "model": llm_info.model(),
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": llm_info.max_tokens().unwrap_or(ANTHROPIC_DEFAULT_MAX_TOKENS),
                "temperature": llm_info.temp(),
            });
        }
        Provider::DeepSeek => {
            url = "https://api.deepseek.com/v1/chat/completions".to_string();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {api}")
                    .parse()
                    .map_err(|_| Error::FailedGetAPIKey)?,
            );
            body = json!({
                "model": llm_info.model(),
                "messages": [{"role": "user", "content": prompt}],
                "temperature": llm_info.temp(),
                "max_tokens": llm_info.max_tokens(),
            });
        }
        Provider::Ollama => return Err(Error::NotSuppoeredProvider),
    }

    Ok((url, headers, body))
}

/// Extract the assistant message content from a raw JSON response body.
fn extract_content(provider: &Provider, raw: &str) -> Result<String, Error> {
    let v: serde_json::Value = serde_json::from_str(raw)
        .map_err(|e| Error::ChatCompletion(format!("Deserialization error: {e}")))?;

    let content = match provider {
        Provider::OpenAI | Provider::DeepSeek => {
            v["choices"][0]["message"]["content"].as_str().map(str::to_string)
        }
        Provider::Gemini => v["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(str::to_string),
        Provider::Anthropic => v["content"][0]["text"].as_str().map(str::to_string),
        Provider::Ollama => unreachable!("Ollama uses a separate code path and should never reach here"),
    };

    content.ok_or_else(|| {
        Error::ChatCompletion(format!(
            "Could not extract content from response: {raw}"
        ))
    })
}

pub async fn call_llm<T: AsRef<str>>(
    llm_info: LlmReqInfo,
    prompt: T,
    debug: bool,
) -> Result<String, Error> {
    if debug {
        eprintln!("[DEBUG] === LLM Request ===");
        eprintln!("[DEBUG] Provider: {:?}", llm_info.provider());
        eprintln!("[DEBUG] Model: {}", llm_info.model());
        eprintln!("[DEBUG] Temperature: {:?}", llm_info.temp());
        eprintln!("[DEBUG] Max tokens: {:?}", llm_info.max_tokens());
        eprintln!("[DEBUG] Prompt:\n{}", prompt.as_ref());
        eprintln!("[DEBUG] ==================");
    }

    let spinner = Spinner::new("Calling LLM...");
    let result = async {
        // llm_api_rs isn't support ollama
        if llm_info.provider() == &Provider::Ollama {
            let base_url = llm_info.clone().base_url.ok_or(Error::FailedGetBaseURL)?;
            let o_res = Ollama::new(base_url.0, base_url.1);
            let res = o_res
                .generate(GenerationRequest::new(
                    llm_info.clone().model,
                    prompt.as_ref(),
                ))
                .await;
            match res {
                Ok(v) => Ok(v.response.to_string()),
                Err(e) => Err(Error::OllamaE(e)),
            }
        } else if debug {
            // In debug mode: use reqwest directly to capture and print the raw JSON response.
            let api = llm_info.resolve_api_key()?;
            let (url, headers, body) = build_raw_request(&llm_info, &api, prompt.as_ref())?;

            let client = reqwest::Client::new();
            let response = client
                .post(&url)
                .headers(headers)
                .json(&body)
                .send()
                .await
                .map_err(|e| Error::ChatCompletion(e.to_string()))?;

            let raw_body = response
                .text()
                .await
                .map_err(|e| Error::ChatCompletion(e.to_string()))?;

            eprintln!("[DEBUG] === Raw JSON Response ===");
            eprintln!("{raw_body}");
            eprintln!("[DEBUG] ========================");

            extract_content(llm_info.provider(), &raw_body)
        } else {
            let api = llm_info.resolve_api_key()?;
            let client: Box<dyn LlmProvider + Send> = match llm_info.provider {
                Provider::OpenAI => Box::new(OpenAI::new(api)),
                Provider::Gemini => Box::new(Gemini::new(api)),
                Provider::Anthropic => Box::new(Anthropic::new(api)),
                Provider::DeepSeek => Box::new(DeepSeek::new(api)),
                _ => return Err(Error::NotSuppoeredProvider),
            };

            let request = ChatCompletionRequest {
                model: llm_info.model().clone(),
                messages: vec![ChatMessage {
                    role: "user".to_string(),
                    content: prompt.as_ref().to_string(),
                }],
                temperature: *llm_info.temp(),
                max_tokens: *llm_info.max_tokens(),
            };

            match client.chat_completion(request).await {
                Ok(response) => {
                    if let Some(choice) = response.choices.first() {
                        Ok(choice.message.content.clone())
                    } else {
                        Ok(String::new())
                    }
                }
                Err(e) => Err(Error::ChatCompletion(e.to_string())),
            }
        }
    }
    .await;
    spinner.stop("LLM call finished.");
    result
}
