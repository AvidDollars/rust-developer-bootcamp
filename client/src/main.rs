#![deny(unused, unreachable_code, unreachable_patterns)]

mod app;
mod utils;

use crate::utils::create_missing_folders;

use shared::constants::{FILES_FOLDER, IMAGES_FOLDER};
use shared::env_args::EnvArgs;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let env_args = EnvArgs::new()?;
    create_missing_folders(&[FILES_FOLDER, IMAGES_FOLDER])?;
    app::run(env_args)?;
    Ok(())
}
