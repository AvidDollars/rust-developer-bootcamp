use std::fmt::Display;

#[derive(Debug)]
struct MyError<'a> {
    message: &'a str,
}

impl<'a> Display for MyError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}