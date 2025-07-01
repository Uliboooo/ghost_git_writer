mod llm;

use clap::Parser;

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
    #[arg(short='c', long="no-commit")]
    no_commit: bool,
}

#[derive(Debug, clap::Args)]
struct Readme {

}

pub enum ServiceModel {
    Ollama(String),
}

fn commit(cmt: Commit) -> Result<(),()> {
    
    Ok(())
}

fn readme(rmd: Readme) -> Result<(), ()> {
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    match cli.subcommand {
        Commands::Cmt(commit) => commit(commit),
        Commands::Rdm(readme) => readme(readme),
    }

    println!("Hello, world!");
}
