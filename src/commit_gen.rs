use std::fmt::Display;

use crate::llm;

#[derive(Debug)]
pub enum Error {
    Llm(llm::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Llm(e) => write!(f, "llm error: {}", e),
        }
    }
}

impl From<llm::Error> for Error {
    fn from(value: llm::Error) -> Self {
        Self::Llm(value)
    }
}

const GEN_MSG_PMT: &str = "You are an assistant that writes Git commit messages.\
When code changes include modifications to documentation files (e.g., README.md, docs/), ignore those changes and generate the commit message based solely on source code changes.\
Given a description of code changes, output only a single-line commit message in Conventional Commits format (e.g., \"feat:\", \"fix:\", \"docs:\", etc.).\
Do not include any extra text, code blocks, or formatting. Only output the commit message.\
Diff Changes:";

pub async fn gen_commit_msg<T: AsRef<str>>(
    diff: T,
    model: llm::LlmReqInfo,
    lang: Option<&T>,
    extra: Option<&T>,
) -> Result<String, Error> {
    let diff = diff.as_ref();
    let lang = lang
        .map(|f| f.as_ref().to_string())
        .unwrap_or("english".to_string());
    let extra = format!(
        " # Additional Instructions: {}",
        extra.map_or("".to_string(), |f| f.as_ref().to_string())
    );

    let prompt = format!("Please in {lang}.\n{GEN_MSG_PMT}{diff}.\n{extra}");

    Ok(llm::call_llm(model, prompt).await?)
}
