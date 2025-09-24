use std::collections::HashMap;

use crate::config::Config;
use anyhow::Result;
use colored::Colorize;

pub fn run_command_run(config: &Config) -> Result<()> {
    println!(
        "{}",
        "[==== TERMUP: Run setup steps ====]\n"
            .bold()
            .bright_blue()
            .reversed()
    );

    // print config info
    println!("{}", &config.info);

    // create step map to map steps :P
    let action_map: HashMap<_, _> = config
        .actions
        .iter()
        .map(|action| (action.name.as_str(), action))
        .collect();

    println!("{:?}", action_map);

    Ok(())
}
