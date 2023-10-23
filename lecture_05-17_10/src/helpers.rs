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

type ArgFnMap = HashMap<&'static str, fn(&str) -> String>;

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

pub struct TextTransformer<'a> {
    method: fn(&str) -> String,
    pub method_name: &'a str,
}

impl<'a> TextTransformer<'a> {
    pub fn new(method_name: &'a str) -> Result<Self, AppError> {
        let mapping = Self::get_mapping();
        let maybe_func = mapping.get(method_name);

        match maybe_func {
            Some(func) => Ok(Self {
                method: *func,
                method_name,
            }),
            None => {
                let allowed: Vec<&str> = mapping.keys().copied().collect();
                let message = format!(
                    "Invalid argument: '{}'\nAllowed arguments: {}",
                    method_name,
                    allowed.join(" | ")
                );
                Err(AppError::InvalidCliArgument { message })
            }
        }
    }

    pub fn apply(&self, text: &str, output: &mut dyn io::Write) -> Result<(), impl Error> {
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
        mapping.insert("csv", |arg| arg.into()); // processing done in 'CsvTable' struct
        mapping
    }
}

pub fn transform_stdin_with(
    transformer: TextTransformer,
    mut output_stream: &mut dyn io::Write,
) -> Result<(), AppError> {
    // tried to init CsvTable with usafe code, so that it would initialize only if "csv" arg is provided,
    // but then started getting "Segmentation fault" error... too early to mess up with usafe code
    let mut csv_table = CsvTable::new();

    let output: &mut dyn io::Write = match transformer.method_name {
        "csv" => {
            println!("\
                Provide fields delimited by ','. \
                First row is CSV header. Every next row must have exact amount of fields specified by the header. \
                Type 'exit' for yielding CSV table.");
            &mut csv_table
        }
        _ => {
            println!("Provide your input. Type 'exit' for exiting the program.");
            &mut output_stream
        }
    };

    for line in io::stdin().lines() {
        let line = line.map_err(|error| AppError::OnOutput(error.to_string()))?;

        if line == "exit" {
            if transformer.method_name == "csv" {
                csv_table
                    .output_to(output_stream)
                    .map_err(|error| AppError::OnOutput(error.to_string()))?;
            }
            break;
        } else {
            transformer
                .apply(line.trim(), output)
                .map_err(|error| AppError::OnOutput(error.to_string()))?;
        }
    }

    Ok(())
}
