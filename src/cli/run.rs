use std::collections::HashMap;

use crate::{config::Config, execute::Executable};
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

    let action_map: HashMap<_, _> = config
        .actions
        .iter()
        .map(|action| (action.id.as_str(), action))
        .collect();

    // println!("{:?}", action_map);

    // run setup block: pre, main and post actions
    for action_id in config.setup.pre.iter() {
        let Some(&action) = action_map.get(action_id.as_str()) else {
            continue;
        };

        // TODO: Make it look nicer and actually... I don't know something I can't
        // remember right now :D
        action.execute()?;
    }

    Ok(())
}
