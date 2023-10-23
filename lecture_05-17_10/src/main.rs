#![deny(unreachable_code, unreachable_patterns, unused)]

pub mod csv_processing;
pub mod custom_errors;
pub mod helpers;

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
