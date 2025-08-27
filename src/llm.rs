use std::fmt::Display;

use derive_getters::Getters;
use llm_api_rs::{
    self, LlmProvider,
    core::{ChatCompletionRequest, ChatMessage},
    providers::{Anthropic, DeepSeek, Gemini, OpenAI},
};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

use crate::cli_helper::Spinner;
use crate::{cli_helper, config};

#[derive(Debug)]
pub enum Error {
    NotSuppoeredProvider,
    FailedGetAPIKey,
    FailedGetBaseURL,
    ChatCompletion(String),
    CliHelper(String),
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
            Error::CliHelper(e) => write!(f, "cli helper error: {}", e),
            Error::OllamaE(e) => write!(f, "ollama error: {}", e),
            Error::Conf(error) => write!(f, "config error {error}"),
        }
    }
}

impl From<cli_helper::Error> for Error {
    fn from(e: cli_helper::Error) -> Self {
        match e {
            cli_helper::Error::Io(io_err) => Error::CliHelper(io_err.to_string()),
        }
    }
}
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
            "authropic" => Ok(Self::Anthropic),
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

    // pub fn is_ollama(&self) -> bool {
    //     self == &Provider::Ollama
    // }
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

pub async fn call_llm<T: AsRef<str>>(llm_info: LlmReqInfo, prompt: T) -> Result<String, Error> {
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
