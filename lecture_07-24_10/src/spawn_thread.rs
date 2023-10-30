use std::env;
use std::io;
use std::mem::MaybeUninit;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use crate::commands::*;
use crate::extensions::OnlyFirstCliArg;

pub fn input_receiver(sender: mpsc::Sender<Command>) -> JoinHandle<()> {
    let handler = thread::spawn(move || {
        let env_arg = env::args().only_first_provided();

        #[allow(invalid_value)] // guaranteed to be initialized
        let mut transformer = unsafe { MaybeUninit::uninit().assume_init() };

        if let Some(ref arg) = env_arg {
            match get_text_transformer(arg) {
                Ok(arg) => transformer = arg,
                Err(error) => {
                    eprintln!("{}", error);
                    std::process::exit(1);
                }
            }
        }

        for line in io::stdin().lines() {
            let line = match line {
                Ok(content) => content,
                Err(error) => {
                    eprintln!("Couldn't read the line: {error}.");
                    continue;
                }
            };

            let maybe_command = match env_arg.is_none() {
                true => Command::try_from(line),
                false => Command::new(transformer, line),
            };

            let command = match maybe_command {
                Ok(content) => content,
                Err(error) => {
                    eprintln!("{}", error);
                    continue;
                }
            };

            if let Err(error) = sender.send(command) {
                eprintln!("Couldn't send message over channel: {error}.");
                return;
            }
        }
    });
    handler
}

pub fn processing() -> (JoinHandle<()>, mpsc::Sender<Command>) {
    // couldn't infer... wtf?
    let (sender, receiver): (mpsc::Sender<Command>, mpsc::Receiver<Command>) = mpsc::channel();
    let handler = thread::spawn(move || {
        receiver.iter().for_each(|command| {
            if let Err(error) = command.apply(&mut io::stdout()) {
                eprintln!("Error during processing command: {error}.");
            }
        });
    });
    (handler, sender)
}
