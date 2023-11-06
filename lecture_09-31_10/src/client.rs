use std::error::Error;
use std::io;
use std::io::{prelude::*, BufReader};
use std::net::{TcpStream, SocketAddr, SocketAddrV4};
use crate::client_helpers::*;
use crate::message::*;

pub fn run(address: impl Into<SocketAddrV4>) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(address.into())?;
    println!("Connected to: {}", get_peer_address(&stream));
    
    loop {
        let mut stdin_line = String::new();
        //let mut buffer = Vec::new();
        io::stdin().read_line(&mut stdin_line)?;
        let maybe_message = Message::try_from(stdin_line);

        let message = match maybe_message {
            Ok(message) => message,
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
        };

        println!("M: {:?}", message);
        let (length, message) = message.encode()?;
        //stream.write(&length.to_be_bytes())?;
        stream.write_all(&message)?;
        //message.to_writer(&mut stream).map_err(|error| eprintln!("Unable to send message to server: {error}."));
    }

    Ok(())
}

pub fn run_(address: impl Into<SocketAddrV4>) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(address.into())?;
    //stream.set_nonblocking(true)?;
    println!("Connected to: {}", get_peer_address(&stream));
    stream.write_all(b"nothing will be shown on the server side")?; // SUDDENLY THIS DOESN'T WORK
    //return Ok(());

    // TODO: .flatten -> .map
    let i = stdin_lines_iter();
    for stdin_line in i {
        let maybe_message = Message::try_from(stdin_line);
        stream.write(b"wtf");

        let message = match maybe_message {
            Ok(message) => message,
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
        };
        //let (length, message) = message.encode()?;
        //message.to_writer(&mut stream).map_err(|error| eprintln!("Unable to send message to server: {error}."));
        //stream.flush();

        //let msg = "hey";
        //let write_result = stream.write_all(msg.as_bytes());
        //stream.flush();
        //println!("{:?}", write_result);
        //let mut buf = vec![0_u8; 1024];
        //stream.read(&mut buf);
        //let text = String::from_utf8(buf).unwrap();
        //println!("[FROM SERVER]: {}", text);
    }

    println!("[CLIENT - closed]");
    Ok(())
}

fn stdin_lines_iter() -> impl Iterator<Item = String> {
    io::stdin()
        .lines()
        .flatten() // TODO... now it discards errors
        .take_while(|message| !message.is_empty())
}
