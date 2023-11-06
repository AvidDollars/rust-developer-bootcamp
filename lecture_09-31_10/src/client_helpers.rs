use std::net::TcpStream;

pub fn get_peer_address(server: &TcpStream) -> String {
    server
        .peer_addr()
        .map(|address| address.to_string())
        .unwrap_or_else(|_| "CLIENT ERROR: unable to show address".into())
}