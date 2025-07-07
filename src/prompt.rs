use crate::{Error, config::ModelInfo, llm};

/// if prompt is None, use default prompt.
pub fn cmt_msg(
    diff: String,
    prompt: Option<String>,
    model_info: ModelInfo,
) -> Result<String, Error> {
    let prmt = prompt.unwrap_or(format!(
        "write a git commit message for this diff. \ndiff: {}",
        diff
    ));
    llm::call_llms(prmt, model_info).map_err(Error::llm)
}
