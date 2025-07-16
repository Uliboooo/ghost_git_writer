mod cmt_msg;
mod git;
mod llm;

use clap::{Parser, Subcommand};
use std::{env, path::Path};

const ANTHROPIC_API: &str = "GGW_ANTHROPIC_API";
const GEMINI_API: &str = "GGW_GEMINI_API";
const OPENAI_API: &str = "GGW_OPENAI_API";
const DEEPSEEK: &str = "GGW_DEEPSEEK_API";

#[derive(Debug)]
pub enum Error {
    GitE(git2::Error),
    Llm(llm::LlmError),
    EnvE(env::VarError),
}

#[derive(Debug, Parser, Clone)]
struct Cli {
    #[arg(short = 'r', long = "no-rewrite")]
    no_rewrite: bool,

    #[arg(short = 'y', long = "yes")]
    yes: bool,

    #[arg(short = 'p', long = "provider")]
    provider: Option<String>,

    #[arg(short = 'm', long = "model")]
    model: Option<String>,

    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    Cmt(Commit),
    // Rdm(Readme),
    // Sum(Sum),
    // Chat(Chat),
}

#[derive(Debug, clap::Args, Clone)]
struct Commit {
    #[arg(short = 'c', long = "auto-commit")]
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

impl From<Cli> for Model {
    fn from(value: Cli) -> Self {
        Self {
            provider: value.provider.unwrap(),
            model_name: value.model.unwrap(),
        }
    }
}

fn create_msg<T: AsRef<str>>(
    diff: String,
    model: Model,
    api_key: Option<T>,
) -> Result<String, Error> {
    cmt_msg::create_cmt_msg(diff, model, api_key.map(|f| f.as_ref().to_string()))
}

fn commit_from_gitdiff<T: AsRef<Path>, U: AsRef<str>>(
    project_path: T,
    model: Model,
    api_key: Option<U>,
    options: (&Cli, &Commit),
) -> Result<String, Error> {
    let git_diff = git::get_diff(&project_path)?;
    let msg = create_msg(git_diff, model, api_key)?;

    if options.1.auto_commit
        || options.0.yes
        || get_input::yes_no(format!("cmt: {}\ncontinue?(y/n)", &msg))
    {
        git::git_commit(project_path, &msg)?;
    }

    Ok(msg)
}

fn resolve_api_key(model: &Model) -> Option<Result<String, env::VarError>> {
    match model.model_name.as_str() {
        "anthropic" => Some(env::var(ANTHROPIC_API)),
        "deepseek" => Some(env::var(GEMINI_API)),
        "gemini" => Some(env::var(OPENAI_API)),
        "openai" => Some(env::var(DEEPSEEK)),
        _ => None,
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let use_model = Model::from(cli.clone());
    let resolved_api_key = resolve_api_key(&use_model)
        .transpose()
        .map_err(Error::EnvE)?;

    let _result = match &cli.subcommand {
        Commands::Cmt(commit) => {
            commit_from_gitdiff("project_path", use_model, resolved_api_key, (&cli, commit))
        } //     Commands::Rdm(readme) => todo!(),
          //     Commands::Sum(sum) => todo!(),
          //     Commands::Chat(chat) => todo!(),
    };
    Ok(())
}
