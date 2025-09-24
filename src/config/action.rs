use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{config::constants::*, pretty_list, print::*};

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
                styled_option(self.multi_run, |s| style_default(s), || style_none())
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
