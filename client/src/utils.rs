use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver};

use shared::constants::LOGS_FOLDER;
use shared::message::Message;
use shared::tracing::{error, info};
use shared::utils::new_thread;

use anyhow::{anyhow, Result as AnyResult};
use chrono::Utc;

pub fn get_log_file() -> fn() -> Box<dyn Write> {
    || {
        let log_file = create_log_file().unwrap_or_else(|error| {
            eprintln!("unable to create log file: {error}");
            std::process::exit(1);
        });
        Box::new(log_file)
    }
}

pub fn create_log_file() -> AnyResult<File> {
    let logs_path = Path::new(LOGS_FOLDER);
    let today_date = Utc::now().format("%d-%m-%Y"); // one log file per day

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(logs_path.join(today_date.to_string() + ".log"))?;

    Ok(file)
}

pub fn create_missing_folders(paths: &[&str]) -> AnyResult<()> {
    for path in paths {
        let folder_creation = match fs::create_dir(path) {
            Err(error) => Err(error),
            Ok(_) => Ok(()),
        };

        if let Err(error) = folder_creation {
            match error.kind() {
                ErrorKind::AlreadyExists => (),
                other_error => return Err(anyhow!(other_error)),
            }
        }
    }
    Ok(())
}

// receiving stdin:
// spawned in new thread, so that client cant both receive stdin input & messages from server in non-blocking manner
pub fn spawn_stdin_channel() -> Receiver<Message> {
    let (sender, receiver) = mpsc::channel::<Message>();

    new_thread!({
        info!("spawned stdin channel");

        loop {
            let mut stdin_line = String::new();

            match io::stdin().read_line(&mut stdin_line) {
                Ok(_) => (),
                Err(error) => {
                    error!("{}", error);
                    eprintln!("unable to read from stdin");
                    continue;
                }
            }

            match Message::try_from(stdin_line.trim()) {
                Ok(message) => sender.send(message).unwrap_or_else(|error| {
                    error!("{}", error);
                    eprintln!("unable to send via channel");
                }),
                Err(error) => {
                    error!("error: {}, stdin input: {}", error, stdin_line.trim());
                    eprintln!("invalid input: {error}")
                }
            };
        }
    });
    receiver
}
