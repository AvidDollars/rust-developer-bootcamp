use std::fmt::{Debug, Display, Formatter, Result};

//#[derive(Debug)]
pub enum AppError {
    MissingCliArgument,
    InvalidCliArgument { message: String },
}

// IS THERE A WAY TO GET RID OF DUPLICATE CODE?
// BOTH "Debug" & "Display" HAVE THE SAME IMPLEMENTATION
impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use AppError::*;

        let message = match self {
            MissingCliArgument => "Missing CLI argument.",
            InvalidCliArgument { message } => message,
        };

        write!(f, "{}", message)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use AppError::*;

        let message = match self {
            MissingCliArgument => "Missing CLI argument.",
            InvalidCliArgument { message } => message,
        };

        write!(f, "{}", message)
    }
}
