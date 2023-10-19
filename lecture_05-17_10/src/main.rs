#![deny(unreachable_code, unreachable_patterns)]
// TODO! remove after homework is done
#![allow(unused)]

mod custom_errors;
mod helpers;

use custom_errors::AppError;
use helpers::*;
use std::{env, io, process};

fn main() -> Result<(), AppError> {
    // GETTING CLI ARG
    let cli_arg = env::args()
        .only_first_provided()
        .ok_or(AppError::MissingCliArgument)?; //.unwrap_or_else(||{eprintln!("{}",AppError::MissingCliArgument);process::exit(1);});

    // GETTING TEXT TRANSFORMER
    let text_transformer = TextTransformer::new(&cli_arg)?;

    Ok(())
}
