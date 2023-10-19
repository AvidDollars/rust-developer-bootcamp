#![deny(unreachable_code, unreachable_patterns)]
// TODO! remove after homework is done
#![allow(unused)]

mod custom_errors;
mod helpers;

use custom_errors::AppError;
use helpers::*;
use std::error::Error;
use std::{env, io, process};

fn main() -> Result<(), AppError> {
    let cli_arg = env::args()
        .only_first_provided()
        .ok_or(AppError::MissingCliArgument)?;

    let transformer = TextTransformer::new(&cli_arg)?;
    transform_stdin_with(transformer, &mut io::stderr())?;

    Ok(())
}
