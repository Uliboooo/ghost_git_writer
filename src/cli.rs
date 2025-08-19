use clap;

#[derive(Debug, clap::Parser, Clone)]
#[command(name = "ggw", version, about = "generate git commit msg by llm")]
pub struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}

impl RootOption for Cli {
    fn get_root_options(&self) -> RootOptions {
        self.subcommand.get_root_options()
    }
}

trait RootOption {
    fn get_root_options(&self) -> RootOptions;
}

#[derive(Debug, clap::Args, Clone)]
struct RootOptions {
    #[arg(
        short = 'm',
        long = "model",
        conflicts_with = "alias",
        help = "`-m gemini/gemino-2.0-flash`"
    )]
    model: Option<String>,

    #[arg(
        short = 'a',
        long = "alias",
        conflicts_with = "model",
        help = "`-a [alias-name]`"
    )]
    alias: Option<String>,

    #[arg(short = 'p', long = "path", help = "work path. git project root path.")]
    path: Option<String>,

    #[arg(short = 'l', long = "lang", help = "language. `-l japanese`")]
    lang: Option<String>,

    #[arg(
        short = 'e',
        long = "extra",
        help = "extra prompt. append to default prompt"
    )]
    extra: Option<String>,
}

#[derive(Debug, clap::Subcommand, Clone)]
enum Commands {
    #[command(name = "commit", about = "gen git commit msg")]
    Commit(Commit),

    #[command(name = "readme", about = "gen README by codebase")]
    Readme(Readme),

    #[command(name = "diffsum", about = "summarize changes by git diff")]
    DiffSum(DiffSum),
}

impl RootOption for Commands {
    fn get_root_options(&self) -> RootOptions {
        match self {
            Commands::Commit(commit) => commit.get_root_options(),
            Commands::Readme(readme) => readme.get_root_options(),
            Commands::DiffSum(diff_sum) => diff_sum.get_root_options(),
        }
    }
}

#[derive(Debug, clap::Args, Clone)]
struct Commit {
    #[command(flatten)]
    root_options: RootOptions,

    #[arg(long = "auto-commit", help = "allow auto git commit")]
    auto_commit: bool,
}

#[derive(Debug, clap::Args, Clone)]
struct Readme {
    #[command(flatten)]
    root_options: RootOptions,

    #[arg(short = 's', long = "sources", help = "source files path list")]
    source_path_list: Option<String>,

    #[arg(short = 'd', long = "directory", help = "source folder")]
    source_dir: Option<String>,

    #[arg(long = "merge-readme", help = "allow to merge to `./README.md`")]
    allow_merge: bool,

    #[arg(long = "over-write", help = "allow to overwrite `./README.md`")]
    allow_over_write: bool,
}

#[derive(Debug, clap::Args, Clone)]
struct DiffSum {
    #[command(flatten)]
    root_options: RootOptions,
}

#[derive(Debug, clap::Args, Clone)]
struct Config {
    #[command(flatten)]
    root_options: RootOptions,

    #[arg(short = 'c', long = "check", help = "check config")]
    check: bool,

    #[arg(short = 's', long = "show", help = "show current config")]
    show: bool,
}

impl RootOption for Commit {
    fn get_root_options(&self) -> RootOptions {
        self.root_options.clone()
    }
}

impl RootOption for Readme {
    fn get_root_options(&self) -> RootOptions {
        self.root_options.clone()
    }
}

impl RootOption for DiffSum {
    fn get_root_options(&self) -> RootOptions {
        self.root_options.clone()
    }
}
