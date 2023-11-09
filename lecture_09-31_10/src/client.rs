use std::error::Error;
use std::io::{self, ErrorKind};
use std::io::{prelude::*, BufReader};
use std::net::{TcpStream, SocketAddr, SocketAddrV4};
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use std::thread;
use std::time::Duration;
use crate::message::*;
use crate::helpers::*;

pub fn run(address: impl Into<SocketAddrV4>) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(address.into())?;
    stream.set_nonblocking(true).expect("cannot be nonblocking... client");
    print_peer_address(&stream);

    let stdin_message = spawn_stdin_channel();
    
    loop {
        //println!("a");
        match stdin_message.try_recv() {
            Ok(mut message) => {
                send_encoded(message, &mut stream)
                    .unwrap_or_else(|error| {
                        format!("unable to send encoded message: {error}");
                    });
            },
            Err(TryRecvError::Empty) => (),
            Err(error) => eprintln!("error during receiving stdin messages: {error}"),
        }

        let mut b = vec![0; 4];
        //println!("b");
        match stream.read_exact(&mut b) {
            Ok(_) => println!("[REC]: {:?}", b),
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => {
                continue;
            },
            Err(e) => continue,
        }

        //thread::sleep(Duration::from_millis(1000));
    }
    Ok(())
}


// TODO: stdin().lines()
fn spawn_stdin_channel() -> Receiver<Message> {
    let (sender, receiver) = mpsc::channel::<Message>();

    thread::spawn((move || {
        #[cfg(debug_assertions)] println!("INFO: spawned stdin channel");
        
        loop {
            let mut stdin_line = String::new();

            match io::stdin().read_line(&mut stdin_line) {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("unable to read from stdin: {error}");
                    continue;
                },
            }
        
            match Message::try_from(stdin_line) {
                Ok(message) => sender.send(message).unwrap_or_else(|error| {
                    eprintln!("unable to send via channel: {error}");
                }),
                Err(error) => {
                    eprintln!("{}", error);
                    continue;
                }
            };
        }
    }));
    receiver
}

pub fn print_peer_address(server: &TcpStream) {
    let address = server
        .peer_addr()
        .map(|address| address.to_string())
        .unwrap_or_else(|error| "CLIENT ERROR: unable to show address".into());
    println!("Connected to: {address}");
}
