use std::error::Error;
use std::fmt::Display;

use json::JsonItem;
use parser::json_parser;

pub mod json;
pub mod parser;

pub fn json_parse(input: &str) -> Result<JsonItem, JsonParseError> {
    match json_parser(input) {
        Ok((_, json)) => Ok(json),
        Err(_) => Err(JsonParseError {}),
    }
}

#[derive(Debug)]
pub struct JsonParseError {}

impl Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot pass this json")
    }
}

impl Error for JsonParseError {}
