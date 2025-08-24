use std::fmt::Display;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    NotFoundAlias,
    InvalidPortFormat,
    InvalidBaseUrlFormat,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFoundAlias => write!(f, "not found alias"),
            Error::InvalidPortFormat => todo!(),
            Error::InvalidBaseUrlFormat => todo!(),
        }
    }
}

#[derive(Debug, Getters, Serialize, Deserialize)]
pub struct Config {
    llms: Option<Llm>,
}

impl Config {
    pub fn exist_alias<T: AsRef<str>>(&self, alias: T) -> bool {
        match &self.llms {
            Some(v) => match &v.models {
                Some(vv) => vv.contains_key(&alias.as_ref().to_string()),
                None => false,
            },
            None => false,
        }
    }

    pub fn get_alias<T: AsRef<str>>(&self, alias: T) -> Option<Model> {
        self.llms.as_ref().and_then(|v| v.get_model(alias))
    }
}

impl easy_storage::Storeable for Config {}

#[derive(Debug, Getters, Serialize, Deserialize, Clone)]
pub struct Llm {
    default_model: Option<Model>,
    models: Option<HashMap<String, Model>>,
    ollama: Option<OllamaConfig>,
}

impl Llm {
    /// check to exist alias
    pub fn exist_alias<T: AsRef<str>>(&self, alias: T) -> bool {
        match self.models() {
            Some(v) => v.contains_key(&alias.as_ref().to_string()),
            None => false,
        }
    }

    pub fn exist_default_alias(&self) -> bool {
        self.default_model.is_some()
    }

    pub fn get_default(&self) -> Option<Model> {
        self.default_model.clone()
    }

    pub fn get_model<T: AsRef<str>>(&self, name: T) -> Option<Model> {
        let res = match &self.models {
            Some(v) => Some(v.get(&name.as_ref().to_string())),
            None => None,
        }
        .flatten();
        match res {
            Some(v) => Some(v.clone()),
            None => None,
        }
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
        match &self.base_url {
            Some(url) => match url.split_once(':') {
                Some(vv) => {
                    let port = vv.1.parse::<u16>().map_err(|_| Error::InvalidPortFormat)?;
                    Ok(Some((vv.0.to_string(), port)))
                }
                None => Err(Error::InvalidBaseUrlFormat),
            },
            None => Ok(None),
        }
    }
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
