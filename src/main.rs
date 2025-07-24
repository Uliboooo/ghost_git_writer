mod cli_helper;
mod cmt_msg;
mod config;
mod custom_prompt;
mod git;
mod llm;
mod read_codes;
mod readme;
mod storage;
mod sum;

use chrono::Local;
use clap::{Args, Parser, Subcommand};
use config::Model;
use custom_prompt::custom_prompt;
use dialoguer::Input;
use get_input::yes_no;
use std::{
    env::{self},
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
};
use storage::Storage;
use sum::summarize_diff;

const ANTHROPIC_API: &str = "GGW_ANTHROPIC_API";
const GEMINI_API: &str = "GGW_GEMINI_API";
const OPENAI_API: &str = "GGW_OPENAI_API";
const DEEPSEEK: &str = "GGW_DEEPSEEK_API";

#[derive(Debug)]
pub enum Error {
    GitE(git2::Error),
    Llm(llm::LlmError),
    EnvE(env::VarError),
    StorageE(storage::Error),
    FailedParseCli,
    IoE(io::Error),
    NotFoundFile,
    InvalidModelFormat(String),
    NotSettingPath,
    NotFoundHome,
    NotFoundConfig,
    NotFoundDefaultModel,
    NotFoundModelAlias(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GitE(e) => write!(f, "git error: {e}"),
            Error::Llm(e) => write!(f, "llm error: {e}"),
            Error::EnvE(e) => write!(f, "environment var error: {e}"),
            Error::FailedParseCli => write!(f, "failed parse cli"),
            Error::IoE(e) => write!(f, "io error: {e}"),
            Error::NotFoundFile => write!(f, "not found file"),
            Error::InvalidModelFormat(e) => write!(f, "invalid model format {e}"),
            Error::NotSettingPath => write!(f, "file path could not be read"),
            Error::NotFoundConfig => write!(f, "not found config"),
            Error::NotFoundHome => write!(f, "not found home dir in your machine"),
            Error::StorageE(error) => write!(f, "storage error: {error}"),
            Error::NotFoundDefaultModel => write!(f, ""),
            Error::NotFoundModelAlias(model) => write!(f, "not found model alias {model}"),
        }
    }
}

trait RootOption {
    fn get_root_options(&self) -> RootOptions;
}

#[derive(Debug, Args, Clone)]
struct RootOptions {
    #[arg(
        short = 'm',
        long = "model",
        help = "-m gemini/gemini-2.0-flash",
        // required only if `--alias` is not specified
        conflicts_with = "alias"
    )]
    model: Option<String>,

    #[arg(
        short = 'a',
        long = "alias",
        help = "registed alias",
        conflicts_with = "model"
    )]
    alias: Option<String>,

    #[arg(short = 'p', long = "path", help = "work path")]
    path: Option<String>,

    #[arg(short = 'o', long = "one-line", help = "print only result")]
    oneline: bool,

    #[arg(short = 'l', long = "lang", help = "change output language")]
    lang: Option<String>,

    #[arg(short = 'e', long = "extra", help = "extra prompt")]
    extra: Option<String>,
}

#[derive(Debug, Parser, Clone)]
#[command(
    name = "ggw",
    version,
    about = "this cli create a git commit msg by llm"
)]
struct Cli {
    #[arg(short = 'y', long = "yes")]
    yes: bool,

    // #[arg(short = 's', long = "service")]
    // provider: Option<String>,
    // #[arg(short = 'm', long = "model", help = "-m gemini/gemini-2.0-flash")]
    // model: Option<String>,
    // #[arg(short = 'd', long = "default-model", help = "use default model")]
    // default_model: Option<String>,

    // #[arg(short = 'p', long = "path", help = "work path")]
    // path: Option<String>,

    // #[arg(short = 'o', long = "one-line", help = "print only result")]
    // oneline: bool,
    #[command(subcommand)]
    subcommand: Commands,
}

impl RootOption for Cli {
    fn get_root_options(&self) -> RootOptions {
        self.subcommand.get_root_options()
    }
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    #[command(name = "cmt", about = "gen commit msg and git commit")]
    Cmt(Commit),

    #[command(name = "rdm", about = "create a readme")]
    Rdm(Readme),

    #[command(name = "sum", about = "out diff summary")]
    Sum(Sum),

    #[command(name = "cst", about = "use custom prompt")]
    Cst(Cst), // Chat(Chat),
}

impl RootOption for Commands {
    fn get_root_options(&self) -> RootOptions {
        match self {
            Commands::Cmt(commit) => commit.get_root_options(),
            Commands::Rdm(readme) => readme.get_root_options(),
            Commands::Sum(sum) => sum.get_root_options(),
            Commands::Cst(cst) => cst.get_root_options(),
        }
    }
}

#[derive(Debug, clap::Args, Clone)]
struct Commit {
    #[command(flatten)]
    root_options: RootOptions,

    #[arg(short = 'c', long = "auto-commit", help = "allow auto git commit")]
    auto_commit: bool,
    // #[arg(short = 'e', long = "extra", help = "add custom prompt")]
    // additional_pmt: bool,
}

impl RootOption for Commit {
    fn get_root_options(&self) -> RootOptions {
        self.root_options.clone()
    }
}

#[derive(Debug, clap::Args, Clone)]
struct Readme {
    #[command(flatten)]
    root_options: RootOptions,

    #[arg(
        short = 's',
        long = "sources",
        conflicts_with = "dir",
        required_unless_present = "dir"
    )]
    source_path_list: Option<Vec<String>>,

    #[arg(
        short = 'd',
        long = "directory",
        conflicts_with = "source_path_list",
        required_unless_present = "source_path_list"
    )]
    dir: Option<String>,

    #[arg(short = 'm', long = "allow-merge")]
    allow_merge: bool,

    #[arg(short = 'o', long = "over-write")]
    allow_over_write: bool,
}

impl RootOption for Readme {
    fn get_root_options(&self) -> RootOptions {
        self.root_options.clone()
    }
}

#[derive(Debug, clap::Args, Clone)]
struct Sum {
    #[command(flatten)]
    root_options: RootOptions,
}

impl RootOption for Sum {
    fn get_root_options(&self) -> RootOptions {
        self.root_options.clone()
    }
}

// #[derive(Debug, clap::Args, Clone)]
// struct Chat {}

#[derive(Debug, clap::Args, Clone)]
struct Cst {
    #[command(flatten)]
    root_options: RootOptions,

    preset: String,
}

impl RootOption for Cst {
    fn get_root_options(&self) -> RootOptions {
        self.root_options.clone()
    }
}

fn commit_from_gitdiff<T: AsRef<Path>, U: AsRef<str>>(
    project_path: &T,
    model: Model,
    api_key: Option<U>,
    lang: Option<U>,
    extra: Option<U>,
) -> Result<String, Error> {
    let git_diff = git::get_diff(project_path)?;
    let commit_msg = cmt_msg::create_cmt_msg(
        git_diff,
        model,
        api_key.map(|f| f.as_ref().to_string()),
        lang,
        extra,
    )?;

    Ok(commit_msg)
}

fn resolve_api_key(model: &Model) -> Option<Result<String, env::VarError>> {
    match model.provider.as_str() {
        "anthropic" => Some(env::var(ANTHROPIC_API)),
        "deepseek" => Some(env::var(DEEPSEEK)),
        "gemini" => Some(env::var(GEMINI_API)),
        "openai" => Some(env::var(OPENAI_API)),
        _ => None,
    }
}

fn resolve_work_path<T: RootOption>(option: &T) -> Result<PathBuf, Error> {
    let p = match option.get_root_options().path {
        Some(p) => PathBuf::from(p),
        None => env::current_dir().map_err(Error::IoE)?,
    };

    if !p.exists() {
        Err(Error::NotFoundFile)
    } else {
        Ok(p)
    }
}

/// Resolves the configuration file path for the GGW application.
///
/// This function searches for configuration files in the user's home directory
/// in the following order of preference:
///
/// 1. `~/.ggw.json` - Primary configuration file in JSON format
/// 2. `~/.ggw/.ggw.json` - Secondary configuration file in the `.ggw` subdirectory
///
/// # Returns
///
/// * `Ok(PathBuf)` - The path to the first existing configuration file found
/// * `Err(Error::NotFoundHome)` - If the home directory cannot be determined
/// * `Err(Error::NotFoundConfig)` - If no configuration file is found in any of the expected locations
///
/// # Examples
///
/// ```rust
/// match resolve_config_path() {
///     Ok(config_path) => println!("Found config at: {}", config_path.display()),
///     Err(e) => eprintln!("Config resolution failed: {}", e),
/// }
/// ```
fn resolve_config_path() -> Result<PathBuf, Error> {
    let home_path = home::home_dir().ok_or(Error::NotFoundHome)?;

    let primary = home_path.join(".ggw.json");
    let secondary = home_path.join(".ggw").join(".ggw.json");

    if primary.exists() {
        Ok(primary)
    } else if secondary.exists() {
        Ok(secondary)
    } else {
        Err(Error::NotFoundConfig)
    }
}

#[test]
fn res() {
    let res = resolve_config_path();
    println!("{res:?}");
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let conf_path = resolve_config_path()?;

    println!("{conf_path:?}");

    let conf = config::Config::open::<config::Config>(conf_path).map_err(Error::StorageE)?;

    let work_path = resolve_work_path(&cli.subcommand)?;

    let use_model = Model::to_model(cli.clone(), conf)?;

    let resolved_api_key = resolve_api_key(&use_model)
        .transpose()
        .map_err(Error::EnvE)?;

    match &cli.subcommand {
        Commands::Cmt(commit) => {
            println!("<<<commit mode>>>\n\nread git diff...\ncreating commit message...\n");
            let msg = commit_from_gitdiff(
                &work_path,
                use_model,
                resolved_api_key,
                cli.get_root_options().lang, // commit.auto_commit,
                cli.get_root_options().extra,
            )?;

            if cli.get_root_options().oneline {
                println!("{msg}");
            } else {
                println!("created msg:{msg}");
            }

            if !cli.get_root_options().oneline {
                let msg = if yes_no("do you edit msg?(y/n)") {
                    Input::new()
                        .with_prompt("edit")
                        .default(msg.clone())
                        .interact_text()
                        .unwrap()
                } else {
                    msg
                };

                let git_user = git::get_user_email()?;

                if commit.auto_commit || cli.yes || yes_no("\ncontinue?(y/n)>") {
                    git::git_commit(work_path, &msg, git_user.0, git_user.1)?;
                }
            }
        }
        Commands::Sum(_sum) => {
            if !cli.get_root_options().oneline {
                println!("<<<summarize mode>>> \n\nread git diff...\nsummarizing diff...");
            }
            let git_diff = git::get_diff(work_path)?;
            let sum = summarize_diff(
                git_diff,
                use_model,
                resolved_api_key,
                cli.get_root_options().lang,
                cli.get_root_options().extra,
            )?;

            println!(
                "{}\n\n{sum}",
                if cli.get_root_options().oneline {
                    ""
                } else {
                    "summarize:"
                }
            );
        }
        Commands::Rdm(r) => {
            if !cli.get_root_options().oneline {
                println!("<<readme mode>>> \n\nread project...\ncreating README");
            }
            let resolved_path_list = {
                match r.source_path_list.clone() {
                    Some(v) => v,
                    None => match r.dir.clone() {
                        Some(v) => {
                            let l = fs::read_dir(v).map_err(Error::IoE)?;
                            let mut ll = Vec::new();
                            l.for_each(|i| {
                                if let Ok(pp) = i {
                                    ll.push(pp.path().to_string_lossy().to_string());
                                }
                            });
                            ll
                        }
                        None => return Err(Error::NotSettingPath),
                    },
                }
            };
            let lang = cli.get_root_options().lang;
            let readme_s = readme::create_readme(
                resolved_path_list.as_ref(),
                use_model,
                resolved_api_key,
                lang,
                cli.get_root_options().extra,
            )?;

            let save_path = readme::find_readme(&work_path)
                .filter(|_| r.allow_merge)
                .unwrap_or_else(|| {
                    let now = Local::now().format("%b-%d-%H-%M").to_string();
                    work_path.join(now).with_extension("md")
                });

            println!(
                "created readme{}\n{readme_s}",
                if cli.get_root_options().oneline {
                    ""
                } else {
                    "created readme"
                }
            );
            if cli.yes || yes_no(format!("save to {}?", save_path.to_string_lossy())) {
                let a = if r.allow_merge {
                    readme::merge_readme(&save_path, r.allow_over_write, readme_s)
                } else {
                    readme::save_new_readme(&save_path, r.allow_over_write, readme_s)
                };
                match a {
                    Ok(_) => {
                        if !cli.get_root_options().oneline {
                            println!("success! save to {}", save_path.to_string_lossy())
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        Commands::Cst(cst) => {
            if !cli.get_root_options().oneline {
                println!("<<<custom prompt mode>>>");
            }
            let res = custom_prompt(
                cst.clone().preset,
                use_model,
                resolved_api_key,
                cli.get_root_options().extra,
            )?;
            println!("\n{res}");
        }
    };
    Ok(())
}
