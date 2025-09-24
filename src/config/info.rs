use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    config::constants::{KEY_NAME, KEY_VERSION},
    pretty_list,
    print::PadAlign,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub version: String,
}

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = pretty_list![
            (KEY_NAME, self.name.clone()),
            (KEY_VERSION, self.version.clone()),
        ];

        write!(f, "{}", lines.pad_align_default())
    }
}
