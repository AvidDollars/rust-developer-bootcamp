#![deny(unused, unreachable_code, unreachable_patterns)]

use std::error::Error;

pub mod client;
pub mod config;
pub mod env_args;
pub mod helpers;
pub mod message;
pub mod server;

use config::*;
use env_args::*;

fn main() -> Result<(), Box<dyn Error>> {
    pick_mode()?;
    Ok(())
}

fn pick_mode() -> Result<(), Box<dyn Error>> {
    let env_args = EnvArgs::new()?;
    if env_args.is_server() {
        server::run(env_args)?;
    } else {
        helpers::create_missing_folders(&[IMAGES_FOLDER, FILES_FOLDER])?;
        client::run(env_args)?;
    }

    Ok(())
}
