use serde::{Deserialize, Serialize};
use crate::structure::field::Field;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub name: String,
    pub code: String,
    pub fields: Vec<Field>,
}

impl Input {
    pub fn new(name: &str, code: &str, fields: Vec<Field>) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            fields,
        }
    }
}