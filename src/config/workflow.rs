use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    config::{
        style_default, style_none, vec_multiline_split, KEY_DESCRIPTION, KEY_ID, KEY_NAME,
        KEY_NOTES, KEY_PROMPT, KEY_REQUIRES, KEY_STEPS,
    },
    pretty_list,
    print::PadAlign,
};

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
                    .map_or(style_none(), |v| style_default(&v.join(", ")))
            ),
            (KEY_PROMPT, self.prompt.clone()),
            (KEY_STEPS, self.actions.join(", ")),
        ];

        // notes
        let formatted_notes = if self.notes.is_empty() {
            // add the default formatting for empty notes to our lines and move on...
            lines.push((KEY_NOTES, Box::new(style_none())));
            Vec::new()
        } else {
            vec_multiline_split(KEY_NOTES, &self.notes)
        };

        for (key, value) in &formatted_notes {
            lines.push((key, Box::new(value.to_string())));
        }

        writeln!(f, "{}", lines.pad_align_default())
    }
}
