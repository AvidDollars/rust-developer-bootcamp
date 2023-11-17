#![deny(unused, unreachable_code, unreachable_patterns)]

mod app;
use shared::env_args::EnvArgs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let env_args = EnvArgs::new()?;
    app::run(env_args)?;
    Ok(())
}
