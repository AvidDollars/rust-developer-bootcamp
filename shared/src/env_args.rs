use clap::{arg, command, value_parser};

use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Debug)]
pub struct EnvArgs {
    pub mode: String,
    pub host: Ipv4Addr,
    pub port: u16,
}

impl Into<SocketAddrV4> for EnvArgs {
    fn into(self) -> SocketAddrV4 {
        SocketAddrV4::new(self.host, self.port)
    }
}

impl EnvArgs {
    pub fn is_server(&self) -> bool {
        self.mode == "server"
    }
}

impl EnvArgs {
    pub fn new() -> Result<Self, String> {
        let matches = command!()
            .about("Client-Server chat application for broadcasting messages, files & images.")
            .arg(
                arg!(mode: "Mode of operation: server (default) or client")
                    .short('m')
                    .long("mode")
                    .value_parser(["server", "client"])
                    .default_value("server"),
            )
            .arg(
                arg!(host: "IPv4 address of the host (default: 127.0.0.1)")
                    .short('o')
                    .long("host")
                    .value_parser(value_parser!(Ipv4Addr))
                    .default_value("127.0.0.1"),
            )
            .arg(
                arg!(port: "Specifies a port (default: 11111)")
                    .short('p')
                    .long("port")
                    .value_parser(value_parser!(u16))
                    .default_value("11111"),
            )
            .get_matches();

        Ok(Self {
            mode: matches
                .get_one::<String>("mode")
                .expect("provided")
                .to_string(),
            host: *matches.get_one::<Ipv4Addr>("host").expect("provided"),
            port: *matches.get_one::<u16>("port").expect("provided"),
        })
    }
}
