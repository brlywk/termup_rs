use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{config::constants::*, pretty_list, print::PadAlign};

#[derive(Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prompt: String,

    #[serde(default)]
    pub requires: Option<Vec<String>>,
    #[serde(default)]
    pub notes: Vec<String>,

    pub actions: Vec<String>,
}

impl Display for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = pretty_list![
            (KEY_ID, self.id.clone()),
            (KEY_NAME, self.name.clone()),
            (KEY_DESCRIPTION, self.description.clone()),
            (
                KEY_REQUIRES,
                self.requires
                    .as_ref()
                    .map(|v| style_default(&v.join(", ")))
                    .unwrap_or(style_none())
            ),
            (KEY_PROMPT, self.prompt.clone()),
            (KEY_STEPS, self.actions.join(", ")),
        ];

        // notes
        if !self.notes.is_empty() {
            for (key, value) in vec_multiline_split(KEY_NOTES, &self.notes) {
                lines.push((key, Box::new(value.to_string())));
            }
        } else {
            lines.push((KEY_NOTES, Box::new(style_none())));
        }

        writeln!(f, "{}", lines.pad_align_default())
    }
}
