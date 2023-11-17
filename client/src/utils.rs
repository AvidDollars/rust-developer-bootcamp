use std::fs;
use std::io::{self, ErrorKind};

use shared::message::Message;
use shared::utils::new_thread;

use std::sync::mpsc::{self, Receiver};

pub fn create_missing_folders(paths: &[&str]) -> io::Result<()> {
    for path in paths {
        let folder_creation = match fs::create_dir(path) {
            Err(error) => Err(error),
            Ok(_) => Ok(()),
        };

        if let Err(error) = folder_creation {
            match error.kind() {
                ErrorKind::AlreadyExists => (),
                other_error => return Err(io::Error::from(other_error)),
            }
        }
    }
    Ok(())
}

pub fn spawn_stdin_channel() -> Receiver<Message> {
    let (sender, receiver) = mpsc::channel::<Message>();

    new_thread!({
        #[cfg(debug_assertions)]
        println!("INFO: spawned stdin channel");

        loop {
            let mut stdin_line = String::new();

            match io::stdin().read_line(&mut stdin_line) {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("unable to read from stdin: {error}");
                    continue;
                }
            }

            match Message::try_from(stdin_line.trim()) {
                Ok(message) => sender.send(message).unwrap_or_else(|error| {
                    eprintln!("unable to send via channel: {error}");
                }),
                Err(error) => eprintln!("cannot create message from the input: {}", error),
            };
        }
    });
    receiver
}
