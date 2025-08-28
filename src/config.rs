use std::fmt::Display;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use url::Url;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    // PortIsNotNumber,
    Url(url::ParseError),
    NotFoundPort,
    NotFoundHost,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Error::PortIsNotNumber => write!(f, "failed parse port to number"),
            Error::Url(parse_error) => write!(f, "failed parse url {parse_error}"),
            Error::NotFoundPort => write!(f, "not found port in base_url"),
            Error::NotFoundHost => write!(f, "not found host (exmaple.com)"),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value)
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub struct Config {
    llms: Option<Llm>,
}

impl easy_storage::Storeable for Config {}

#[derive(Debug, Getters, Serialize, Deserialize, Clone)]
pub struct Llm {
    default_model: Option<Model>,
    models: Option<HashMap<String, Model>>,
    ollama: Option<OllamaConfig>,
}

impl Llm {
    pub fn get_default(&self) -> Option<Model> {
        self.default_model.clone()
    }

    pub fn get_model<T: AsRef<str>>(&self, name: T) -> Option<Model> {
        let res = match &self.models {
            Some(v) => Some(v.get(&name.as_ref().to_string())),
            None => None,
        }
        .flatten();
        res.cloned()
    }
}

#[derive(Debug, Getters, Serialize, Deserialize, Clone)]
pub struct Model {
    provider: String,
    model: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    base_url: Option<String>,
}

impl Model {
    pub fn new<T: AsRef<str>>(
        provider: T,
        model: T,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        base_url: Option<String>,
    ) -> Self {
        Self {
            provider: provider.as_ref().to_string(),
            model: model.as_ref().to_string(),
            temperature,
            max_tokens,
            base_url,
        }
    }

    pub fn resolve_base_url(&self) -> Result<Option<(String, u16)>, Error> {
        // let url = match &self.base_url {
        //     Some(v) => v,
        //     None => return Ok(None),
        // };
        // let parsed_url = Url::parse(url)?;
        // let port = parsed_url.port().unwrap();
        // let sh = parsed_url.scheme();
        // let host = parsed_url.host_str().unwrap();
        // let url = format!("{sh}://{host}{}", parsed_url.path());
        // Ok(Some((url, port)))
        self.base_url.as_ref().map(parse_port).transpose()
    }
}

fn parse_port<T: AsRef<str>>(url: T) -> Result<(String, u16), Error>{
        let parsed_url = Url::parse(url.as_ref())?;
        let port = parsed_url.port().ok_or(Error::NotFoundPort)?;
        let sh = parsed_url.scheme();
        let host = parsed_url.host_str().ok_or(Error::NotFoundHost)?;
        let url = format!("{sh}://{host}{}", parsed_url.path());
        Ok((url, port))
}

#[derive(Debug, Getters, Serialize, Deserialize, Clone)]
pub struct OllamaConfig {
    base_url: Option<String>,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: Some(String::from("http://localhost:11434")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use easy_storage::Storeable;

    use crate::config::{self, parse_port, Error, Model};

    #[test]
    fn save_config() {
        let def = Model::new("gemini", "gemini-2.0-flash", None, None, None);
        let mut models = HashMap::new();
        models.insert(
            "ge".to_string(),
            Model::new("gemini", "gemini-2.5-flash", None, None, None),
        );

        let config = config::Config {
            llms: Some(config::Llm {
                default_model: Some(def),
                models: Some(models),
                ollama: None,
            }),
        };
        let c = std::env::current_dir()
            .unwrap()
            .join("config_template")
            .with_extension("toml");
        config.save_by_extension(c, true).unwrap();
    }

    #[test]
    fn parsed_url_test() {
        let test_urls = 
            [("http://localhost:11434", Ok(("http://localhost/".to_string(), 11434))),
            ("http://foo.com:11434/bar", Ok(("http://foo.com/bar".to_string(), 11434))),
            ("foo.com:11434/bar", Err(Error::NotFoundPort)),
            ("foo.com/bar", Err(Error::Url(url::ParseError::RelativeUrlWithoutBase)))
            ];

        for u in test_urls {
            let parsed = parse_port(u.0);
            assert_eq!(parsed, u.1);
        }
    }
}
