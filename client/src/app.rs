use std::io::{self, prelude::*, Error as IoError, ErrorKind};
use std::net::SocketAddr;
use std::net::{SocketAddrV4, TcpStream};
use std::sync::mpsc::TryRecvError;

use crate::utils::spawn_stdin_channel;

use shared::constants::CLIENT_MSG_BUFFER_SIZE;
use shared::message::{CborResult, Message, MessageType};
use shared::tracing::{error, info};
use shared::utils::{get_peer_address, send_encoded};

pub fn run(address: impl Into<SocketAddrV4>) -> io::Result<SocketAddr> {
    let mut stream = try_connect(address)?;
    println!("connected to: {}", get_peer_address(&stream));

    let stdin_message = spawn_stdin_channel();
    let mut message_buffer = [0; CLIENT_MSG_BUFFER_SIZE];

    loop {
        // receives stdin input & sends it to server for broadcasting
        match stdin_message.try_recv() {
            Ok(message) => send_stdin_message(message, &mut stream)?,
            Err(TryRecvError::Empty) => (),
            Err(error) => error!("error during receiving stdin messages: {}", error),
        }

        // receives broadcasted messages from server
        match stream.read(&mut message_buffer) {
            Ok(_) => match try_decode_message_from_buffer(message_buffer) {
                Ok(message) => message_handler(message),
                Err(error) => error!("message couldn't be decoded: {}", error),
            },
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => (),
            Err(ref error) if error.kind() == ErrorKind::ConnectionReset => {
                let message = "exiting the process - connection was terminated by the server";
                println!("{message}");
                info!("{}\n", message);
                std::process::exit(0);
            }
            Err(error) => error!("error during receiving message from server: {}", error),
        }
    }
}

fn tcp_connect_error_hander(error: IoError) -> IoError {
    if error.kind() == ErrorKind::ConnectionRefused {
        eprintln!("no connection could be made - target computer actively refused it\n");
        std::process::exit(1);
    }
    return error;
}

fn message_sender_error_handler(error: IoError) {
    if error.kind() == ErrorKind::OutOfMemory {
        eprintln!(
            "too big to send - max size of a message: {} bytes",
            CLIENT_MSG_BUFFER_SIZE
        );
    } else {
        eprintln!("unable to send encoded message");
        error!("{}", error);
    }
}

fn try_connect(address: impl Into<SocketAddrV4>) -> Result<TcpStream, IoError> {
    TcpStream::connect(address.into())
        .map(|stream| {
            stream
                .set_nonblocking(true)
                .expect("client: TcpStream cannot be nonblocking");
            stream
        })
        .map_err(tcp_connect_error_hander)
}

fn send_stdin_message(mut message: Message, stream: &mut TcpStream) -> io::Result<()> {
    if message.is_empty() {
        return Ok(());
    }

    let address = stream.local_addr()?;
    message.set_sender(address);
    let is_quit_message = message.type_ == MessageType::QuitSignal;
    send_encoded(message, stream).unwrap_or_else(message_sender_error_handler);

    if is_quit_message {
        info!("exiting the process\n");
        std::process::exit(0);
    } else {
        Ok(())
    }
}

fn try_decode_message_from_buffer(
    message_buffer: [u8; CLIENT_MSG_BUFFER_SIZE],
) -> CborResult<Message> {
    const SIZE: usize = 4;
    let mut message = message_buffer.into_iter();
    let mut len_bytes = [0_u8; SIZE];

    message
        .by_ref()
        .take(SIZE)
        .enumerate()
        .for_each(|(index, byte)| len_bytes[index] = byte);

    let content_len = u32::from_be_bytes(len_bytes) as usize;
    let message_in_bytes: Vec<u8> = message.take(content_len).collect();
    Message::decode_from_slice(&message_in_bytes)
}

fn message_handler(mut message: Message) {
    println!("{}", message);

    // Ok(false) -> is image, but not png -> conversion to .png
    if let Ok(false) = message.is_image_and_png() {
        let conversion_result = message.convert_image_to_png();

        if let Err(error) = conversion_result {
            match error.kind() {
                ErrorKind::InvalidData => {
                    error!("{}", error);
                    eprintln!("image cannot be converted to .png");
                }
                ErrorKind::Other => {
                    error!("{}", error);
                    eprintln!("image cannot be saved");
                }
                ErrorKind::Unsupported => (),
                _ => unreachable!(),
            }
            return;
        }
    }

    let _ = message.save_file().map_err(|error| {
        // saving plain message -> Unsupported
        if error.kind() != ErrorKind::Unsupported {
            error!("{}", error);
            eprintln!("cannot save to file");
        }
    });
}
