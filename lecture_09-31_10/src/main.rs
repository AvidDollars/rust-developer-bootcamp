#![allow(unused)]

use std::error::Error;

use env_args::*;
use message::Message;
pub mod client;
pub mod config;
pub mod env_args;
pub mod fs_ops;
mod helpers;
pub mod message;
pub mod server;
use std::net::Ipv4Addr;

fn main() -> Result<(), Box<dyn Error>> {
    pick_mode()?;
    Ok(())
}

fn pick_mode() -> Result<(), Box<dyn Error>> {
    let env_args = EnvArgs::new()?;
    if env_args.is_server() {
        //fs_ops::create_missing_folders(&[IMAGES_FOLDER, FILES_FOLDER])?;
        server::run(env_args)?
    } else {
        client::run(env_args)?
    }

    Ok(())
}
