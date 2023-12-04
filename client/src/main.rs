#![deny(unused, unreachable_code, unreachable_patterns)]

mod app;
mod utils;

use crate::utils::{create_missing_folders, get_log_file};

use shared::constants::{FILES_FOLDER, IMAGES_FOLDER, LOGS_FOLDER};
use shared::env_args::EnvArgs;
use shared::logging::init_tracing;
use shared::tracing::info;

use anyhow::Result as AnyResult;

fn main() -> AnyResult<()> {
    let env_args = EnvArgs::new();
    create_missing_folders(&[FILES_FOLDER, IMAGES_FOLDER, LOGS_FOLDER])?;
    init_tracing(&env_args, get_log_file())?;
    let connection_address = env_args.get_address();

    info!("[{}]: process started", connection_address);
    app::run(env_args)?;
    info!("[{}]: process ended", connection_address);

    Ok(())
}
