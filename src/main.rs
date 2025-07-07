mod config;
mod git;
mod llm;
mod prompt;

use clap::Parser;
use config::ModelInfo;
use git2::{DiffOptions, Repository};

#[derive(Debug)]
enum Error {
    llm(llm::LlmError),
    git2(git2::Error),
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'r', long = "no-rewrite")]
    no_rewrite: bool,

    #[arg(short = 'y', long = "yes")]
    yes: bool,

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

fn commit_ctrl(_cmt: Commit, repo: git2::Repository) -> Result<String, Error> {
    let head_commit = repo
        .head()
        .map_err(Error::git2)?
        .peel_to_commit()
        .map_err(Error::git2)?;
    let head_tree = head_commit.tree().map_err(Error::git2)?;
    let diff = repo
        .diff_tree_to_workdir(Some(&head_tree), Some(&mut DiffOptions::new()))
        .map_err(Error::git2)?;
    let mut pa = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        if let Ok(t) = std::str::from_utf8(line.content()) {
            pa.push_str(t);
        }
        true
    })
    .map_err(Error::git2)?;
    Ok(pa)
}

fn create_readme(_rmd: Readme) -> Result<String, Error> {
    todo!()
}

fn sum(_sum: Sum, model: ModelInfo) -> Result<String, Error> {
    let diff = git::get_diff("")?;
    llm::call_llms("summarize changes", model).map_err(Error::llm)
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let repo = Repository::open("").unwrap();

    let res = match cli.subcommand {
        Commands::Cmt(commit) => commit_ctrl(commit, repo),
        Commands::Rdm(readme_f) => create_readme(readme_f),
        Commands::Sum(sum) => todo!(),
    }?;

    println!("{}", res);
    Ok(())
}
