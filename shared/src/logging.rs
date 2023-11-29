use crate::env_args::EnvArgs;
use crate::errors::AppError;
use std::io::Write;

pub fn init_tracing(env_args: &EnvArgs, writer: fn() -> Box<dyn Write>) -> Result<(), AppError> {
    let subscriber = tracing_subscriber::fmt()
        .with_file(true)
        .with_max_level(env_args.log_level)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_writer(writer) // Client: file, Server: stdout
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
