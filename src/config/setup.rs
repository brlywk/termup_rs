use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{config::constants::*, pretty_list, print::PadAlign};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Setup {
    #[serde(default)]
    pub pre: Vec<String>,

    #[serde(default)]
    pub main: Vec<String>,

    #[serde(default)]
    pub post: Vec<String>,
}

impl Display for Setup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = pretty_list![
            (KEY_PRE, self.pre.join(", ")),
            (KEY_MAIN, self.main.join(", ")),
            (KEY_POST, self.post.join(", "))
        ];

        writeln!(f, "{}", lines.pad_align_default())
    }
}
