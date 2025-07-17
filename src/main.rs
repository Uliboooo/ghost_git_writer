mod cmt_msg;
mod git;
mod llm;
mod config;
mod storage;

use clap::{Parser, Subcommand};
use get_input::yes_no;
use std::{
    env::{self},
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

const ANTHROPIC_API: &str = "GGW_ANTHROPIC_API";
const GEMINI_API: &str = "GGW_GEMINI_API";
const OPENAI_API: &str = "GGW_OPENAI_API";
const DEEPSEEK: &str = "GGW_DEEPSEEK_API";

#[derive(Debug)]
pub enum Error {
    GitE(git2::Error),
    Llm(llm::LlmError),
    EnvE(env::VarError),
    FailedParseCli,
    IoE(io::Error),
    NotFoundFile,
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
        }
    }
}

#[derive(Debug, Parser, Clone)]
#[command(name = "ggw")]
#[command(about = "this cli create a git commit msg by llm")]
struct Cli {
    #[arg(short = 'y', long = "yes")]
    yes: bool,

    #[arg(short = 's', long = "service")]
    provider: Option<String>,

    #[arg(short = 'm', long = "model")]
    model: Option<String>,

    #[arg(short = 'p', long = "path")]
    path: Option<String>,

    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    #[clap(about = "gen commit msg and git commit")]
    Cmt(Commit),
    // Rdm(Readme),
    // Sum(Sum),
    // Chat(Chat),
}

#[derive(Debug, clap::Args, Clone)]
struct Commit {
    #[arg(short = 'c', long = "auto-commit", help = "allow auto git commit")]
    auto_commit: bool,
    // #[arg(long = "push")]
    // auto_push: bool,
}

// #[derive(Debug, clap::Args, Clone)]
// struct Readme {}

// #[derive(Debug, clap::Args, Clone)]
// struct Sum {}

// #[derive(Debug, clap::Args, Clone)]
// struct Chat {}

struct Model {
    provider: String,
    model_name: String,
}

impl Model {
    fn new<T: AsRef<str>>(provider: T, model_name: T) -> Self {
        Self {
            provider: provider.as_ref().to_string(),
            model_name: model_name.as_ref().to_string(),
        }
    }
}

// impl From<Cli> for Model {
//     fn from(value: Cli) -> Self {
//         let res = match value.model {
//             Some(v) => match v.split_once('/') {
//                 Some(v) => (v.0.to_string(), v.1.to_string()),
//                 None => (value.provider.unwrap(), v.to_string()),
//             },
//             None => todo!(),
//         };
//             provider: value.provider.unwrap(),
//             model_name: value.model.unwrap(),
//         }
//     }
// }

impl TryFrom<Cli> for Model {
    type Error = Error;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        match value.model {
            Some(v) => match v.split_once('/') {
                Some(v) => Ok((v.0.to_string(), v.1.to_string())),
                None => Ok((value.provider.unwrap(), v.to_string())),
            },
            None => Err(Error::FailedParseCli),
        }
        .map(|f| Model::new(f.0, f.1))
    }
}

// fn create_msg<T: AsRef<str>>(
//     diff: String,
//     model: Model,
//     api_key: Option<T>,
// ) -> Result<String, Error> {
//     cmt_msg::create_cmt_msg(diff, model, api_key.map(|f| f.as_ref().to_string()))
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

    // println!("created_msg:\n\n{commit_msg}");

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

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let use_model = Model::try_from(cli.clone())?;
    let resolved_api_key = resolve_api_key(&use_model)
        .transpose()
        .map_err(Error::EnvE)?;
    let path = resolve_work_path(cli.clone())?;

    match &cli.subcommand {
        Commands::Cmt(commit) => {
            println!("<<<commit mode>>>\n\nread git diff...\ncreating commmit message...");
            let msg = commit_from_gitdiff(
                &path,
                use_model,
                resolved_api_key,
                // commit.auto_commit,
                // cli.yes,
            )?;
            println!("created msg:{msg}");
            let git_user = git::get_user_email()?;

            if commit.auto_commit || cli.yes || yes_no("\ncontinue?(y/n)>") {
                git::git_commit(path, &msg, git_user.0, git_user.1)?;
            }
        } //     Commands::Rdm(readme) => todo!(),
          //     Commands::Sum(sum) => todo!(),
          //     Commands::Chat(chat) => todo!(),
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
            crate::Model::new("gemini", "gemini-2.0-flash"),
            Some(a),
        );
        println!("{res:?}");
    }
}
