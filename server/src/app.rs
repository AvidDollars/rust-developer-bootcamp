use shared::message::Message;
use shared::tracing::{debug, error, info, trace};
use shared::utils::{get_server_address, new_thread, send_encoded};

use std::fmt::Debug;
use std::io::{self, ErrorKind};
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, TryRecvError};

pub fn run<A: Into<SocketAddrV4> + Debug>(address: A) -> Result<(), io::Error> {
    let listener = TcpListener::bind(address.into())?;
    listener.set_nonblocking(true)?;
    info!("listening on: {}", get_server_address(&listener));

    let mut clients = vec![]; // not ideal, but sufficient enough for small amount of connected clients
    let (client_sender, server_receiver) = mpsc::channel::<Message>();

    for stream in listener.incoming() {
        match stream {
            // 1 client = 1 new thread
            Ok(connection) => {
                handle_new_connection(connection, &mut clients, client_sender.clone())?;
            }

            // process messages coming from clients
            Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => {
                match &server_receiver.try_recv() {
                    Ok(message) => broadcast(message, &mut clients, &stream)?,
                    Err(TryRecvError::Empty) => (),
                    Err(error) => error!("error during receiving stdin messages: {}", error),
                }
            }
            Err(error) => error!("encountered IO error: {}", error),
        }
    }
    Ok(())
}

// removes TcpStream from the vector when a client ends the connection
fn quit_message_handler(
    message: &Message,
    clients: &mut Vec<TcpStream>,
    stream: &Result<TcpStream, io::Error>,
) {
    let address = message.get_sender().expect("message will have sender");

    let maybe_idx = clients
        .iter()
        .enumerate()
        .find(|(_index, stream)| stream.peer_addr().unwrap() == address);

    if maybe_idx.is_some() {
        let (idx, stream) = maybe_idx.expect("maybe_idx.is_some() == true");
        debug!("removing from clients: {:?}", stream);
        clients.remove(idx);
    } else {
        error!("not in the list of clients: {:?}", stream);
    }
}

fn handle_new_connection(
    connection: TcpStream,
    clients: &mut Vec<TcpStream>,
    client_sender: Sender<Message>,
) -> io::Result<()> {
    info!("new connection: {:?}", connection);

    // broadcasting that new client is connected
    let new_connection_message = Message::new_connection(Some(connection.peer_addr().unwrap()));
    broadcast_message_to_other_clients(&new_connection_message, clients)?;

    // size_of::<TcpStream>() -> 8 bytes... I guess in that case is cloing OK, isn't it?
    clients.push(connection.try_clone()?);
    new_thread!(read_from_client_and_send_for_broadcasting(
        connection,
        client_sender
    ));
    Ok(())
}

fn broadcast_message_to_other_clients(
    message: &Message,
    clients: &mut Vec<TcpStream>,
) -> Result<(), io::Error> {
    trace!(
        "broadcasting message: {:?}, num of clients: {}",
        message,
        clients.len()
    );

    for client_stream in clients {
        let peer_address = client_stream.peer_addr().map_err(|error| {
            error!("cannot obtain peer address during broadcasting: {}", error);
            io::Error::from(ErrorKind::AddrNotAvailable)
        })?;

        if peer_address != message.get_sender().expect("message will have sender") {
            match send_encoded(message.clone(), client_stream) {
                Ok(_) => debug!("sending encoded message: {:?}", client_stream),
                Err(error) => error!("error during broadcasting: {}", error),
            }
        }
    }
    Ok(())
}

fn broadcast(
    message: &Message,
    clients: &mut Vec<TcpStream>,
    stream: &Result<TcpStream, io::Error>,
) -> Result<(), io::Error> {
    debug!("message arrived: {:?}", message);

    if message.is_quit() {
        broadcast_message_to_other_clients(message, clients)?;
        quit_message_handler(message, clients, stream);
    } else {
        broadcast_message_to_other_clients(message, clients)?
    }
    Ok(())
}

fn read_from_client_and_send_for_broadcasting(
    mut stream: TcpStream,
    client_sender: Sender<Message>,
) {
    loop {
        let message = Message::try_from(&mut stream);

        match message {
            Ok(message) => {
                let is_quit_message = message.is_quit();

                let _ = client_sender
                    .send(message)
                    .map_err(|error| error!("unable to send message: {}", error));

                if is_quit_message {
                    return; // otherwise [DISCONNECTED] message will be broadcasted twice
                }
            }
            Err(ref error)
                if error.kind() == ErrorKind::ConnectionReset // e.g. SIGINT
                    || error.kind() == ErrorKind::ConnectionAborted =>
            {
                let peer_address = stream.peer_addr().map_err(|error| {
                    error!("cannot obtain peer address (client will not be removed from Vector of clients -> memory leak): {}", error);
                });

                if let Ok(address) = peer_address {
                    let quit_message = Message::quit_signal(Some(address));
                    client_sender
                        .send(quit_message)
                        .unwrap_or_else(|error| error!("unable to send message: {}", error));
                }

                return;
            }
            Err(error) => {
                error!("unable to broadcast: {}", error);
                return;
            }
        };
    }
}
