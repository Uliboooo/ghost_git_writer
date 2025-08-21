mod cli;
mod cli_helper;
mod commit_gen;
mod config;
mod diff_sum_gen;
mod get_input;
mod git;
mod helper;
mod llm;
mod readme_gen;

use clap::Parser;
use easy_storage::Storeable;
use git2::{Worktree, opts::get_mwindow_mapped_limit};
use ollama_rs::models::ModelInfo;
use std::{env, fs::OpenOptions, io::Write, path::PathBuf};

use crate::{
    cli::{Cli, RootOption},
    config::{Config, Model},
    get_input::yes_no,
    helper::{exist_readme, find_readme, get_now},
};

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Store(easy_storage::Error),
    Llm(llm::Error),
    Config(config::Error),
    Cli(cli::Error),
    Git(git::Error),
    Cmt(commit_gen::Error),
    Rdm(readme_gen::Error),
    NotFoundHome,
    NotFoundConfig,
    NotFoundWorkFolder,
    NotFoundAlias,
    InvalidModelName,
    DoesNotExistAlias,
    CancelCommit,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<easy_storage::Error> for Error {
    fn from(value: easy_storage::Error) -> Self {
        Self::Store(value)
    }
}

impl From<config::Error> for Error {
    fn from(value: config::Error) -> Self {
        Self::Config(value)
    }
}

impl From<cli::Error> for Error {
    fn from(value: cli::Error) -> Self {
        Self::Cli(value)
    }
}

impl From<git::Error> for Error {
    fn from(value: git::Error) -> Self {
        Self::Git(value)
    }
}

impl From<commit_gen::Error> for Error {
    fn from(value: commit_gen::Error) -> Self {
        Self::Cmt(value)
    }
}

impl From<readme_gen::Error> for Error {
    fn from(value: readme_gen::Error) -> Self {
        Self::Rdm(value)
    }
}

/// Resolves the configuration file path for the GGW application.
///
/// This function searches for configuration files in the user's home directory
/// in the following order of preference:
///
/// 1. `~/.config/ggw/config.toml` - Primary
/// 2. `~/.ggw.toml` - Secondary
///
/// # Returns
///
/// * `Ok(PathBuf)` - The path to the first existing configuration file found
/// * `Err(Error::NotFoundHome)` - If the home directory cannot be determined
/// * `Err(Error::NotFoundConfig)` - If no configuration file is found in any of the expected locations
fn resolve_config_path() -> Result<PathBuf, Error> {
    let home_path = home::home_dir().ok_or(Error::NotFoundHome)?;

    let primary = home_path
        .join(".config")
        .join("ggw")
        .join("config")
        .with_extension("toml");
    let secondary = home_path.join(".ggw").with_extension("toml");

    if primary.exists() {
        Ok(primary)
    } else if secondary.exists() {
        Ok(secondary)
    } else {
        Err(Error::NotFoundConfig)
    }
}

fn resolve_work_path<T: RootOption>(opt: &T) -> Result<PathBuf, Error> {
    let p = match opt.get_root_options().path() {
        Some(p) => PathBuf::from(p),
        None => env::current_dir()?,
    };

    if !p.exists() {
        Err(Error::NotFoundWorkFolder)
    } else {
        Ok(p)
    }
}

/// * `gemini/gemini-2.0-flash` -> (gemini, gemini-2.0-flash)
/// * `g2f` -> `(gemini, gemini-2.0-flash)` (if an alias is registered)
fn resolve_model(
    model_name: &str,
    config: &config::Config,
    root_options: &cli::RootOptions,
) -> Result<config::Model, Error> {
    match config
        .llms()
        .as_ref()
        .and_then(|llms| llms.get_model(model_name))
    {
        Some(model) => Ok(model),
        None => {
            let parts: Vec<&str> = model_name.split('/').collect();
            if parts.len() != 2 {
                return Err(Error::InvalidModelName);
            }
            Ok(config::Model::new(
                parts[0],
                parts[1],
                *root_options.temperature(),
                *root_options.max_tokens(),
                root_options.parse_base_url()?,
            ))
        }
    }
}

fn resolve_model_info(cli: &Cli, config: &Config) -> Result<llm::LlmReqInfo, Error> {
    let root_options = cli.get_root_options();
    let model_name = root_options.model();
    let model = resolve_model(model_name, config, &root_options)?;

    let provider = model
        .provider()
        .as_str()
        .try_into()
        .map_err(|_| Error::InvalidModelName)?;
    let base_url = model.base_url().clone();

    Ok(llm::LlmReqInfo::new(
        provider,
        model.model().clone(),
        None, // API key is not handled here, you might need to adjust this
        *model.temperature(),
        *model.max_tokens(),
        base_url,
    ))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = cli::Cli::parse();

    let config_path = resolve_config_path()?;
    let work_path = resolve_work_path(&cli)?;

    let config = config::Config::load_by_extension(config_path)?;

    let model_info = resolve_model_info(&cli, &config)?;

    let diff = git::get_diff(&work_path)?;

    let git_user = git::get_user_email()?;

    let root_options = cli.get_root_options();
    let lang = root_options.lang().as_ref();
    let extra = root_options.extra().as_ref();

    let res = match &cli.subcommand {
        cli::Commands::Commit(commit) => {
            let msg = commit_gen::gen_commit_msg(diff, model_info, lang, extra).await?;
            println!("Generated msg: {msg}");

            if *commit.auto_commit() || yes_no("commit?") {
                git::git_commit(&work_path, &msg, git_user.0, git_user.1)?;
                Ok(())
            } else {
                Err(Error::CancelCommit)
            }
        }
        cli::Commands::Readme(readme) => {
            let path_list = readme.export_path_list().unwrap();
            let readme_content =
                readme_gen::gen_readme(&path_list, model_info, lang, extra).await?;
            println!("Generated README:\n{readme_content}\n\n");
            let readme_file = find_readme(&work_path);
            let mut f = if let Some(v) = readme_file {
                if *readme.allow_merge() || yes_no("merge to README.md") {
                    OpenOptions::new().append(true).open(v)?
                } else {
                    let path = work_path.join(format!("{}.md", get_now()));
                    OpenOptions::new().write(true).create(true).open(path)?
                }
            } else {
                let path = work_path.join(format!("{}.md", get_now()));
                OpenOptions::new().write(true).create(true).open(path)?
            };
            Ok(f.write_all(readme_content.as_bytes())?)
        }
        cli::Commands::DiffSum(_diff_sum) => todo!(),
    };

    // Now you can use model_info
    // For example:
    // let response = llm::call_llm(model_info, "Hello, world!").await?;
    // println!("{}", response);

    Ok(())
}
