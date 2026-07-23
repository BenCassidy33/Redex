use clap::{Parser, ValueEnum};
use derive_more::{Display, IsVariant};
use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct CLIArgs {
    #[arg(required = true)]
    pub(crate) files: Vec<PathBuf>,

    #[arg(short, long, conflicts_with = "outdir")]
    pub(crate) outfile: Option<PathBuf>,

    #[arg(long)]
    pub(crate) outdir: Option<PathBuf>,

    #[arg(short, long, default_value_t = false)]
    pub(crate) nostdlib: bool,

    #[arg(short, long, default_value_t = false, conflicts_with_all = ["outfile", "outdir"])]
    pub(crate) stdout: bool,

    #[arg(short, long, default_value_t = false)]
    pub(crate) interactive: bool,

    #[arg(short, long, default_value_t = LogLevel::All)]
    pub(crate) loglevel: LogLevel,
}

/// Sets the log level and what to output, each level has its own capabilites and the capabilites of all the
/// options **above** it
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, ValueEnum, IsVariant)]
pub enum LogLevel {
    ReductionsOnly,
    AssignmentReplacements,
    Assignment,
    All,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}
