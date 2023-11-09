use std::net::{TcpStream, TcpListener};
use crate::message::Message;
use std::error::Error;
use std::io::{Write, ErrorKind, self};

pub fn get_server_address(server: &TcpListener) -> String {
    server
        .local_addr()
        .map(|address| address.to_string())
        .unwrap_or_else(|_| "SERVER ERROR: unable to show address".into())
}

pub fn send_encoded(message: Message, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let (length, message) = message.encode()?;
    //println!("enc: {:?}", message);
    stream.write(&length.to_be_bytes())?;
    stream.write_all(&message)?;
    Ok(())
}

pub fn send_encoded_(message: Message, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    stream.set_nonblocking(true).unwrap();
    let (length, message) = message.encode()?;

    loop {
        match stream.write(&length.to_be_bytes()) {
            Ok(_) => stream.write_all(&message)?,
            Err(ref error) if error.kind() == ErrorKind::WouldBlock => break,
            Err(error) => panic!("PANIC in send_encode fn"),
        }
    }
    
    //println!("written: {:?}, {:?}", x, stream);
    //stream.write(&length.to_be_bytes())?;
    
    Ok(())
}

macro_rules! new_thread {
    ( $x:expr ) => {
        std::thread::spawn(move || {
            $x;
        })
    };
}
pub(crate) use new_thread; 


macro_rules! thread_share {
    ( $x:expr ) => {
        std::sync::Arc::new(std::sync::Mutex::new($x));
    };
}
pub(crate) use thread_share; 

macro_rules! buffered {
    ( $x:expr ) => {
        std::io::BufReader::new($x)
    };
}
pub(crate) use buffered; 