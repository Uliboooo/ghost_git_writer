mod cli;
mod cli_helper;
mod commit_gen;
mod config;
mod diff_sum_gen;
mod git;
mod llm;
mod readme_gen;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    println!("{cli:?}");
}
