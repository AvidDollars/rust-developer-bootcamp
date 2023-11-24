mod a_simple_error;
mod b_borrowed_error;
mod c_handling_different_errors_without_conversions;
mod d_downcasting_errors;
mod e_anyhow;
mod f_thiserror;

#[cfg(feature = "nice-errors")]
use color_eyre::{Result, eyre::{anyhow, Context}};

#[cfg(not(feature = "nice-errors"))]
use anyhow::{Result, anyhow, Context};

// BEST PRACTISE:
//  -   ThisError ->    library
//  -   AnyHow    ->    code built on top of library

fn main() {
    // 1:00:34
    
    // cargo run --feature nice-errors
    #[cfg(feature = "nice-errors")] {
        println!("nice-errors: PRESENT");
    }
     
    //e_anyhow_thiserror::process_tasks();
    
}

// LIBRARIES

// AnyHow
//
//  -   useful in application code
//  -   forks:
//          eyre
//          color-eyre

// ThisError
//
//  -   aimed for creating libs / frameworks