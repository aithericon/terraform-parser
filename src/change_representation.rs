use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ChangeRepresentation {
    pub actions: Vec<Action>,
    #[serde(default)]
    pub before: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub after: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub after_unknown: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub before_sensitive: bool,
    #[serde(default)]
    pub after_sensitive: HashMap<String, serde_json::Value>,
    pub replace_paths: Option<Vec<Vec<String>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Action {
    NoOp,
    Create,
    Read,
    Update,
    Delete,
}
