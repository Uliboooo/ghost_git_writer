use crate::{Error, config::Model, llm};

pub fn custom_prompt<T: AsRef<str>>(
    pmt: T,
    model: Model,
    api_key: Option<T>,
    extra: Option<T>,
) -> Result<String, Error> {
    let ext = match extra {
        Some(v) => format!(" # Additional Instructions: {}", v.as_ref()),
        None => "".to_string(),
    };
    let pmt = format!("{}. {}", pmt.as_ref(), ext);
    llm::call_llm(
        pmt,
        model.provider,
        model.model,
        api_key.map(|f| f.as_ref().to_string()),
        None,
        None,
    )
    .map_err(Error::Llm)
}
