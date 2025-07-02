mod llm;

use clap::Parser;
use llm::ServiceModel;

enum Error {
    llm(llm::LlmError),
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

fn commit_ctrl(_cmt: Commit) -> Result<String, ()> {
    let res = llm::call_llms("pmt", ServiceModel::new("ollama", "model_name"), "api_key");
    todo!()
}

fn create_readme(_rmd: Readme) -> Result<String, ()> {
    todo!()
}

fn main() {
    let cli = Cli::parse();
    let _a = match cli.subcommand {
        Commands::Cmt(commit) => commit_ctrl(commit),
        Commands::Rdm(readme_f) => create_readme(readme_f),
    };
    println!("Hello, world!");
    todo!()
}
