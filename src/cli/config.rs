use crate::config::Config;
use anyhow::Result;

pub fn config_command_run(config: &Config) -> Result<()> {
    println!("{}", config);

    Ok(())
}
