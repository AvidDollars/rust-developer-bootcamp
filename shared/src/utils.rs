use crate::constants::CLIENT_MSG_BUFFER_SIZE;
use crate::message::Message;
use std::fs;
use std::io::{self, ErrorKind, Result as IoResult, Write};
use std::net::{TcpListener, TcpStream};
use std::time::SystemTime;
use tracing::trace;

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

pub fn current_timestamp() -> Result<String, String> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|value| value.as_millis().to_string())
        .map_err(|error| error.to_string())?;
    Ok(timestamp)
}

pub fn get_peer_address(server: &TcpStream) -> String {
    server
        .peer_addr()
        .map(|address| address.to_string())
        .unwrap_or_else(|error| format!("client: unable to show address: {error}"))
}

pub fn get_server_address(server: &TcpListener) -> String {
    server
        .local_addr()
        .map(|address| address.to_string())
        .unwrap_or_else(|error| format!("SERVER ERROR: unable to show address: {error}"))
}

pub fn send_encoded(message: Message, stream: &mut TcpStream) -> IoResult<()> {
    trace!(
        "sending encoded message: {:?}, stream: {:?}",
        message,
        stream
    );
    let (length, message) = message
        .encode()
        .map_err(|_error| io::Error::from(ErrorKind::InvalidData))?;

    let buffer_overflow = (length + 4) as usize > CLIENT_MSG_BUFFER_SIZE;

    if buffer_overflow {
        return Err(io::Error::from(ErrorKind::OutOfMemory));
    }

    let len_and_message_chained: Vec<_> = length.to_be_bytes().into_iter().chain(message).collect();
    stream.write_all(&len_and_message_chained)?;
    Ok(())
}

#[macro_export]
macro_rules! buffered {
    ( $x:expr ) => {
        std::io::BufReader::new($x)
    };
}
pub(crate) use buffered;

#[macro_export]
macro_rules! new_thread {
    ( $x:expr ) => {
        std::thread::spawn(move || {
            $x;
        })
    };
}
pub use new_thread;
