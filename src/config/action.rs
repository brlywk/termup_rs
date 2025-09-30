use std::{
    fmt::Display,
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{bail, Context};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{
    config::{
        style_default, style_none, vec_multiline_split, KEY_ARGS, KEY_CMD, KEY_COND, KEY_CONTENT,
        KEY_ID, KEY_MULTI_RUN, KEY_NAME, KEY_REQUIRES, KEY_REQUIRES_DIR, KEY_REQUIRES_FILES,
        KEY_WORKING_DIR,
    },
    execute::Executable,
    pretty_list,
    print::{styled_option, PadAlign},
};

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
    pub requires_dir: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = pretty_list![
            (KEY_ID, self.id.clone()),
            (KEY_NAME, self.name.clone()),
            (KEY_CMD, self.cmd.clone()),
            (KEY_ARGS, self.args.join(", ")),
            (
                KEY_WORKING_DIR,
                styled_option(self.working_dir.clone(), style_default, style_none)
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
                styled_option(self.condition.clone(), style_default, style_none)
            ),
            (
                KEY_REQUIRES,
                self.requires
                    .as_ref()
                    .map_or(style_none(), |v| style_default(&v.join(", ")))
            ),
            (
                KEY_REQUIRES_FILES,
                self.requires_files
                    .as_ref()
                    .map_or(style_none(), |v| style_default(&v.join(", ")))
            ),
            (
                KEY_REQUIRES_DIR,
                styled_option(self.requires_dir.clone(), style_default, style_none)
            ),
        ];

        // content of files
        let formatted_content: Vec<(String, String)>;

        if let Some(content) = &self.content {
            let content_lines = content.lines().map(str::to_string).collect::<Vec<String>>();
            formatted_content = vec_multiline_split(KEY_CONTENT, &content_lines);
        } else {
            lines.push((KEY_CONTENT, Box::new(style_none())));
            formatted_content = Vec::new();
        }

        for (key, value) in &formatted_content {
            lines.push((key, Box::new(value.to_string())));
        }

        writeln!(f, "{}", lines.pad_align_default())
    }
}

impl Executable for Action {
    fn execute(&self) -> anyhow::Result<()> {
        println!("[{}] {}:", "Running Action".bright_cyan(), self.name.bold());

        // TODO:
        // - Besides a pre-condition, be also need to check `required`, `required_fils` and
        //   `required_dir` before doing anything
        // - Should `required` attempt any installation? should that be configurable via a flag?

        // before executing anything, we need to check if there are any pre-conditions
        // that need to be fulfilled
        if let Some(cond) = &self.condition {
            let mut cond_cmd = Command::new("sh");
            cond_cmd.arg("-c").arg(cond);

            // silence the pre-condition check (no output)
            cond_cmd.stdout(Stdio::null());
            cond_cmd.stderr(Stdio::null());

            let cond_status = cond_cmd
                .spawn()
                .with_context(|| format!("failed to spawn pre-condition check: {cond}"))?
                .wait()
                .with_context(|| "failed to wait for pre-condition check to finish")?;

            if !cond_status.success() {
                bail!("pre-condition check failed: {cond}")
            }
        }

        // collect and prepare all commands to actually run:
        // when multi_run is set to true, every arg should spawn it's own command
        let cmds_to_run: Vec<(String, Vec<String>)> = if self.multi_run.unwrap_or(false) {
            self.args
                .iter()
                .map(|arg| (self.cmd.clone(), vec![arg.clone()]))
                .collect()
        } else {
            vec![(self.cmd.clone(), self.args.clone())]
        };

        // execute each command
        for (cmd_name, cmd_args) in cmds_to_run {
            let cmd_str = format!("{} {}", cmd_name, cmd_args.join(" "));

            println!(
                "{} {}\n",
                "$".yellow().dimmed(),
                cmd_str.as_str().yellow().dimmed()
            );

            let mut cmd = Command::new(&cmd_name);
            cmd.args(&cmd_args);

            if let Some(dir) = &self.working_dir {
                cmd.current_dir(dir);
            }

            // create a new stdin pipe if there is some
            // content to write
            if self.content.is_some() {
                cmd.stdin(Stdio::piped());
            }

            // repipe the pipes
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());

            // create child command to write to stdin if necessary
            let mut child = cmd
                .spawn()
                .with_context(|| format!("failed to spawn command: {cmd_str}"))?;

            if let Some(content) = &self.content && let Some(mut stdin) = child.stdin.take() {
                    let cont = content.clone();
                    std::thread::spawn(move || {
                        stdin
                            .write_all(cont.as_bytes())
                            .expect("failed to write to stdin");
                    });
            }

            // run cmd, run!
            let status = child
                .wait()
                .with_context(|| "failed to wait for command to finish")?;

            if status.success() {
                println!("\n{} {}", "\u{2713}".bright_green().bold(), "Done".bright_green().bold());
            } else {
                bail!("Action '{}' failed with status: {}", &self.name, status)
            }

            println!();
        }

        Ok(())
    }
}
