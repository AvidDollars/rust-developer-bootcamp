use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyErr;

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyErr")
    }
}

impl Error for MyErr {}

fn get_error() -> Box<dyn Error> {
    Box::new(MyErr)
}

fn downcasting(error: Box<dyn Error>) {
    // impl dyn Error + 'static
    if let Some(e) = error.downcast_ref::<MyErr>() { // or .downcast (consumes error)
        println!("Caught specific error: {}", e);
    }
}

fn process_num(s: &str) -> Result<String, String> {
    s.parse::<i32>()
        .map_err(|s| s.to_string())
        .and_then(|n| Ok(format!("Num: {}", n))) // executed only if Result is Ok
}