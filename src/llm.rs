use llm_api_rs::{
    Anthropic, ChatCompletionRequest, ChatMessage, Gemini, LlmApiError, LlmProvider, OpenAI,
};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use tokio::runtime::Runtime;

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

pub fn call_llm<T: AsRef<str>>(
    pmt: T,
    provider: T,
    model: T,
    api_key: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String, LlmError> {
    let model = model.as_ref().to_string();
    let pmt = pmt.as_ref().to_string();
    let rt = Runtime::new().unwrap();

    let api_key = match api_key {
        Some(v) => v,
        None => {
            if provider.as_ref().to_lowercase() != "ollama" {
                return Err(LlmError::NotFoundAPIKey);
            } else {
                String::new()
            }
        }
    };
    match provider.as_ref().to_lowercase().as_str() {
        "ollama" => rt.block_on(ollama(pmt, model)),
        "anthropic" => rt.block_on(anthropic(api_key, model, pmt, temperature, max_tokens)),
        "deepseek" => rt.block_on(deep_seek(api_key, model, pmt, temperature, max_tokens)),
        "gemini" => rt.block_on(gemini(api_key, model, pmt, temperature, max_tokens)),
        "openai" => rt.block_on(openai(api_key, model, pmt, temperature, max_tokens)),
        _ => Err(LlmError::UndefinedProvider),
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

#[cfg(test)]
mod tests {
    use crate::llm::{call_llm, gemini};
    use std::env;
    use tokio::runtime::Runtime;

    #[test]
    fn call_test() {
        let res = call_llm(
            "hello",
            "gemini",
            "gemini-2.0-flash",
            Some(env::var("GEMINI_API_KEY").unwrap().to_string()),
            None,
            None,
        );

        println!("res: {res:?}");
    }

    #[test]
    /// this is require GEMINI_API_KEY in your env.
    fn test_gemini() {
        let api = env::var("GEMINI_API_KEY");
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(gemini(
            api.unwrap(),
            "gemini-2.0-flash".to_string(),
            "hello".to_string(),
            None,
            None,
        ));

        println!("\n\nresult\n{result:?}");
    }
}
