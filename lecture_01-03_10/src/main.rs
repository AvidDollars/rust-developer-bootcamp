use std::{error::Error, io};

fn main() {
    read_from_stdin();
}

fn read_from_stdin() {
    println!("Enter your name: ");

    let mut name = String::new();
    let stdin_read_result = io::stdin().read_line(&mut name);
    let name = name.trim();

    match stdin_read_result {
        Ok(_) => println!("Hello, {}!", name),
        Err(error) => print_err_and_exit(error),
    }
}

fn print_err_and_exit(error: impl Error) {
    eprintln!("[ ERROR ]: {error}");
    std::process::exit(1);
}
