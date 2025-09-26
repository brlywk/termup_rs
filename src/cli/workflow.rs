use crate::config::Config;
use anyhow::Result;

pub fn workflow_command_run(config: &Config) -> Result<()> {
    println!("custom command called");

    Ok(())
}
