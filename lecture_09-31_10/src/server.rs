use crate::env_args::EnvArgs;
use crate::helpers::*;
use crate::message::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, ErrorKind};
use std::io::{prelude::*, BufReader};
use std::net::{Shutdown, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn broadcast_from(sender: String, recipients: Vec<String>) {}

pub fn run(address: impl Into<SocketAddrV4>) -> Result<(), io::Error> {
    let listener = TcpListener::bind(address.into())?;
    listener.set_nonblocking(true)?;
    println!("SERVER LISTENING ON: {}", get_server_address(&listener));

    //let clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    let mut clients = vec![];
    let (client_sender, server_receiver) = mpsc::channel::<Message>();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                clients.push(stream.try_clone().unwrap()); // something better than cloning ???
                let client_sender = client_sender.clone();
                new_thread!(read_and_broadcast(stream, client_sender));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                match &server_receiver.try_recv() {
                    Ok(message) => {
                        #[cfg(debug_assertions)]
                        println!("INFO: message arrived: {:?}", message);

                        for client_stream in &mut clients {
                            if client_stream.peer_addr().unwrap() != message.address().unwrap() {
                                // TODO: unwrap
                                send_encoded_to_client(message.clone(), client_stream);
                            }
                        }
                    }
                    Err(TryRecvError::Empty) => (),
                    Err(error) => eprintln!("error during receiving stdin messages: {error}"),
                }
            }
            Err(error) => eprintln!("encountered IO error: {error}"),
        }
        //println!("{:?}", clients);
        //thread::sleep(Duration::from_millis(2000));
    }
    println!("[END]: listening");

    Ok(())
}

// TODO: rename it to something less retarded
fn read_and_broadcast(mut stream: TcpStream, client_sender: Sender<Message>) {
    loop {
        let message = Message::try_from(&mut stream).unwrap(); // may return connection reset
        client_sender.send(message);
    }
}
