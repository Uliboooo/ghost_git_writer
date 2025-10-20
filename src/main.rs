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
mod which_sem;

use crate::{
    cli::{DiffOption, RootOption},
    config::Config,
    get_input::{get_input, yes_no},
    git::get_git_status,
    helper::{find_readme, get_now},
};
use clap::Parser;
use easy_storage::Storeable;
use std::{
    env,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{stdout, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
};

const ANTHROPIC_API: &str = "GGW_ANTHROPIC_API";
const GEMINI_API: &str = "GGW_GEMINI_API";
const OPENAI_API: &str = "GGW_OPENAI_API";
const DEEPSEEK: &str = "GGW_DEEPSEEK_API";

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Store(easy_storage::Error),
    Llm(llm::Error),
    Config(config::Error),
    Cli(cli::Error),
    Git(git2::Error),
    // Cmt(commit_gen::Error),
    Rdm(readme_gen::Error),
    EnvVar,
    NotFoundHome,
    NotFoundConfig,
    NotFoundLlmField,
    NotFoundSelectedModel,
    NotFoundDefaultModel,
    NotFoundWorkFolder,
    Cancel,
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

impl From<git2::Error> for Error {
    fn from(value: git2::Error) -> Self {
        Self::Git(value)
    }
}

// impl From<commit_gen::Error> for Error {
//     fn from(value: commit_gen::Error) -> Self {
//         Self::Cmt(value)
//     }
// }

impl From<readme_gen::Error> for Error {
    fn from(value: readme_gen::Error) -> Self {
        Self::Rdm(value)
    }
}

impl From<llm::Error> for Error {
    fn from(value: llm::Error) -> Self {
        Self::Llm(value)
    }
}

impl From<env::VarError> for Error {
    fn from(_value: env::VarError) -> Self {
        Self::EnvVar
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "io error: {error}"),
            Error::Store(error) => write!(f, "save error: {error}"),
            Error::Llm(error) => write!(f, "llm error: {error}"),
            Error::Config(error) => write!(f, "config error: {error}"),
            Error::Cli(error) => write!(f, "cli error: {error}"),
            Error::Git(error) => write!(f, "git error: {error}"),
            // Error::Cmt(error) => write!(f, "commit gen error: {error}"),
            Error::Rdm(error) => write!(f, "readme gen error: {error}"),
            Error::NotFoundHome => write!(f, "not found home directory"),
            Error::NotFoundConfig => write!(f, "not found config file"),
            Error::NotFoundWorkFolder => write!(f, "not found work folder"),
            Error::Cancel => write!(f, "commit canceled"),
            Error::EnvVar => write!(f, "failed get api key as env var. please set it."),
            Error::NotFoundLlmField => write!(f, "not found llm field in config file"),
            Error::NotFoundSelectedModel => write!(f, "not found selected model in config"),
            Error::NotFoundDefaultModel => write!(f, "not found default model in config"),
        }
    }
}

/// Resolves the configuration file path.
///
/// If a path is provided, returns it. Otherwise, searches for config files in standard locations:
/// 1. `~/.config/ggw/config.toml`
/// 2. `~/.ggw.toml`
///
/// # Arguments
/// * `path` - Optional config file path
///
/// # Errors
/// Returns `Error::NotFoundHome` if home directory cannot be determined.
/// Returns `Error::NotFoundConfig` if no config file is found in standard locations.
fn resolve_config_path<T: AsRef<Path>>(path: &Option<T>) -> Result<PathBuf, Error> {
    let home_path = home::home_dir().ok_or(Error::NotFoundHome)?;

    if let Some(p) = path {
        return Ok(p.as_ref().to_path_buf());
    }

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
/// * `` -> default in config
fn resolve_model(
    config: &Option<Config>,
    root_opts: &cli::RootOptions,
) -> Result<config::Model, Error> {
    match root_opts.model() {
        Some(v) => match v.split_once('/') {
            // `-m gemini/gemini-2.0-flash`
            Some(vv) => Ok(config::Model::new(
                vv.0,
                vv.1,
                *root_opts.temperature(),
                *root_opts.max_tokens(),
                root_opts.base_url().clone(),
            )),
            // `-m gem2`
            None => match config {
                Some(loaded_config) => loaded_config.llms().clone().ok_or(Error::NotFoundLlmField),
                None => Err(Error::NotFoundConfig),
            }?
            .get_model(v)
            .ok_or(Error::NotFoundSelectedModel),
        },
        // without mode arg
        None => match config {
            Some(v) => v.llms().clone().ok_or(Error::NotFoundConfig),
            None => Err(Error::NotFoundConfig),
        }?
        .get_default()
        .ok_or(Error::NotFoundDefaultModel),
    }
}

fn resolve_api_key(model: &config::Model) -> Result<Option<String>, Error> {
    let prov = llm::Provider::try_from(model.provider().as_str())?;
    Ok(match prov {
        llm::Provider::Ollama => None,
        llm::Provider::OpenAI => Some(env::var(OPENAI_API)),
        llm::Provider::Gemini => Some(env::var(GEMINI_API)),
        llm::Provider::Anthropic => Some(env::var(ANTHROPIC_API)),
        llm::Provider::DeepSeek => Some(env::var(DEEPSEEK)),
    }
    .transpose()?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = cli::Cli::parse();

    let config_path = resolve_config_path(cli.get_root_options().config_path()).ok();
    let loaded_config = config_path
        .map(config::Config::load_by_extension)
        .transpose()
        .ok()
        .flatten();

    let work_path = resolve_work_path(&cli)?;

    let model = resolve_model(&loaded_config, &cli.get_root_options())?;
    let api_key = resolve_api_key(&model)?;

    let model_info = llm::LlmReqInfo::new_with_api(model, api_key)?;

    //let diff = git::get_diff((None, None), &work_path)?;

    let git_user = git::get_user_email()?;

    let root_options = cli.get_root_options();
    let lang = root_options.lang().as_ref();
    let extra = root_options.extra().as_ref();

    let git_status = get_git_status(&work_path)?;
    //
    // let diff = if root_options.stdin() {
    //     todo!()
    // } else {
    // }

    match &cli.subcommand {
        cli::Commands::Commit(commit) => {
            //                     👇 is from pipe.
            let diff = if !atty::is(atty::Stream::Stdin) {
                let mut input = String::new();
                std::io::stdin().lock().read_to_string(&mut input)?;
                stdout().flush()?;
                input
            } else {
                let diff_opt = commit.resolve_diff_commit();
                git::get_diff(diff_opt, &work_path)?
            };

            let msg = commit_gen::gen_commit_msg(diff, git_status, model_info, lang, extra).await?;
            let conti = || {
                let mut tty = BufReader::new(File::open("/dev/tty").unwrap());
                stdout().flush().unwrap();
                let mut ans = String::new();
                tty.read_line(&mut ans).unwrap();
                matches!(ans.trim(), "y" | "Y" | "Yes" | "yes")
            };
            if *commit.get_root_options().oneline() {
                println!("{msg}");
                Ok(())
            } else {
                let fd_msg = cli_helper::Printer::from(&msg);
                println!("Generated msg:\n{fd_msg}");

                if *commit.auto_commit() ||
                    //yes_no("commit?(y/n)") 
                    conti()
                {
                    git::git_commit(&work_path, &msg, git_user.0, git_user.1)?;
                    Ok(())
                } else {
                    Err(Error::Cancel)
                }
            }
        }
        cli::Commands::Readme(readme) => {
            let path_list = readme.export_path_list()?;
            let readme_content =
                readme_gen::gen_readme(&path_list, model_info, lang, extra).await?;
            if *readme.get_root_options().oneline() {
                println!("{readme_content}");
                Ok(())
            } else {
                println!("Generated README:\n{readme_content}\n\n");
                let readme_file = find_readme(&work_path);
                let mut f = if let Some(v) = readme_file {
                    if *readme.allow_merge() || yes_no("merge to README.md? (y/n)") {
                        OpenOptions::new().append(true).open(v)?
                    } else {
                        let now = get_now();
                        let path = work_path.join(format!("{}.md", now));
                        if yes_no("save to {now}.md?(y/n)") {
                            OpenOptions::new()
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(path)?
                        } else {
                            return Err(Error::Cancel);
                        }
                    }
                } else {
                    let path = work_path.join(format!("{}.md", get_now()));
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(path)?
                };
                Ok(f.write_all(readme_content.as_bytes())?)
            }
        }
        cli::Commands::SumDiff(diff_sum) => {
            let diff = if *root_options.stdin() {
                let mut input = String::new();
                std::io::stdin().read_to_string(&mut input)?;
                input
            } else {
                let diff_s = diff_sum.resolve_diff_commit();
                git::get_diff(diff_s, &work_path)?
            };

            let res =
                diff_sum_gen::sum_diff(diff, git_status, model_info, lang.cloned(), extra.cloned())
                    .await?;
            if *diff_sum.get_root_options().oneline() {
                println!("{res}");
                Ok(())
            } else {
                println!("diff summarize:\n{res}");
                Ok(())
            }
        }
        cli::Commands::WhichSem(which) => {
            let diff = if *root_options.stdin() {
                let mut input = String::new();
                std::io::stdin().read_to_string(&mut input)?;
                input
            } else {
                let diff_s = which.resolve_diff_commit();
                git::get_diff(diff_s, &work_path)?
            };
            let res =
                which_sem::whichi_sem(diff, git_status, model_info, lang.cloned(), extra.cloned())
                    .await?;

            if *which.get_root_options().oneline() {
                println!("{}", res.0);
                Ok(())
            } else {
                let s = cli_helper::Printer::from(res.0.trim());
                println!(
                    "shoud increase at \n{s}\nreasons:\n{}",
                    res.1.unwrap_or(String::new()).trim()
                );
                Ok(())
            }
        }
    }
}
