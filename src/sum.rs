use crate::{llm, Error, Model};

const DEFAULT_PROMT: &str = "Read the following diff and summarize the changes in plain English.
List the key modifications, what was added, removed, or modified, and briefly explain their purpose or impact if possible.
--- diff here ---";

pub fn summarize_diff<T: AsRef<str>>(
    diff: T,
    model: Model,
    api_key: Option<T>,
) -> Result<String, Error> {
    let pmt = format!("{DEFAULT_PROMT} {}", diff.as_ref());
    llm::call_llm(
        pmt.to_string(),
        model.provider,
        model.model_name,
        api_key.map(|f| f.as_ref().to_string()),
        None,
        None,
    )
    .map_err(Error::Llm)
}
