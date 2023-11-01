use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::error::Error;

/// LISTENING ON SERVER ///
pub fn listen_and_read() -> Result<(), Box<dyn Error>> {
    let port = "0.0.0.0:42069";
    let listener = TcpListener::bind(port)?;

    println!("[START]: listening on {port}");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        read_stream_lines(stream);
        break;
        
    };
    println!("[END]: listening");

    Ok(())
}

// on client: "netcat 10.0.0.58 42069"
fn read_stream_lines(mut stream: TcpStream) {
    BufReader::new(&mut stream)
        .lines()
        .flatten() // .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .for_each(|line| println!("{}", line));
}

/// SENDING MESSAGE TO SERVER ///
/// 
/// on server: nc -l 42069
pub fn send_to_server() -> Result<(), Box<dyn Error>> {
    let messages = ["hello\n", "world\n", "wtf?\n", "bye...\n"];

    let mut stream = TcpStream::connect("10.0.0.4:42069")?;

    messages.into_iter()
        .for_each(|message| {
            stream.write_all(message.as_bytes());
        });
    Ok(())
}
