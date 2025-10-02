use std::{
    fmt::Display,
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{Context, Result, bail};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{
    config::{
        KEY_ARGS, KEY_CMD, KEY_CONTENT, KEY_ID, KEY_MULTI_RUN, KEY_NAME, KEY_REQUIRES,
        KEY_REQUIRES_DIR, KEY_REQUIRES_FILES, KEY_WORKING_DIR, style_default, style_none,
        vec_multiline_split,
    },
    execute::Executable,
    pretty_list,
    print::{PadAlign, styled_option},
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
        // - Should `required` attempt any installation? should that be configurable via a flag?

        // we also need to check some potential requirements
        check_required_commands(self.requires.as_ref())?;
        check_required_files(self.requires_files.as_ref())?;
        check_required_dir(self.requires_dir.as_ref())?;

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

            if let Some(content) = &self.content
                && let Some(mut stdin) = child.stdin.take()
            {
                let cont = content.clone();
                let handle = std::thread::spawn(move || stdin.write_all(cont.as_bytes()));

                handle
                    .join()
                    .unwrap()
                    .with_context(|| "failed to write to stdin")?;
            }

            // run cmd, run!
            let status = child
                .wait()
                .with_context(|| "failed to wait for command to finish")?;

            if status.success() {
                println!(
                    "\n{} {}",
                    "\u{2713}".bright_green().bold(),
                    "Done".bright_green().bold()
                );
            } else {
                bail!("Action '{}' failed with status: {}", &self.name, status)
            }

            println!();
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum ExecutionMode {
    Direct,
    Subshell,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum OutputMode {
    Inherit,
    Mute,
}

/// Run the given `command` with arguments `args`. `mode` determines whether the `command`
/// is run directly or in a subshell (`sh -c`) to enable shell substitutions (e.g. `~/`
/// for the users home directory).
///
/// `output` can be set to redirect the output of the `command` to the parents pipe, or
/// mute the output.
fn run_shell_command(
    command: &str,
    args: &[&str],
    mode: ExecutionMode,
    output: OutputMode,
    working_dir: Option<&str>,
) -> Result<()> {
    let mut cmd = match mode {
        ExecutionMode::Subshell => {
            let sub_cmd = format!("{} {}", command, args.join(" "));
            let mut c = Command::new("sh");
            c.arg("-c").arg(sub_cmd);
            c
        }
        ExecutionMode::Direct => {
            let mut c = Command::new(command);
            c.args(args);
            c
        }
    };

    let mute = matches!(output, OutputMode::Mute);
    cmd.stdout(if mute { Stdio::null() } else { Stdio::piped() });
    cmd.stderr(if mute { Stdio::null() } else { Stdio::piped() });

    if let Some(work_dir) = working_dir {
        cmd.current_dir(work_dir);
    }

    let printable_cmd = format!("{} {}", command, args.join(" "));

    let cmd_status = cmd
        .spawn()
        .with_context(|| format!("failed to spawn command: {printable_cmd}"))?
        .wait()
        .with_context(|| format!("failed to wait for command: {printable_cmd}"))?;

    if !cmd_status.success() {
        bail!("failed: {printable_cmd}")
    }

    Ok(())
}

/// Checks if all `required_commands` (read: programs, apps) are actually available.
/// Fails for the first command not found.
pub fn check_required_commands(required_commands: Option<&Vec<String>>) -> Result<()> {
    let Some(commands) = required_commands else {
        return Ok(());
    };

    for cmd in commands {
        run_shell_command(
            "command",
            &["-v", cmd],
            ExecutionMode::Direct,
            OutputMode::Mute,
            None,
        )
        .with_context(|| format!("command '{cmd}' not found"))?;
    }

    Ok(())
}

fn check_required_files(required_files: Option<&Vec<String>>) -> Result<()> {
    let Some(files) = required_files else {
        return Ok(());
    };

    for file in files {
        run_shell_command(
            "test",
            &["-f", file],
            ExecutionMode::Subshell,
            OutputMode::Mute,
            None,
        )
        .with_context(|| format!("required file not found: {file}"))?;
    }

    Ok(())
}

fn check_required_dir(required_dir: Option<&String>) -> Result<()> {
    let Some(dir) = required_dir else {
        return Ok(());
    };

    run_shell_command(
        "test",
        &["-d", dir],
        ExecutionMode::Subshell,
        OutputMode::Mute,
        None,
    )
    .with_context(|| format!("required directory not found: {dir}"))?;

    Ok(())
}
