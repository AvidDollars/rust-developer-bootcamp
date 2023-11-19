use shared::message::Message;
use shared::tracing::{debug, error, info};
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
            Ok(connection) => {
                handle_new_connection(connection, &mut clients, client_sender.clone())?
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

fn quit_message_handler(
    message: &Message,
    clients: &mut Vec<TcpStream>,
    stream: &Result<TcpStream, io::Error>,
) {
    let address = message.get_sender().expect("message will have sender");

    // remove TcpStream from the vector when a client sends ".quit"
    let maybe_idx = clients
        .iter()
        .enumerate()
        .find(|(_index, stream)| stream.peer_addr().unwrap() == address);

    if maybe_idx.is_some() {
        let (idx, stream) = maybe_idx.expect("maybe_idx.is_some() == true");
        debug!("removing from clients: {:?}", stream);
        clients.remove(idx);
    } else {
        error!("{:?} not in the list of clients", stream);
    }
}

fn handle_new_connection(
    connection: TcpStream,
    clients: &mut Vec<TcpStream>,
    client_sender: Sender<Message>,
) -> io::Result<()> {
    info!("new connection: {:?}", connection);

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

        let _ = match message {
            Ok(message) => client_sender
                .send(message)
                .map_err(|error| error!("unable to send message: {}", error)),
            Err(ref error) // e.g. SIGINT
                if error.kind() == ErrorKind::ConnectionReset
                    || error.kind() == ErrorKind::ConnectionAborted =>
            {
                let peer_address = stream.peer_addr().map_err(|error| {
                    error!("cannot obtain peer address (client will not be removed from Vector of clients -> memory leak): {}", error);
                });

                if let Ok(address) = peer_address {
                    let message = Message::quit_signal(Some(address));
                    client_sender.send(message)
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
