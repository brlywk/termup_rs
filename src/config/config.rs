use std::fmt::Display;

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::config::{action::Action, info::Info, setup::Setup, workflow::Workflow};

// TODO: Change this to the new structure otherwise this will super break!

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub info: Info,

    #[serde(default)]
    pub setup: Setup,

    #[serde(default, rename = "action")]
    pub actions: Vec<Action>,

    #[serde(default, rename = "workflow")]
    pub workflows: Vec<Workflow>,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // title / header
        writeln!(
            f,
            "{}\n",
            " Configuration ".bold().bright_green().reversed()
        )?;

        // config info
        writeln!(f, "{}\n{}\n", "Info".blue(), self.info)?;

        // setup block
        writeln!(f, "{}\n{}\n", "Setup".blue(), self.setup)?;

        // actions
        if !self.actions.is_empty() {
            writeln!(f, "{}", "Actions".blue())?;
            for s in self.actions.iter() {
                writeln!(f, "{}", s)?;
            }
            writeln!(f)?;
        }

        // workflows
        if !self.workflows.is_empty() {
            writeln!(f, "{}", "Manual Workflows".blue())?;
            for p in self.workflows.iter() {
                writeln!(f, "{}", p)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
