use std::error::Error as StdError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ErrorType {
    Parsing,
    Header,
}

#[derive(Debug)]
pub struct DemoError {
    error_type: ErrorType,
    description: String,
}

impl Display for DemoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "error: {:#?}\nDescription: {}",
            self.error_type, self.description
        )
    }
}

impl StdError for DemoError {}

impl DemoError {
    pub fn new<T: ToString>(error_type: ErrorType, description: T) -> DemoError {
        DemoError {
            error_type,
            description: description.to_string(),
        }
    }
}
