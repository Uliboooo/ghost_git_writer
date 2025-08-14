use crate::{Cli, Error, RootOption, storage::Storage};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn get_default_m(self) -> Option<Model> {
        self.llm?.get_default_model()
    }

    pub fn get_model_by_alias<T: AsRef<str>>(&self, alias: T) -> Option<Model> {
        self.llm.as_ref()?.get_model_by_alias(alias)
    }
}

impl<P: AsRef<Path>> easy_storage::Storeable<P> for Config {}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    fn get_default_model(&self) -> Option<Model> {
        self.model_alias.get(self.default_alias.as_ref()?).cloned()
    }
    fn get_model_by_alias<T: AsRef<str>>(&self, alias: T) -> Option<Model> {
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

impl TryFrom<String> for Model {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.split_once('/') {
            Some(v) => Ok(Model::new(v.0, v.1, None, None)),
            None => Err(Error::FailedParseCli),
        }
    }
}

impl TryFrom<Config> for Model {
    type Error = Error;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        value
            .llm
            .and_then(|f| f.get_default_model())
            .ok_or(Error::NotFoundDefaultModel)
    }
}

impl Model {
    /// Resolves a `Model` from CLI arguments and configuration.
    ///
    /// This function attempts to create a `Model` by checking CLI arguments in the following priority order:
    /// 1. If the `-m` flag is provided with a model specification, parse it directly
    /// 2. If an alias is provided via CLI arguments, resolve it to a model
    /// 3. If neither is provided, fall back to the default model from the configuration
    ///
    /// # Arguments
    ///
    /// * `value` - The `Cli` struct containing parsed command-line arguments
    /// * `config` - The `Config` struct containing application configuration including model aliases
    ///
    /// # Returns
    ///
    /// Returns `Result<Model, Error>`:
    /// - `Ok(Model)` if a model could be successfully resolved from any of the sources
    /// - `Err(Error)` if:
    ///   - The model format is invalid when parsing from `-m` flag
    ///   - The alias cannot be resolved to a model
    ///   - No default model is configured and no CLI arguments are provided
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let cli = Cli::parse();
    /// let config = Config::load()?;
    /// let model = Model::to_model(cli, config)?;
    /// ```
    pub fn to_model(value: Cli, config: Config) -> Result<Model, Error> {
        // if exist arg `-m`
        match value.get_root_options().model {
            // if exist arg `-m`
            Some(mm) => Model::try_from(mm),
            None => match value.get_root_options().alias {
                // if exists `-a` arg
                Some(alias_name) => config
                    .get_model_by_alias(alias_name.clone())
                    .ok_or(Error::NotFoundModelAlias(alias_name)),
                None => Model::try_from(config),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use easy_storage::Storeable;

    use crate::{
        config::{Config, Llm, Model, Prompt},
        // storage::Storage,
    };
    use std::{collections::HashMap, env};

    #[test]
    fn create_config() {
        let mut pmtss = HashMap::new();
        pmtss.insert("test".to_string(), "this is test".to_string());
        let pmt: Prompt = Prompt::new(pmtss);

        let alias = Llm::new(Some("ge".to_string()), {
            let model = Model::new("gemini", "gemini-2.0-flash", None, None);
            let mut lls = HashMap::new();
            lls.insert("ge".to_string(), model);
            lls
        });
        let save_path = env::current_dir().unwrap().join("test_config.toml");
        // let res =
        // Config::new(pmt, alias).map(|f| f.save(save_path, true, easy_storage::Format::Toml));

        let res = Config::new(pmt, alias).unwrap();
        let save_res = res.save(save_path, true, easy_storage::Format::Toml);

        let print_res = match save_res {
            Ok(_) => "success".to_string(),
            Err(e) => e.to_string(),
        };
        println!("{print_res}");
    }
}
