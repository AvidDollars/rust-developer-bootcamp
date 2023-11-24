use std::error::Error;
use std::fmt::{self, Display, Debug};
use std::backtrace::Backtrace;
use std::io;

#[derive(Debug)]
struct AppError {
    message: String,
    inner_error: Option<io::Error>,
}

impl Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "AppError: {}", self.message)
    }
}

impl Error for AppError {

    // to track down from where the error is coming from
    fn source(&self) -> Option<&'static dyn Error> {
        let r = self.inner_error.as_ref();
        todo!();
    }
}