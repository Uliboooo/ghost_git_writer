use crate::{Cli, Error, RootOption, storage::Storage};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    prompt: Option<Prompt>,
    llm: Option<Llm>,
}

impl Config {
    pub fn new(prompt: Prompt, llm: Llm) -> Option<Self> {
        Some(Self {
            prompt: Some(prompt),
            llm: Some(llm),
        })
    }
}

impl<P: AsRef<Path>> Storage<P> for Config {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prompt {
    // 0: prompt alias
    // 1: customized prompt
    custom_prompt: HashMap<String, String>,
}

impl Prompt {
    pub fn new(customs: HashMap<String, String>) -> Self {
        Self {
            custom_prompt: customs,
        }
    }
    fn get_prompt<T: AsRef<str>>(&self, alias: T) -> Option<&String> {
        self.custom_prompt.get(alias.as_ref())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Llm {
    default_alias: Option<String>,
    model_alias: HashMap<String, Model>,
}

impl Llm {
    pub fn new(default: Option<String>, model_alias: HashMap<String, Model>) -> Self {
        Self {
            default_alias: default,
            model_alias,
        }
    }
    pub fn get_default_model(&self) -> Option<Model> {
        self.model_alias.get(self.default_alias.as_ref()?).cloned()
    }
    pub fn get_model_by_alias<T: AsRef<str>>(&self, alias: T) -> Option<Model> {
        self.model_alias.get(alias.as_ref()).cloned()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub provider: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

impl Model {
    pub fn new<T: AsRef<str>>(
        pro: T,
        model: T,
        temp: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Self {
        Self {
            provider: pro.as_ref().to_string(),
            model: model.as_ref().to_string(),
            temperature: temp,
            max_tokens,
        }
    }
}

impl TryFrom<Cli> for Model {
    type Error = Error;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        match value.get_root_options().model {
            Some(v) => match v.split_once('/') {
                Some(v) => Ok((v.0.to_string(), v.1.to_string())),
                None => Err(Error::InvalidModelFormat(v)),
            },
            None => Err(Error::FailedParseCli),
        }
        .map(|f| Model::new(f.0, f.1, None, None))
    }
}

impl Model {
    /// if value does not haev model info, get from config.
    pub fn to_model(value: Cli, config: Config) -> Result<Model, Error> {
        match Model::try_from(value) {
            Ok(v) => Ok(v),
            Err(e) => match e {
                Error::FailedParseCli => Ok(config
                    .llm
                    .and_then(|f| f.get_default_model())
                    .ok_or(Error::NotFoundDefaultModel)?),
                e => Err(e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{Config, Llm, Model, Prompt},
        storage::Storage,
    };
    use std::{collections::HashMap, env};

    #[test]
    fn create_config() {
        let mut pmtss = HashMap::new();
        pmtss.insert("test".to_string(), "this is test".to_string());
        let pmt: Prompt = Prompt::new(pmtss);

        let alias = Llm::new(None, {
            let model = Model::new("gemini", "gemini-2.0-flash", None, None);
            let mut lls = HashMap::new();
            lls.insert("ge".to_string(), model);
            lls
        });
        let save_path = env::current_dir().unwrap().join("test_config.json");
        let res = Config::new(pmt, alias).map(|f| f.save(save_path, true));
        let print_res = match res {
            Some(v) => match v {
                Ok(v) => "success".to_string(),
                Err(e) => e.to_string(),
            },
            None => "not_found".to_string(),
        };
        println!("{print_res}");
    }
}
