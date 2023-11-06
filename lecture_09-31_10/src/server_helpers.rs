use std::net::TcpListener;

pub fn get_server_address(server: &TcpListener) -> String {
    server
        .local_addr()
        .map(|address| address.to_string())
        .unwrap_or_else(|_| "SERVER ERROR: unable to show address".into())
}
