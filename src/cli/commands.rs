use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "termup")]
#[command(about = "termup - the TERMinal setUP tool.")]
#[command(long_about = r#"termup - The definitely maybe helpful TERMinal setUP helper."#)]
#[command(version = "0.0.1")]
pub struct Cli {
    #[arg(
        short = 'c',
        long = "config",
        default_value = "config.toml",
        help = "Config file to parse.",
        global = true
    )]
    pub config_file: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run terminal setup
    Run,
    /// Run custom / semi-manual setup steps
    Custom,
    /// Pretty-print loaded config file
    Config,
}
