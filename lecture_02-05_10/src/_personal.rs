#![allow(unused)]
//! also doc comment

use std::mem;

/// this is doc comment
pub fn lol() {
    let hex = 0xFF;
    let inf = f64::INFINITY;
    let very_raw_string = r#" very raw "#;
    let no_newline = 
        "lol\
        no newline
        cool, right?";

    println!("{no_newline}");

    // shadowing
    let x = 5;
    let mut x = x.to_string();
    
}

fn show_size() {
    let size = mem::size_of::<char>();
    println!("{size}");
}

fn block_of_code() {
    // block of code is expression
    let out = {
        5
    };

    // STATEMENTS:
    //  
}

fn gimme() -> Option<i32> {
    None
}

fn tuples() {
    let (a, .., c) = (1, 2, 3);
    mem::drop(a);

    let (ref x, ..) = ("hello".to_owned(), "world".to_owned());

    let Some(num) = gimme() else {
        return;
    };
}

fn arr() {
    let a = ["a".to_owned(), "b".to_owned()];
    let el = &a[0];
}