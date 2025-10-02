use crate::{
    cli::{
        Cli, Commands, config::config_command_run, run::run_command_run,
        workflow::workflow_command_run,
    },
    config::Config,
};
use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod execute;
mod print;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_content = std::fs::read_to_string(&cli.config_file)?;
    let config: Config = toml::from_str(&config_content)?;

    match cli.command {
        Commands::Run => run_command_run(&config)?,
        Commands::Workflow { workflow_id } => workflow_command_run(&config, workflow_id.as_ref())?,
        Commands::Config => config_command_run(&config),
    }

    Ok(())
}
