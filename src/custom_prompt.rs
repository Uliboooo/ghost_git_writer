use crate::{Error, config::Model, llm};

pub fn custom_prpmt<T: AsRef<str>>(
    pmt: T,
    model: Model,
    api_key: Option<T>,
) -> Result<String, Error> {
    llm::call_llm(
        pmt.as_ref().to_string(),
        model.provider,
        model.model,
        api_key.map(|f| f.as_ref().to_string()),
        None,
        None,
    )
    .map_err(Error::Llm)
}
