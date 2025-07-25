use crate::{Error, Model, llm};

const DEFAULT_PROMPT: &str = "Read the following diff and summarize the changes in plain English.
List the key modifications, what was added, removed, or modified, and briefly explain their purpose or impact if possible.
diff:";

pub fn summarize_diff<T: AsRef<str>, U: AsRef<str>>(
    diff: T,
    model: Model,
    api_key: Option<T>,
    lang: Option<U>,
    extra: Option<U>,
) -> Result<String, Error> {
    let lang = lang
        .map(|f| f.as_ref().to_string())
        .unwrap_or("english".to_string());

    let extra = extra
        .map(|f| format!("# Additional Instructions:{}", f.as_ref()))
        .unwrap_or("".to_string());

    let pmt = format!(
        "Generate the README.md in {lang}. {DEFAULT_PROMPT} {}.{}",
        diff.as_ref(),
        extra,
    );

    llm::call_llm(
        pmt.to_string(),
        model.provider,
        model.model,
        api_key.map(|f| f.as_ref().to_string()),
        None,
        None,
    )
}
