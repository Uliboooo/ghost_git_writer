use std::fs::OpenOptions;

use crate::{Error, cli_helper};
use llm_api_rs::{
    Anthropic, ChatCompletionRequest, ChatMessage, Gemini, LlmApiError, LlmProvider, OpenAI,
};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

#[derive(Debug)]
pub enum LlmError {
    Ollama(ollama_rs::error::OllamaError),
    Other(LlmApiError),
    UndefinedProvider,
    NotFoundAPIKey,
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmError::Ollama(e) => write!(f, "Ollama error: {e}"),
            LlmError::Other(e) => write!(f, "Other error: {e}"),
            LlmError::UndefinedProvider => write!(f, "Undefined LLM provider"),
            LlmError::NotFoundAPIKey => write!(f, "API key not found"),
        }
    }
}

#[derive(Debug, derive_getters::Getters)]
pub struct ReqConfig {
    api_key: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    base_url: Option<(String, u16)>,
}

impl ReqConfig {
    pub fn new(
        api_key: Option<String>,
        temp: Option<f32>,
        max_t: Option<u32>,
        base_url: Option<(String, u16)>,
    ) -> Self {
        Self {
            api_key,
            temperature: temp,
            max_tokens: max_t,
            base_url,
        }
    }
}

pub fn call_llm<T: AsRef<str>>(
    pmt: T,
    provider: T,
    model: T,
    req_config: ReqConfig,
    // api_key: Option<String>,
    // temperature: Option<f32>,
    // max_tokens: Option<u32>,
    // base_url: Option<(String, u16)>,
) -> Result<String, Error> {
    let model = model.as_ref().to_string();
    let pmt = pmt.as_ref().to_string();

    // let api_key = match req_config.api_key() {
    //     Some(v) => v,
    //     None => {
    //         if provider.as_ref().to_lowercase() != "ollama" {
    //             return Err(Error::Llm(LlmError::NotFoundAPIKey));
    //         } else {
    //             String::new()
    //         }
    //     }
    // };

    let api_key = req_config
        .api_key()
        .clone()
        .or_else(|| {
            if provider.as_ref().eq_ignore_ascii_case("ollama") {
                Some(String::new())
            } else {
                None
            }
        })
        .ok_or(Error::Llm(LlmError::NotFoundAPIKey))?;

    let temp = *req_config.temperature();
    let m_token = *req_config.max_tokens();
    let base_url = req_config
        .base_url()
        .clone()
        .unwrap_or(("http://localhost".to_string(), 11434));

    match provider.as_ref().to_lowercase().as_str() {
        "authropic" => match cli_helper::run_with_spinner(move || {
            anthropic(api_key, model, pmt, temp, m_token)
        }) {
            Ok(v) => match v {
                Ok(vv) => Ok(vv),
                Err(e) => Err(Error::Llm(e)),
            },
            Err(e) => Err(e),
        },
        "deepseek" => match cli_helper::run_with_spinner(move || {
            deep_seek(api_key, model, pmt, temp, m_token)
        }) {
            Ok(v) => match v {
                Ok(vv) => Ok(vv),
                Err(e) => Err(Error::Llm(e)),
            },
            Err(e) => Err(e),
        },
        "gemini" => {
            match cli_helper::run_with_spinner(move || gemini(api_key, model, pmt, temp, m_token)) {
                Ok(v) => match v {
                    Ok(vv) => Ok(vv),
                    Err(e) => Err(Error::Llm(e)),
                },
                Err(e) => Err(e),
            }
        }
        "openai" => {
            match cli_helper::run_with_spinner(move || openai(api_key, model, pmt, temp, m_token)) {
                Ok(v) => match v {
                    Ok(vv) => Ok(vv),
                    Err(e) => Err(Error::Llm(e)),
                },
                Err(e) => Err(e),
            }
        }
        "ollama" => match cli_helper::run_with_spinner(move || ollama(pmt, model, base_url)) {
            Ok(v) => match v {
                Ok(vv) => Ok(vv),
                Err(e) => Err(Error::Llm(e)),
            },
            Err(e) => Err(e),
        },
        _ => Err(Error::Llm(LlmError::UndefinedProvider)),
    }
}

async fn ollama(pmt: String, model: String, base_url: (String, u16)) -> Result<String, LlmError> {
    // let ollama = Ollama::default();
    let ollama = Ollama::new(base_url.0, base_url.1);

    let res = ollama.generate(GenerationRequest::new(model, pmt)).await;
    match res {
        Ok(v) => Ok(v.response.to_string()),
        Err(e) => Err(LlmError::Ollama(e)),
    }
}

async fn anthropic<T: AsRef<str>>(
    api_key: T,
    model: String,
    pmt: String,
    tmp: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String, LlmError> {
    let client = Anthropic::new(api_key.as_ref().to_string());

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

async fn gemini<T: AsRef<str>>(
    api_key: T,
    model: String,
    pmt: String,
    tmp: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String, LlmError> {
    let client = Gemini::new(api_key.as_ref().to_string());

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

async fn openai<T: AsRef<str>>(
    api_key: T,
    model: String,
    pmt: String,
    tmp: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String, LlmError> {
    let client = OpenAI::new(api_key.as_ref().to_string());
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

async fn deep_seek<T: AsRef<str>>(
    api_key: T,
    model: String,
    pmt: String,
    tmp: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String, LlmError> {
    let client = OpenAI::new(api_key.as_ref().to_string());
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

// #[cfg(test)]
// mod tests {
//     use crate::llm::{call_llm, gemini};
//     use std::env;
//     use tokio::runtime::Runtime;

//     #[test]
//     fn call_test() {
//         let res = call_llm(
//             "hello",
//             "gemini",
//             "gemini-2.0-flash",
//             Some(env::var("GEMINI_API_KEY").unwrap().to_string()),
//             None,
//             None,
//         );

//         println!("res: {res:?}");
//     }

//     #[test]
//     /// this is require GEMINI_API_KEY in your env.
//     fn test_gemini() {
//         let api = env::var("GEMINI_API_KEY");
//         let rt = Runtime::new().unwrap();
//         let result = rt.block_on(gemini(
//             api.unwrap(),
//             "gemini-2.0-flash".to_string(),
//             "hello".to_string(),
//             None,
//             None,
//         ));

//         println!("\n\nresult\n{result:?}");
//     }
// }
