#![allow(unused)]

use std::error::Error;

use env_args::*;
use message::Message;
pub mod env_args;
pub mod client;
pub mod config;
pub mod fs_ops;
pub mod server;
pub mod message;
mod helpers;
use std::net::Ipv4Addr;

// logging, env vars parsing

fn main() -> Result<(), Box<dyn Error>> {
    pick_mode()?;
    Ok(())
}


fn pick_mode() -> Result<(), Box<dyn Error>> {
    let env_args = EnvArgs::new()?;
    match env_args.is_server() {
        true => {
            //fs_ops::create_missing_folders(&[IMAGES_FOLDER, FILES_FOLDER])?;
            server::run(env_args)?
        },
        false => client::run(env_args)?,
    }
    Ok(())
}
