//! module containing [`AppError`] enum.

use std::fmt::{Debug, Display, Formatter, Result};

pub enum AppError {
    MissingCommand,
    MissingArgument,
    InvalidCliArgument { message: String },
    FileRead { message: String },
    OnOutput { message: String },
    ThreadSpawn { message: String },
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use AppError::*;

        let message = match self {
            MissingCommand => "Missing command.",
            MissingArgument => "Missing argument for command.",
            InvalidCliArgument { message }
            | OnOutput { message }
            | ThreadSpawn { message }
            | FileRead { message } => message,
        };

        write!(f, "{}", message)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use AppError::*;

        let message = match self {
            MissingCommand => "Missing command.",
            MissingArgument => "Missing argument for command.",
            InvalidCliArgument { message }
            | OnOutput { message }
            | ThreadSpawn { message }
            | FileRead { message } => message,
        };

        write!(f, "{}", message)
    }
}
