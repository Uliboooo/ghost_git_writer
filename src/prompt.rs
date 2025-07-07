use crate::{llm, Error};

pub fn cmt_msg(diff: String, prompt: Option<String>) -> Result<String, Error> {
    let prmt = prompt.unwrap_or(format!("write a git commit message for this diff. \ndiff: {}", diff));
    llm::call_llms(prmt, service, api_key, temperature, max_tokens)
}
