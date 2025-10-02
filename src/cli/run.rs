use std::collections::HashMap;

use crate::{
    config::{Action, Config},
    execute::Executable,
    print::{SEPARATOR_COUNT, SEPARATOR_DASH, print_header},
};
use anyhow::{Result, bail};
use colored::Colorize;

pub fn run_command_run(config: &Config) -> Result<()> {
    print_header(
        "TermUp: Run setup steps",
        |b| b.bright_blue().reversed(),
        |t| t.bold().bright_blue().reversed(),
    );

    // print setup info
    println!("{}\n", &config.setup);

    // mapping of actions to action IDs for faster and more
    // secure lookup
    let action_map: HashMap<&str, &Action> = config
        .actions
        .iter()
        .map(|action| (action.id.as_str(), action))
        .collect();

    // run setup block: pre, main and post actions
    print_header(
        "Pre-Setup steps",
        |b| b.yellow().reversed(),
        |t| t.yellow().bold().reversed(),
    );
    run_actions(&config.setup.pre, &action_map)?;

    print_header(
        "Main Setup steps",
        |b| b.yellow().reversed(),
        |t| t.yellow().bold().reversed(),
    );
    run_actions(&config.setup.main, &action_map)?;

    print_header(
        "Post-Setup steps",
        |b| b.yellow().reversed(),
        |t| t.yellow().bold().reversed(),
    );
    run_actions(&config.setup.post, &action_map)?;

    Ok(())
}

pub fn run_actions(actions: &[String], action_map: &HashMap<&str, &Action>) -> Result<()> {
    let mut actions_iter = actions.iter().peekable();

    while let Some(action_id) = actions_iter.next() {
        let Some(&action) = action_map.get(action_id.as_str()) else {
            bail!("Action with ID '{action_id}' not found in config file!");
        };

        action.execute()?;

        if actions_iter.peek().is_some() {
            println!("{}\n", SEPARATOR_DASH.repeat(SEPARATOR_COUNT));
        }
    }

    println!();

    Ok(())
}
