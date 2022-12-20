use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ChangeRepresentation {
    pub actions: Vec<Action>,
    #[serde(default)]
    pub before: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub after: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub after_unknown: Option<HashMap<String, serde_json::Value>>,
    #[serde(deserialize_with="deserialize_field")]
    pub before_sensitive: Field,
    #[serde(deserialize_with="deserialize_field")]
    pub after_sensitive: Field,
    pub replace_paths: Option<Vec<Vec<String>>>,
}

#[derive(Serialize)]
pub enum Field {
    Bool(bool),
    Map(HashMap<String, serde_json::Value>),
}

fn deserialize_field<'de, D>(deserializer: D) -> Result<Field, D::Error>
    where
        D: Deserializer<'de>,
{
    let value = <serde_json::Value>::deserialize(deserializer)?;
    match value {
        serde_json::Value::Bool(b) => Ok(Field::Bool(b)),
        serde_json::Value::Object(object) =>  {
            let mut map = HashMap::new();
            for (key, value) in object.into_iter() {
                map.insert(key, value);
            }
            Ok(Field::Map(map))
        },
        _ => Err(serde::de::Error::custom("unexpected type for field")),
    }
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
