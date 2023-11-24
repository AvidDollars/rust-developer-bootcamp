#![deny(unused, unreachable_code, unreachable_patterns)]

mod app;
use shared::env_args::EnvArgs;
use shared::logging::init_tracing;
use shared::tracing::info;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let env_args = EnvArgs::new()?;
    init_tracing(&env_args, || Box::new(io::stdout()))?;

    info!("process started");
    app::run(env_args)?;
    info!("process ended");

    Ok(())
}
