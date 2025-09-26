use std::{
    fmt::Display,
    io::Stderr,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{config::constants::*, execute::Executable, pretty_list, print::*};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Action {
    pub id: String,
    pub name: String,

    pub cmd: String,
    pub args: Vec<String>,

    #[serde(default)]
    pub working_dir: Option<String>,
    #[serde(default)]
    pub multi_run: Option<bool>,
    #[serde(default)]
    pub condition: Option<String>,
    #[serde(default)]
    pub requires: Option<Vec<String>>,
    #[serde(default)]
    pub requires_files: Option<Vec<String>>,
    #[serde(default)]
    pub requires_dir: Option<Vec<String>>,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = pretty_list![
            (KEY_ID, self.id.clone()),
            (KEY_NAME, self.name.clone()),
            (KEY_CMD, self.cmd.clone()),
            (KEY_ARGS, self.args.join(", ")),
            (
                KEY_WORKING_DIR,
                styled_option(
                    self.working_dir.clone(),
                    |s| style_default(s),
                    || style_none()
                )
            ),
            (
                KEY_MULTI_RUN,
                if let Some(true) = self.multi_run {
                    "Yes"
                } else {
                    "No"
                }
            ),
            (
                KEY_COND,
                styled_option(
                    self.condition.clone(),
                    |s| style_default(s),
                    || style_none(),
                )
            ),
            (
                KEY_REQUIRES,
                self.requires
                    .as_ref()
                    .map(|v| style_default(&v.join(", ")))
                    .unwrap_or(style_none())
            ),
            (
                KEY_REQUIRES_FILES,
                self.requires_files
                    .as_ref()
                    .map(|v| style_default(&v.join(", ")))
                    .unwrap_or(style_none())
            ),
            (
                KEY_REQUIRES_DIR,
                self.requires_dir
                    .as_ref()
                    .map(|v| style_default(&v.join(", ")))
                    .unwrap_or(style_none())
            ),
        ];

        writeln!(f, "{}", lines.pad_align_default())
    }
}

impl Executable for Action {
    fn execute(&self) -> anyhow::Result<()> {
        println!(
            "[{}] {} ----",
            "Running Action".bright_cyan(),
            self.name.bold()
        );

        // collect and prepare all commands to actually run:
        // when multi_run is set to true, every arg should spawn it's own command
        let cmds_to_run: Vec<String> = if self.multi_run.unwrap_or(false) {
            self.args
                .iter()
                .map(|arg| format!("{} {}", self.cmd, arg))
                .collect()
        } else {
            let args = self.args.join(" ");
            vec![format!("{} {}", self.cmd, args)]
        };

        // execute each command
        for cmd_str in cmds_to_run {
            println!("{}", cmd_str.as_str().dimmed());

            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(&cmd_str);

            if let Some(dir) = &self.working_dir {
                cmd.current_dir(dir);
            }

            // repipe the pipes
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());

            // run cmd, run!
            let status = cmd
                .spawn()
                .with_context(|| format!("failed to spawn command: {}", cmd_str))?
                .wait()
                .with_context(|| "failed to wait for command to finish")?;

            if !status.success() {
                bail!("Action {} failed with status: {}", &self.name, status)
            }
        }

        Ok(())
    }
}
