use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub info: Info,
    #[serde(default)]
    pub pre: Vec<PreStep>,
    #[serde(default)]
    pub step: Vec<Step>,
    #[serde(default)]
    pub post: Vec<PostStep>,
    #[serde(default)]
    pub custom: Vec<CustomStep>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreStep {
    pub name: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub condition: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Step {
    pub name: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub multi_run: Option<bool>,
    pub working_dir: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostStep {
    pub name: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub requires: Option<Vec<String>>,
    pub requires_files: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomStep {
    pub name: String,
    pub description: String,
    pub requires: Vec<String>,
    pub notes: Vec<String>,
    pub prompt: String,
    pub steps: Vec<String>,
}
