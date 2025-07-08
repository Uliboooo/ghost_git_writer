mod config;
mod git;
mod llm;
mod prompt;

use clap::Parser;
use config::{Config, ModelConfig, Provider, Storage, storage};
use git2::Repository;

#[derive(Debug)]
enum Error {
    llm(llm::LlmError),
    git2(git2::Error),
    OpenSaveErr(storage::Error),
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

fn commit_ctrl(_cmt: Commit, model: ModelConfig) -> Result<String, Error> {
    // let head_commit = repo
    //     .head()
    //     .map_err(Error::git2)?
    //     .peel_to_commit()
    //     .map_err(Error::git2)?;
    // let head_tree = head_commit.tree().map_err(Error::git2)?;
    // let diff = repo
    //     .diff_tree_to_workdir(Some(&head_tree), Some(&mut DiffOptions::new()))
    //     .map_err(Error::git2)?;
    // let mut pa = String::new();
    // diff.print(git2::DiffFormat::Patch, |_, _, line| {
    //     if let Ok(t) = std::str::from_utf8(line.content()) {
    //         pa.push_str(t);
    //     }
    //     true
    // })
    // .map_err(Error::git2)?;
    let diff = git::get_diff("")?;
    let prompt = format!("write a git commit comment for this diff: {diff}");
    llm::call_llms(prompt, model).map_err(Error::llm)
}

fn create_readme(_rmd: Readme) -> Result<String, Error> {
    todo!()
}

fn sum(_sum: Sum, model: ModelConfig) -> Result<String, Error> {
    let diff = git::get_diff("")?;
    llm::call_llms("summarize changes", model).map_err(Error::llm)
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let repo = Repository::open("").unwrap();

    let config = config::Config::open::<Config>("").map_err(Error::OpenSaveErr)?;

    // deside using model...
    let default_model = config.llm.default_model;
    let a = match cli.model {
        Some(v) => match v.split_once('/') {
            Some(vv) => (Provider::from(vv.0), vv.1),
            None => (Provider::from(cli.provider.unwrap().as_str()), v.as_str()),
        },
        None => (default_model.0, default_model.1.as_str()),
    };
    let 
    // ~~~

    let res = match cli.subcommand {
        Commands::Cmt(commit) => commit_ctrl(commit, ModelConfig),
        Commands::Rdm(readme_f) => create_readme(readme_f),
        Commands::Sum(sum) => todo!(),
        Commands::Chat(chat) => todo!(),
    }?;

    println!("{res}");
    Ok(())
}
