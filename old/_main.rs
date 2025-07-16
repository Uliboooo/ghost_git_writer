mod config;
mod git;
mod llm;
mod prompt;

use clap::Parser;
use config::{Config, ModelConfig, ModelInfo, Provider, Storage, storage};
use git2::Repository;
use ollama_rs::IntoUrlSealed;

#[derive(Debug)]
enum Error {
    Llm(llm::LlmError),
    Git2(git2::Error),
    OpenSaveErr(storage::Error),
    NotEnoughArgs(String),
}

#[derive(Debug, Parser)]
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

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Cmt(Commit),
    Rdm(Readme),
    Sum(Sum),
    Chat(Chat),
}

#[derive(Debug, clap::Args)]
struct Commit {
    #[arg(short = 'c', long = "no-commit")]
    no_commit: bool,

    #[arg(short = 'p', long = "auto-push")]
    auto_push: bool,
}

#[derive(Debug, clap::Args)]
struct Readme {}

#[derive(Debug, clap::Args)]
struct Sum {}

#[derive(Debug, clap::Args)]
struct Chat {}

fn commit_ctrl(_cmt: Commit, model: &ModelInfo) -> Result<String, Error> {
    let diff = git::get_diff("")?;
    let prompt = format!("write a git commit comment for this diff: {diff}");
    llm::call_llms(prompt, model)
}

fn create_readme(_rmd: Readme, model: &ModelInfo) -> Result<String, Error> {
    todo!()
}

fn diff_sum(_sum: Sum, _model: &ModelInfo) -> Result<String, Error> {
    let diff = git::get_diff("")?;
    llm::call_llms(format!("summaeize changes \ndiff: {diff}"), _model)
}

fn use_chat_mode(_chat: Chat, _model: &ModelInfo) -> Result<String, Error> {
    todo!()
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let repo = Repository::open("").unwrap();

    let config = config::Config::open::<Config>("").map_err(Error::OpenSaveErr)?;

    let use_model = match config.get_default_model() {
        Some(v) => Ok(*v.clone()),
        None => {
            // get Args
            match cli.model.map(|m| match m.split_once('/') {
                Some(p_m) => (Provider::from(p_m.0), p_m.1.to_string()),
                None => (Provider::from(cli.provider.unwrap()), m),
            }) {
                Some(v) => Ok(v),
                None => Err(Error::NotEnoughArgs("Not enought args".to_string())),
            }
        }
    }?;
    let use_model_info = &(
        use_model.0.clone(),
        use_model.1.clone(),
        config.get_config(&use_model.0, &use_model.1).unwrap(),
    );

    // deside using model...
    //    let default_model = config.get_default_model();    let a = match cli.model {
    //    let use_model = Some(v); => match v.split_once('/') {
    //         // -m openai/chatgpt3.5
    //         Some(vv) => (Provider::from(vv.0), vv.1.to_string()),
    //         // -p openai -m chatgpt3.5
    //         None => (Provider::from(cli.provider.unwrap().as_str()), v),
    //     },
    //     // no `pro/mod`
    //     None => {

    //     }
    // };

    // let use_model = {
    //     match config.get_default_model() {
    //         Some(info) => Ok(info),
    //         None => {
    //             match cli.model {
    //                 Some(cli_model) => {
    //                     let a = match cli_model.split_once('/') {
    //                         Some(p_m) => (Provider::from(p_m.0), p_m.1.to_string()),
    //                         None => (Provider::from(cli.provider.unwrap()), cli_model.clone()),
    //                     };
    //                     let m_cf = config.get_config(&a.0, &a.1).unwrap();
    //                     Ok(&(a.0, a.1, *m_cf))
    //                 },
    //                 None => Err(Error::NotEnoughArgs("Could not determine model Config not present or insufficient arguments".to_string())),
    //             }
    //         },
    //     }
    // }?;

    let res = match cli.subcommand {
        Commands::Cmt(commit) => commit_ctrl(commit, use_model_info),
        Commands::Rdm(readme_f) => create_readme(readme_f, use_model_info),
        Commands::Sum(sum) => diff_sum(sum, use_model_info),
        Commands::Chat(chat) => use_chat_mode(chat, use_model_info),
    }?;

    println!("{res}");
    Ok(())
}
