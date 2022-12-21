use crate::values_representation::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StateRepresentation {
    pub values: ValuesRepresentation,
    pub terraform_version: String,
}
