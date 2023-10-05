//! homework - lecture 02  

use slug::slugify;
use std::collections::HashMap;
use std::{env, io, process};

fn main() {
    let cli_args = read_cli_args();
    exit_if_num_of_args_is_not_one(&cli_args);
    let arg = &cli_args[0];
    let string_modifier = get_string_modifier(arg);
    transform_stdin_with(string_modifier, "exit");
}

/// Returns a vector of arguments provied during program run.
/// 1st argument (the path of the executable) is omitted.
pub fn read_cli_args() -> Vec<String> {
    env::args().skip(1).collect()
}

/// Terminates the process if number of CLI arguments is not 1.
/// Prints error message to stderr before process termination.
pub fn exit_if_num_of_args_is_not_one(args: &[String]) {
    match args.len() {
        0 => {
            eprintln!("No argument provided.");
            process::exit(1);
        }
        1 => (),
        _ => {
            eprintln!("Too much arguments provided. Only one argument is allowed.");
            process::exit(1);
        }
    }
}

/// Returns a function pointer based on provided argument.
/// Function pointer takes a string as an argument which is then transformed and printed.
/// Process is terminated if an argument is not valid.
pub fn get_string_modifier(arg: &str) -> fn(&str) {
    let mut mapping: HashMap<&str, fn(&str)> = HashMap::new();
    mapping.insert("lowercase", |arg| println!("{}", arg.to_lowercase()));
    mapping.insert("uppercase", |arg| println!("{}", arg.to_uppercase()));
    mapping.insert("no-spaces", |arg| println!("{}", arg.replace(" ", "")));
    mapping.insert("slugify", |arg| println!("{}", slugify(arg)));
    mapping.insert("scream", |arg| println!("{}!!!", arg));
    mapping.insert("reverse", |arg| {
        println!("{}", arg.chars().rev().collect::<String>())
    });

    match mapping.get(arg) {
        Some(fn_to_execute) => *fn_to_execute,
        None => {
            let valid_args: Vec<_> = mapping.keys().copied().collect();
            eprintln!("Unknown argument '{arg}'.");
            eprintln!("Valid arguments are: {}", valid_args.join(" | "));
            process::exit(1);
        }
    }
}

/// Transforms lines taken from stdin based on provided 'text_transformer' function.
/// Read from stdin can be terminated with typing 'exit_signal'.
pub fn transform_stdin_with(text_transformer: fn(&str), exit_signal: &str) {
    println!("Provide your input. Type '{exit_signal}' for exiting the program.");

    io::stdin()
        .lines()
        .map(|line| match line {
            Ok(text) => text,
            Err(error) => {
                println!("[ FAILED TO READ THE LINE ]: {error}");
                String::new()
            }
        })
        .take_while(|line| line != exit_signal)
        .for_each(|line| text_transformer(&line));
}
