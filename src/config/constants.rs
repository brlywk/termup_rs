use colored::{ColoredString, Colorize};

// Config Info
pub const KEY_PRE: &str = "Pre-Setup Actions:";
pub const KEY_MAIN: &str = "Main Setup Actions:";
pub const KEY_POST: &str = "Post-Setup Actions:";
pub const KEY_ID: &str = "Id:";
pub const KEY_NAME: &str = "Name:";
pub const KEY_VERSION: &str = "Version:";
pub const KEY_CMD: &str = "Command:";
pub const KEY_ARGS: &str = "Arguments:";
pub const KEY_COND: &str = "Run Conditions:";
pub const KEY_MULTI_RUN: &str = "Run for each Arg:";
pub const KEY_WORKING_DIR: &str = "Working Dir";
pub const KEY_REQUIRES: &str = "Requires Flask(s):";
pub const KEY_REQUIRES_FILES: &str = "Requires File(s):";
pub const KEY_REQUIRES_DIR: &str = "Requires Dir(s):";
pub const KEY_DESCRIPTION: &str = "Descripton:";
pub const KEY_NOTES: &str = "Notes:";
pub const KEY_PROMPT: &str = "Prompt:";
pub const KEY_STEPS: &str = "Steps:";

// Default styling

pub fn style_none() -> ColoredString {
    "None".dimmed()
}

pub fn style_default(s: &str) -> ColoredString {
    ColoredString::from(s)
}

/// Turn a `Vec<String>` into a list of `&str` tuples for easier printing.
/// Uses the `key` as the first element of the first tuple, while all other
/// elements will have an empty first element.
///
/// # Example
/// Turns `["one", "two", "three"]` with key "KEY" into:
/// `[("KEY", "one"), ("", "two"), ("", "three")]`
pub fn vec_multiline_split<'a>(key: &'a str, vec: &'a [String]) -> Vec<(&'a str, &'a str)> {
    if let Some((first, rest)) = vec.split_first() {
        let first_line = std::iter::once((key, first.as_str()));
        let rest_lines = rest.iter().map(|line| ("", line.as_str()));

        first_line.chain(rest_lines).collect()
    } else {
        Vec::new()
    }
}
