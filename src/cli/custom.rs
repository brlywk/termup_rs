use crate::config::Config;
use anyhow::Result;

pub fn custom_command_run(config: &Config) -> Result<()> {
    println!("custom command called");

    Ok(())
}
