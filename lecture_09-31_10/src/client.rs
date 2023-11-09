use crate::helpers::*;
use crate::message::*;
use std::error::Error;
use std::io::{self, ErrorKind};
use std::io::{prelude::*, BufReader};
use std::net::{SocketAddr, SocketAddrV4, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

pub fn run(address: impl Into<SocketAddrV4>) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(address.into())?;
    stream
        .set_nonblocking(true)
        .expect("cannot be nonblocking... client");
    print_peer_address(&stream);

    let stdin_message = spawn_stdin_channel();

    let mut message_buffer = [0; 8192];

    loop {
        match stdin_message.try_recv() {
            Ok(mut message) => {
                message.set_missing_address(stream.local_addr().unwrap()); // TODO: unwrap

                send_encoded(message, &mut stream).unwrap_or_else(|error| {
                    format!("unable to send encoded message: {error}");
                });
            }
            Err(TryRecvError::Empty) => (),
            Err(error) => eprintln!("error during receiving stdin messages: {error}"),
        }

        // may not be enough for large data -> does not take into account the case where data exceeds capacity of the buffer
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
                println!("bm: {:?}", Message::decode_from_slice(&message_in_bytes));

                //message_buffer = [0; 8192];
            }
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => continue,
        }

        //thread::sleep(Duration::from_millis(1000));
    }
    Ok(())
}

// TODO: stdin().lines()
fn spawn_stdin_channel() -> Receiver<Message> {
    let (sender, receiver) = mpsc::channel::<Message>();

    thread::spawn(
        (move || {
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
                    Err(error) => {
                        eprintln!("{}", error);
                        continue;
                    }
                };
            }
        }),
    );
    receiver
}

pub fn print_peer_address(server: &TcpStream) {
    let address = server
        .peer_addr()
        .map(|address| address.to_string())
        .unwrap_or_else(|error| "CLIENT ERROR: unable to show address".into());
    println!("Connected to: {address}");
}
