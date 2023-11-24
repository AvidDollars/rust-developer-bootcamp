use std::io;
use std::num::ParseIntError;

enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

fn process_file(path: &str) -> Result<i32, MyError> {

    // io::Error -> MyError
    let content = std::fs::read_to_string(path).map_err(MyError::Io)?;

    // ParseIntError -> MyError
    let number = content.trim().parse().map_err(MyError::Parse)?;
    Ok(number)
}