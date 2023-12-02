use std::fmt::Debug;
use std::num::ParseIntError;
use std::error::Error;
use thiserror::Error as ThisErr;
use reqwest;

#[derive(ThisErr, Debug)]
pub enum MyError {

    #[error("cannot be parsed: {0}")]
    NotParsable(#[from] ParseIntError)
}

pub fn try_parse(input: &str) -> Result<i32, MyError> {
    let parsed: i32 = input.parse()?;
    Ok(parsed)
}

// https://betterprogramming.pub/a-simple-guide-to-using-thiserror-crate-in-rust-eee6e442409b
#[derive(ThisErr)]
pub enum CustomError {
    #[error("failed to read the key")] // "Display" repr
    FileRead(#[source] std::io::Error), // #[source] -> root cause in Error::source

    #[error("failed to send API request")]
    Request(#[from] reqwest::Error), // #[from] -> impl From<request::Error> for CustomError

    #[error("failed to delete key file")]
    FileDelete(#[source] std::io::Error),

    #[error("len should be 10, but was {}", .0.len())]
    Validation(String),
}

impl Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}

pub fn make_request() -> Result<(), CustomError> {
    use CustomError::*;

    let key = std::fs::read_to_string("secret.key")
        .map_err(FileRead)?;

    _ = reqwest::blocking::get(format!("https://httpbin.org/key/{}", key))?
        .error_for_status()?;

    std::fs::remove_file(key).map_err(FileDelete)?;
    Ok(())
}

pub fn test_app_error() -> Result<(), AppErr> {
    use AppErr::*;
    let _: i32 = "1".parse()?;
    Err(InvalidNum { expected: 0, found: 10 })?;
    Ok(())
}

// https://docs.rs/thiserror/latest/thiserror/
#[derive(ThisErr, Debug)]
pub enum AppErr {

    #[error("Cannot parse '{0}' - invalid number")]
    NotParsable(#[from] ParseIntError),

    #[error("Invalid number (expected: {expected:?}, found: {found:?})")]
    InvalidNum { expected: i32, found: i32 },
}