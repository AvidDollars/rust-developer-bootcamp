#![deny(unreachable_code, unreachable_patterns, unused)]

use std::any::Any;

pub mod commands;
pub mod csv_processing;
pub mod custom_errors;
pub mod extensions;
pub mod spawn_thread;

fn main() -> Result<(), Box<dyn Any + Send>> {
    let (thread_for_processing, sender) = spawn_thread::processing();
    let thread_for_receiving_input = spawn_thread::input_receiver(sender);

    thread_for_processing.join()?;
    thread_for_receiving_input.join()?;

    Ok(())
}
