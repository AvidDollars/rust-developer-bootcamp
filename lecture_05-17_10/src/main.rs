#![deny(unreachable_code, unreachable_patterns, unused)]

mod csv_processing;
mod custom_errors;
mod helpers;

use custom_errors::AppError;
use helpers::*;
use std::{env, io};

fn main() -> Result<(), AppError> {
    let cli_arg = env::args()
        .only_first_provided()
        .ok_or(AppError::MissingCliArgument)?;

    let transformer = TextTransformer::new(&cli_arg)?;
    let output = &mut io::stdout();
    transform_stdin_with(transformer, output)?;

    Ok(())
}
