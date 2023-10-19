//! module containing [`AppError`] enum.

use std::fmt::{Debug, Display, Formatter, Result};

pub enum AppError {
    MissingCliArgument,
    InvalidCliArgument { message: String },
    OnOutput(String),
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use AppError::*;

        let message = match self {
            MissingCliArgument => "Missing CLI argument.",
            InvalidCliArgument { message } | OnOutput(message) => message,
        };

        write!(f, "{}", message)
    }
}

impl Display for AppError {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        todo!();
    }
}
