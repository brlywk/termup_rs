use std::collections::HashMap;

use crate::{
    config::{Action, Config},
    execute::Executable,
    print::{print_header, SEPARATOR_COUNT, SEPARATOR_DASH},
};
use anyhow::{bail, Result};
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
    run_actions("Pre-Setup steps", &config.setup.pre, &action_map)?;
    run_actions("Main Setup steps", &config.setup.main, &action_map)?;
    run_actions("Post-Setup steps", &config.setup.post, &action_map)?;

    Ok(())
}

fn run_actions(
    header_text: &str,
    actions: &[String],
    action_map: &HashMap<&str, &Action>,
) -> Result<()> {
    print_header(
        header_text,
        |b| b.yellow().reversed(),
        |t| t.yellow().bold().reversed(),
    );

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
