use std::{collections::HashMap, fmt::Display};

pub struct Response {
    pub meta: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct ResponseError {
    pub meta: HashMap<String, String>,
    pub message: String,
}

impl core::error::Error for ResponseError {}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}
Meta = {:?}",
            self.message, self.meta
        )
    }
}
