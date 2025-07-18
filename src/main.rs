mod cli_helper;
mod cmt_msg;
mod config;
mod git;
mod llm;
mod read_codes;
mod readme;
mod storage;
mod sum;

use chrono::Local;
use clap::{Parser, Subcommand};
use config::Model;
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
    StrE(storage::Error),
    FailedParseCli,
    IoE(io::Error),
    NotFoundFile,
    InvalidModelFormat(String),
    NotSettingPath,
    NotFoundHome,
    NotFoundConfig(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GitE(e) => write!(f, "git error: {e}"),
            Error::Llm(e) => write!(f, "llm error: {e}"),
            Error::EnvE(e) => write!(f, "enviroment var error: {e}"),
            Error::FailedParseCli => write!(f, "failed parse cli"),
            Error::IoE(e) => write!(f, "io error: {e}"),
            Error::NotFoundFile => write!(f, "not found file"),
            Error::InvalidModelFormat(e) => write!(f, "incalid model format {e}"),
            Error::NotSettingPath => write!(f, "file path could not be read"),
            Error::NotFoundConfig(p) => write!(f, "not found config at {p}"),
            Error::NotFoundHome => write!(f, "not found home dir in your machine"),
            Error::StrE(error) => write!(f, "storage error: {error}"),
        }
    }
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
    #[arg(short = 'm', long = "model", help = "-m gemini/gemini-2.0-flash")]
    model: Option<String>,

    #[arg(short = 'd', long = "default-model", help = "use default model")]
    default_model: Option<String>,

    #[arg(short = 'p', long = "path", help = "work path")]
    path: Option<String>,

    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    #[command(name = "cmt", about = "gen commit msg and git commit")]
    Cmt(Commit),

    #[command(name = "rdm", about = "create a readme")]
    Rdm(Readme),

    #[command(name = "sum", about = "out diff summary")]
    Sum(Sum),
    // Chat(Chat),
}

#[derive(Debug, clap::Args, Clone)]
struct Commit {
    #[arg(short = 'c', long = "auto-commit", help = "allow auto git commit")]
    auto_commit: bool,

    #[arg(short = 'a', long = "cumstom-prompt", help = "add custom prompt")]
    a: bool,
}

#[derive(Debug, clap::Args, Clone)]
struct Readme {
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

#[derive(Debug, clap::Args, Clone)]
struct Sum {}

// #[derive(Debug, clap::Args, Clone)]
// struct Chat {}

// struct Model {
//     provider: String,
//     model_name: String,
// }
//
// impl Model {
//     fn new<T: AsRef<str>>(provider: T, model_name: T) -> Self {
//         Self {
//             provider: provider.as_ref().to_string(),
//             model_name: model_name.as_ref().to_string(),
//         }
//     }
// }
//
// impl TryFrom<Cli> for Model {
//     type Error = Error;
//
//     fn try_from(value: Cli) -> Result<Self, Self::Error> {
//         match value.model {
//             Some(v) => match v.split_once('/') {
//                 Some(v) => Ok((v.0.to_string(), v.1.to_string())),
//                 None => Err(Error::InvalidModelFormat(v)),
//             },
//             None => Err(Error::FailedParseCli),
//         }
//         .map(|f| Model::new(f.0, f.1))
//     }
// }

fn commit_from_gitdiff<T: AsRef<Path>, U: AsRef<str>>(
    project_path: &T,
    model: Model,
    api_key: Option<U>,
    // options: (&Cli, &Commit),
    // ⚠️configとかにまとめるかも
    // 拡張性が低い
    // auto_commit: bool,
    // yes_option: bool,
) -> Result<String, Error> {
    let git_diff = git::get_diff(project_path)?;
    let commit_msg =
        cmt_msg::create_cmt_msg(git_diff, model, api_key.map(|f| f.as_ref().to_string()))?;

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

fn resolve_work_path(cli: Cli) -> Result<PathBuf, Error> {
    let p = match cli.path {
        Some(p) => PathBuf::from(p),
        None => env::current_dir().map_err(Error::IoE)?,
    };

    if !p.exists() {
        Err(Error::NotFoundFile)
    } else {
        Ok(p)
    }
}

fn resolve_config_path() -> Result<PathBuf, Error> {
    let home_conf = home::home_dir().ok_or(Error::NotFoundHome)?;

    if home_conf.join(".ggw.json").exists() {
        Ok(home_conf.join(".ggw.json"))
    } else if home_conf.join(".ggw").join(".ggw.conf").exists() {
        Ok(home_conf.join(".ggw").join(".ggw.conf"))
    } else {
        Err(Error::NotFoundConfig("".to_string()))
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let _config =
        config::Config::open::<config::Config>(resolve_config_path()?).map_err(Error::StrE)?;

    let pj_path = resolve_work_path(cli.clone())?;

    // let use_model = if let Some(d) = cli.default_model {
    //     config.get_default_model()
    // } else {
    //     let a = Model::try_from(cli.model)?;
    //     Some(&a)
    // };

    let use_model = Model::try_from(cli.clone())?;

    let resolved_api_key = resolve_api_key(&use_model)
        .transpose()
        .map_err(Error::EnvE)?;

    match &cli.subcommand {
        Commands::Cmt(commit) => {
            println!("<<<commit mode>>>\n\nread git diff...\ncreating commmit message...");
            let msg = commit_from_gitdiff(
                &pj_path,
                use_model,
                resolved_api_key,
                // commit.auto_commit,
                // cli.yes,
            )?;

            println!("created msg:{msg}");
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
                git::git_commit(pj_path, &msg, git_user.0, git_user.1)?;
            }
        }
        Commands::Sum(_sum) => {
            println!("<<<sumarize mode>>> \n\nread git diff...\nsummarizing diff...");
            let git_diff = git::get_diff(pj_path)?;
            let sum = summarize_diff(git_diff, use_model, resolved_api_key)?;
            println!("summarize:\n\n{sum}");
        }
        Commands::Rdm(r) => {
            println!("<<readme mode>>> \n\nread project...\ncreating README");

            let p = {
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

            let readme_s = readme::create_readme(p.as_ref(), use_model, resolved_api_key)?;

            let save_path = readme::find_readme(&pj_path)
                .filter(|_| r.allow_merge)
                .unwrap_or_else(|| {
                    let now = Local::now().format("%b-%d-%H-%M").to_string();
                    pj_path.join(now).with_extension("md")
                });

            println!("created readme\n{readme_s}");
            if cli.yes || yes_no(format!("save to {}?", save_path.to_string_lossy())) {
                let a = if r.allow_merge {
                    readme::merge_readme(&save_path, r.allow_over_write, readme_s)
                } else {
                    readme::save_new_readme(&save_path, r.allow_over_write, readme_s)
                };
                match a {
                    Ok(_) => println!("success! save to {}", save_path.to_string_lossy()),
                    Err(e) => return Err(e),
                }
            }
        }
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use std::env::{self, current_dir};

    use crate::commit_from_gitdiff;

    #[test]
    fn cmt_test() {
        let a = env::var("GEMINI_API_KEY").unwrap();
        let p = current_dir().unwrap();
        println!("project_path: {p:?}");
        let res = commit_from_gitdiff(
            &p,
            crate::Model::new("gemini", "gemini-2.0-flash", None, None),
            Some(a),
        );
        println!("{res:?}");
    }
}
