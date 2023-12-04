use std::io;

use image::ImageError;
use thiserror::Error as ThisErr;
use tracing::subscriber::SetGlobalDefaultError;

#[derive(Debug, ThisErr)]
pub enum AppError {
    #[error("couldn't initialize logging: {0}")]
    InitLogging(#[from] SetGlobalDefaultError),

    #[error("couldn't convert to .png: {0}")]
    ConversionToPng(#[from] ImageError),

    #[error("IO error occurred: {0}")]
    IO(#[from] io::Error),

    #[error("couldn't convert - unsupported conversion")]
    UnsupportedConversion,
}
