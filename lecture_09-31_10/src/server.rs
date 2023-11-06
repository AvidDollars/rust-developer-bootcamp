use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, ErrorKind};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream, SocketAddr, Shutdown, SocketAddrV4};
use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use crate::server_helpers::*;
use crate::env_args::EnvArgs;
use crate::message::*;

macro_rules! new_thread {
    ( $x:expr ) => {
        std::thread::spawn(move || {
            $x;
        })
    };
}

macro_rules! thread_share {
    ( $x:expr ) => {
        std::sync::Arc::new(std::sync::Mutex::new($x));
    };
}

macro_rules! buffered {
    ( $x:expr ) => {
        std::io::BufReader::new($x)
    };
}

fn broadcast_from(sender: String, recipients: Vec<String>) {

}


pub fn run(address: impl Into<SocketAddrV4>) -> Result<(), io::Error> {
    let listener = TcpListener::bind(address.into())?;
    listener.set_nonblocking(true)?;
    println!("SERVER LISTENING ON: {}", get_server_address(&listener));

    //let clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    let mut clients = vec![];
    let (client_sender, server_receiver) = mpsc::channel::<String>();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                clients.push(stream.try_clone().unwrap()); // something better than cloning ???
                let client_sender = client_sender.clone();
                new_thread!(read_stream_bytes(stream, client_sender));
                
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {

                for message in &server_receiver.try_recv() {
                    let message = format!("[FINAL]: {message}\n");

                    for mut client_stream in &clients {
                        client_stream.write(message.as_bytes());
                    }
                }
            }
            Err(error) => eprintln!("encountered IO error: {error}"),
        }
        println!("{:?}", clients);
        thread::sleep(Duration::from_millis(2000));

    }
    println!("[END]: listening");

    Ok(())
}

fn read_stream_lines_(mut stream: TcpStream, client_sender: Sender<String>) {
        
    loop {

        let mut len_bytes = [0_u8; 4];

        match stream.read_exact(&mut len_bytes) {
            Ok(_) => {
                let len = u32::from_be_bytes(len_bytes) as usize;
                let mut buffer = vec![0_u8; len];
                stream.read_exact(&mut buffer);
                println!("D: {:?}", buffer);
                //let d = Message::decode_from_slice(&buffer);
            },
            Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => continue,
            Err(error) => {
                eprintln!("Error during reading stream: {error}");
                continue;
            }
        };

        thread::sleep(Duration::from_millis(2000));

    } 
}

fn read_stream_bytes(mut stream: TcpStream, client_sender: Sender<String>) {
    //let mut len_bytes = [0_u8; 4];
    //let mut index = 0;
    let mut content = vec![];

    for client_stream in buffered!(&stream).bytes() {

        match client_stream {

            Ok(byte) => {

                content.push(byte);     
            },
            
            Err(error) => {
                match error.kind() {
                    ErrorKind::ConnectionReset => return,
                    ErrorKind::WouldBlock => continue,
                    error => {
                        eprintln!("Error during reading stream: {error}");
                        continue; // ?
                    },
                }
            }
        };

        //println!("v: {:?}", v);

        //if line.is_empty() {
        //    break;
        //}

        //println!("[LINE]: {:?}", line);
        //client_sender.send(line);
    }   
}
