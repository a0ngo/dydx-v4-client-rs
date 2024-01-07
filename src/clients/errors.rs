use std::{error::Error, fmt};

#[derive(Debug)]
pub(crate) struct ConstructorError {
    message: String,
}

impl ConstructorError {
    pub fn new(message: String) -> Self {
        ConstructorError { message }
    }
}

impl fmt::Display for ConstructorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ConstructorError {}

#[derive(Debug)]
pub struct APIError {
    message: String,
}

impl APIError {
    pub(crate) fn new(message: String) -> Self {
        APIError { message }
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for APIError {}
