use crate::csv_processing;
use crate::custom_errors::*;
use crate::extensions::*;

use slug::slugify;

use std::io::{self, Error as IoError};
use std::{collections::HashMap, env, fs};

#[derive(Clone, Copy, Debug)]
pub struct TextTransformer {
    func: fn(&str) -> String,
    name: &'static str,
}

#[derive(Debug)]
pub struct Command {
    transformer: TextTransformer,
    input: String,
}

impl Command {
    pub fn new(transformer: TextTransformer, input: String) -> Result<Self, AppError> {
        let input = match transformer.name == "csv" {
            true => {
                let content = fs::read_to_string(&input).map_err(|error| AppError::FileRead {
                    message: format!("Unable to read '{}'. {error}", input),
                })?;
                content
            }
            false => input,
        };

        Ok(Self { transformer, input })
    }

    pub fn apply(&self, output: &mut dyn io::Write) -> Result<(), IoError> {
        let transformed_string = (self.transformer.func)(&self.input);
        writeln!(output, "{}", transformed_string)?;
        Ok(())
    }
}

impl TryFrom<String> for Command {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let transformer;
        let mut input;

        match env::args().only_first_provided() {
            // env arg provided
            Some(env_arg) => {
                transformer = get_text_transformer(env_arg.trim())?;
                input = value.trim().into();
            }

            // no env arg provided
            None => {
                let mut splitted = value.split(' ');
                let command = splitted.next().ok_or_else(|| AppError::MissingCommand)?;

                transformer = get_text_transformer(&command)?;

                input = splitted
                    .next()
                    .ok_or_else(|| AppError::MissingArgument)?
                    .into();
            }
        }

        if transformer.name == "csv" {
            input = fs::read_to_string(&input).map_err(|error| AppError::FileRead {
                message: format!("Unable to read '{}'. {error}", input),
            })?;
        }

        Ok(Self { transformer, input })
    }
}

pub fn get_text_transformer(name: &str) -> Result<TextTransformer, AppError> {
    let mut mapping = HashMap::new();

    mapping.insert(
        "lowercase",
        TextTransformer {
            name: "lowercase",
            func: |arg| arg.to_lowercase(),
        },
    );
    mapping.insert(
        "uppercase",
        TextTransformer {
            name: "uppercase",
            func: |arg| arg.to_uppercase(),
        },
    );
    mapping.insert(
        "no-spaces",
        TextTransformer {
            name: "no-spaces",
            func: |arg| arg.replace(" ", ""),
        },
    );
    mapping.insert(
        "slugify",
        TextTransformer {
            name: "slugify",
            func: |arg| slugify(arg),
        },
    );
    mapping.insert(
        "scream",
        TextTransformer {
            name: "scream",
            func: |arg| format!("{}!!!", arg),
        },
    );
    mapping.insert(
        "reverse",
        TextTransformer {
            name: "reverse",
            func: |arg| arg.chars().rev().collect::<String>(),
        },
    );
    mapping.insert(
        "csv",
        TextTransformer {
            name: "csv",
            func: csv_processing::create_csv_table,
        },
    );

    mapping.get(name).copied().ok_or_else(|| {
        let allowed = mapping.keys().copied().collect::<Vec<_>>().join(" | ");
        let message = format!("Invalid argument '{}'. Allowed are: {}", name, allowed);
        AppError::InvalidCliArgument { message }
    })
}
