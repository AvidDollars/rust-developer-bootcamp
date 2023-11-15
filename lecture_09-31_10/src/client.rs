use crate::config::CLIENT_MSG_BUFFER_SIZE;
use crate::helpers::*;
use crate::message::*;

use std::io::{self, prelude::*, ErrorKind};
use std::net::SocketAddr;
use std::net::{SocketAddrV4, TcpStream};
use std::sync::mpsc::{self, Receiver, TryRecvError};

pub fn run(address: impl Into<SocketAddrV4>) -> io::Result<SocketAddr> {
    let stream = TcpStream::connect(address.into());

    let mut stream = match stream {
        Ok(stream) => stream,
        Err(error) => {
            if error.kind() == ErrorKind::ConnectionRefused {
                eprintln!("no connection could be made - target computer actively refused it");
                std::process::exit(1);
            }
            return Err(error);
        }
    };

    stream
        .set_nonblocking(true)
        .expect("client: TcpStream cannot be nonblocking");

    print_peer_address(&stream);

    let stdin_message = spawn_stdin_channel();
    let mut message_buffer = [0; CLIENT_MSG_BUFFER_SIZE];

    loop {
        match stdin_message.try_recv() {
            Ok(mut message) => {
                message.set_sender(stream.local_addr()?);

                let quit_message = message.type_ == MessageType::QuitSignal;

                send_encoded(message, &mut stream).unwrap_or_else(|error| {
                    if error.kind() == ErrorKind::OutOfMemory {
                        eprintln!(
                            "too big to send - max size of a message: {} bytes",
                            CLIENT_MSG_BUFFER_SIZE
                        );
                    } else {
                        eprintln!("unable to send encoded message: {error}");
                    }
                });

                if quit_message {
                    std::process::exit(0);
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(error) => eprintln!("error during receiving stdin messages: {error}"),
        }

        // messages sent from server
        match stream.read(&mut message_buffer) {
            Ok(_) => {
                const SIZE: usize = 4;
                let mut message = message_buffer.into_iter();
                let mut len_bytes = [0_u8; SIZE];

                message
                    .by_ref()
                    .take(SIZE)
                    .enumerate()
                    .for_each(|(index, byte)| len_bytes[index] = byte);

                let content_len = u32::from_be_bytes(len_bytes) as usize;
                let message_in_bytes: Vec<_> = message.take(content_len).collect();

                match Message::decode_from_slice(&message_in_bytes) {
                    Ok(mut message) => {
                        println!("{}", message);

                        // Ok(false) -> is image, but not png -> conversion to .png
                        if let Ok(false) = message.is_image_and_png() {
                            let conversion_result = message.convert_image_to_png();

                            if let Err(error) = conversion_result {
                                match error.kind() {
                                    ErrorKind::InvalidData => {
                                        eprintln!("image cannot be converterted to .png");
                                    }
                                    ErrorKind::Other => {
                                        eprintln!("image cannot be saved");
                                    }
                                    ErrorKind::Unsupported => (),
                                    _ => unreachable!(),
                                }
                            }
                        }

                        let _ = message.save_file().map_err(|error| {
                            // saving plain message -> Unsupported
                            if error.kind() != ErrorKind::Unsupported {
                                eprintln!("cannot save: {error}");
                            }
                        });
                    }
                    Err(error) => {
                        eprintln!("message couldn't be decoded: {error}");
                    }
                }
            }

            Err(ref error) if error.kind() == ErrorKind::WouldBlock => (),

            Err(ref error) if error.kind() == ErrorKind::ConnectionReset => {
                println!("exiting the process - connection was terminated by the server");
                std::process::exit(0);
            }

            Err(error) => eprintln!("error during receiving message from server: {error}"),
        }
    }
}

fn spawn_stdin_channel() -> Receiver<Message> {
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
