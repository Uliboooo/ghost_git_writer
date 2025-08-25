use std::{fmt::Display, fs, path::PathBuf};

use clap::{self};
use derive_getters::Getters;

#[derive(Debug)]
pub enum Error {
    // InvalidFormatBaseUrl,
    // InvalidPortAsBaseUrl,
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Error::InvalidFormatBaseUrl => write!(f, "invalid format base url"),
            // Error::InvalidPortAsBaseUrl => write!(f, "invalid port as base url"),
            Error::Io(e) => write!(f, "io error: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, clap::Parser, Clone)]
#[command(name = "ggw", version, about = "generate git commit msg by llm")]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Commands,
}

impl RootOption for Cli {
    fn get_root_options(&self) -> RootOptions {
        self.subcommand.get_root_options()
    }
}

pub trait RootOption {
    fn get_root_options(&self) -> RootOptions;
}

#[derive(Debug, clap::Args, Clone, Getters)]
pub struct RootOptions {
    #[arg(
        short = 'm',
        long = "model",
        // conflicts_with = "alias",
        help = "`-m gemini/gemino-2.0-flash` or `-m config's model name`"
    )]
    model: Option<String>,

    #[arg(long = "temperature")]
    temperature: Option<f32>,

    #[arg(long = "max-tokens")]
    max_tokens: Option<u32>,

    #[arg(long = "base-url")]
    base_url: Option<String>,

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

    #[arg(long = "config", help = "config file path")]
    config_path: Option<String>,
}

// impl RootOptions {
//     pub fn parse_base_url(&self) -> Result<Option<(String, u16)>, Error> {
//         match self.clone().base_url {
//             Some(v) => match v.split_once('/') {
//                 Some(vv) => {
//                     let port =
//                         vv.1.parse::<u16>()
//                             .map_err(|_| Error::InvalidPortAsBaseUrl)?;
//                     Ok(Some((vv.0.to_string(), port)))
//                 }
//                 None => Err(Error::InvalidFormatBaseUrl),
//             },
//             None => Ok(None),
//         }
//     }
// }

#[derive(Debug, clap::Subcommand, Clone)]
pub enum Commands {
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

#[derive(Debug, clap::Args, Clone, Getters)]
pub struct Commit {
    #[command(flatten)]
    root_options: RootOptions,

    #[arg(long = "auto-commit", help = "allow auto git commit")]
    auto_commit: bool,
}

#[derive(Debug, clap::Args, Clone, Getters)]
pub struct Readme {
    #[command(flatten)]
    root_options: RootOptions,

    #[arg(short = 's', long = "sources", help = "source files path list")]
    source_path: Option<String>,

    #[arg(short = 'd', long = "directory", help = "source folder")]
    source_dir: Option<String>,

    #[arg(long = "merge-readme", help = "allow to merge to `./README.md`")]
    allow_merge: bool,
    // #[arg(long = "over-write", help = "allow to overwrite `./README.md`")]
    // allow_over_write: bool,
}

impl Readme {
    pub fn export_path_list(&self) -> Option<Vec<PathBuf>> {
        let mut list = Vec::new();
        if let Some(p) = &self.source_path {
            list.push(PathBuf::from(p));
        }
        if let Some(l) = &self.source_dir {
            let path = PathBuf::from(l);
            let path_list = fs::read_dir(path).ok()?;
            for i in path_list {
                let i = i.unwrap().path();
                list.push(i);
            }
        }

        Some(list)
    }
}

#[derive(Debug, clap::Args, Clone, Getters)]
pub struct DiffSum {
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
