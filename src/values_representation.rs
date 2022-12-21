use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ValuesRepresentation {
    #[serde(default)]
    pub outputs: HashMap<String, Output>,
    // #[serde(deserialize_with = "deserialize_field")]
    pub root_module: Option<Module>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub value: serde_json::Value,
    pub sensitive: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    /// `None` for the root module
    pub address: Option<String>,
    pub resources: Option<Vec<Resource>>,
    pub child_modules: Option<Vec<Module>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub address: String,
    pub mode: Mode,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    pub index: Option<serde_json::Value>,
    pub provider_name: String,
    pub schema_version: i8,
    pub values: HashMap<String, serde_json::Value>,
    pub sensitive_values: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Managed,
    Data,
}
