use std::{collections::HashMap, fmt::Display, io::Write};

use crate::{
    cli::run::run_actions,
    config::{Action, Config, KEY_NOTES, Workflow, check_required_commands},
    print::{PadAlign, print_header},
};
use anyhow::{Result, anyhow, bail};
use colored::Colorize;

pub fn workflow_command_run(config: &Config, workflow_id: Option<&String>) -> Result<()> {
    // create workflow map b/c we:
    // a) need it as a simple lookup to check if a potential workflow_id exists
    // b) get a workflow for a specific workflow_id
    // c) use it to run all of our workflows
    let workflow_map: HashMap<&str, &Workflow> = config
        .workflows
        .iter()
        .map(|wf| (wf.id.as_str(), wf))
        .collect();

    // check if we got a workflow_id; if so try to find the correct workflow
    let workflows_to_run: Vec<&Workflow> = match workflow_id {
        Some(wf_id) => workflow_map
            .get(wf_id.as_str())
            .map(|wf| vec![*wf])
            .ok_or_else(|| anyhow!("no workflow found with id: {wf_id}"))?,

        None => config.workflows.iter().collect(),
    };

    // mapping of actions to action IDs for faster and more
    // secure lookup
    let action_map: HashMap<&str, &Action> = config
        .actions
        .iter()
        .map(|action| (action.id.as_str(), action))
        .collect();

    for wf in workflows_to_run {
        run_workflow(wf, &action_map)?;
    }

    Ok(())
}

fn run_workflow(workflow: &Workflow, action_map: &HashMap<&str, &Action>) -> Result<()> {
    println!();

    print_header(
        &workflow.name,
        |b| b.yellow().reversed(),
        |t| t.yellow().bold().reversed(),
    );

    // check if everything is set up to run this workflow
    check_required_commands(workflow.requires.as_ref())?;

    // print notes
    let notes_str = KEY_NOTES[0..KEY_NOTES.len() - 1].blue().bold();
    println!("[{notes_str}]");

    let note_lines: Vec<(String, Box<dyn Display>)> = workflow
        .notes
        .iter()
        .enumerate()
        .map(|(idx, note)| {
            let key = format!("  {}.", (idx + 1)).to_string();
            let value = Box::new(note.clone()) as Box<dyn Display>;

            (key, value)
        })
        .collect();

    println!("{}", note_lines.pad_align(2));
    println!();

    // show prompt to proceed?
    println!(
        "{} [{}/{}]",
        workflow.prompt.blue(),
        "yes".green(),
        "no".red()
    );
    // possible user abort:
    check_user_input(">")?;
    println!();

    // ...aaaand action!
    run_actions(&workflow.actions, action_map)?;

    Ok(())
}

/// Simple user input check that errors on anything but a "yes".
fn check_user_input(prompt: &str) -> Result<()> {
    print!("{prompt} ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();

    if response == "yes" || response == "y" {
        Ok(())
    } else {
        bail!("aborted by user")
    }
}
