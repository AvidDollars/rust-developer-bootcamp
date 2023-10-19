//! Module containing helper methods
//!
//! [`OnlyFirstCliArg`] extends [`std::env::Args`] trait.
//!
//! [`OnlyFirstCliArg::only_first_provided`] method skips 1st argument (path of the executable),
//! and returns Option of the 1st argument provided from an user. If any other user's arguments are provided, they will be ignored.
//!
//! [`TextTransformer`] trait used as text transformation tool.
//!
//! [`transform_stdin_with`] function takes input from stdin and transform is based on an instance of [`TextTransformer`].

use crate::csv_processing::*;
use crate::custom_errors::AppError;
use slug::slugify;
use std::collections::HashMap;
use std::error::Error;
use std::io;

type ArgFnMap = HashMap<&'static str, fn(String) -> String>;

pub trait OnlyFirstCliArg: Iterator {
    fn only_first_provided(self) -> Option<String>
    where
        Self: Iterator<Item = String>,
        Self: Sized,
    {
        self.skip(1).nth(0)
    }
}

impl OnlyFirstCliArg for std::env::Args {}

pub struct TextTransformer {
    method: fn(String) -> String,
}

impl TextTransformer {
    pub fn new(cli_arg: &str) -> Result<Self, AppError> {
        let mapping = Self::get_mapping();
        let maybe_func = mapping.get(cli_arg);

        match maybe_func {
            Some(func) => Ok(Self { method: *func }),
            None => {
                let allowed: Vec<&str> = mapping.keys().copied().collect();
                let message = format!(
                    "Invalid argument: '{}'\nAllowed arguments: {}",
                    cli_arg,
                    allowed.join(" | ")
                );
                Err(AppError::InvalidCliArgument { message })
            }
        }
    }

    pub fn apply(&self, text: String, output: &mut dyn io::Write) -> Result<(), impl Error> {
        let transformed = (self.method)(text);
        writeln!(output, "{}", transformed)
    }

    fn get_mapping() -> ArgFnMap {
        let mut mapping: ArgFnMap = HashMap::new();
        mapping.insert("lowercase", |arg| arg.to_lowercase());
        mapping.insert("uppercase", |arg| arg.to_uppercase());
        mapping.insert("no-spaces", |arg| arg.replace(" ", ""));
        mapping.insert("slugify", |arg| slugify(arg));
        mapping.insert("scream", |arg| format!("{}!!!", arg));
        mapping.insert("reverse", |arg| arg.chars().rev().collect::<String>());
        mapping.insert("csv", as_csv);
        mapping
    }
}

pub fn transform_stdin_with(
    transformer: TextTransformer,
    output_stream: &mut dyn io::Write,
) -> Result<(), AppError> {
    for line in io::stdin().lines() {
        let line = line.map_err(|error| AppError::OnOutput(error.to_string()))?;

        transformer
            .apply(line, output_stream)
            .map_err(|error| AppError::OnOutput(error.to_string()))?;
    }

    Ok(())
}
