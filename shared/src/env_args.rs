use clap::{arg, command, value_parser};
use tracing;

use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

#[derive(Debug)]
pub struct EnvArgs {
    pub mode: String,
    pub host: Ipv4Addr,
    pub port: u16,
    pub log_level: tracing::Level,
}

impl From<EnvArgs> for SocketAddrV4 {
    fn from(env_args: EnvArgs) -> Self {
        SocketAddrV4::new(env_args.host, env_args.port)
    }
}

impl EnvArgs {
    pub fn is_server(&self) -> bool {
        self.mode == "server"
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl EnvArgs {
    pub fn new() -> Self {
        let matches = command!()
            .about("Client-Server chat application for broadcasting messages, files & images.")
            .arg(
                arg!(mode: "Mode of operation: server or client")
                    .short('m')
                    .long("mode")
                    .value_parser(["server", "client"])
                    .default_value("server"),
            )
            .arg(
                arg!(host: "IPv4 address of the host")
                    .short('o')
                    .long("host")
                    .value_parser(value_parser!(Ipv4Addr))
                    .default_value("127.0.0.1"),
            )
            .arg(
                arg!(port: "Specifies a port")
                    .short('p')
                    .long("port")
                    .value_parser(value_parser!(u16))
                    .default_value("11111"),
            )
            .arg(
                arg!(log_level: "Specifies a log level")
                    .short('l')
                    .long("log-level")
                    .value_parser(clap::builder::ValueParser::new(tracing::Level::from_str))
                    .default_value("info"),
            )
            .get_matches();

        Self {
            mode: matches
                .get_one::<String>("mode")
                .expect("provided")
                .to_string(),
            host: *matches.get_one::<Ipv4Addr>("host").expect("provided"),
            port: *matches.get_one::<u16>("port").expect("provided"),
            log_level: *matches
                .get_one::<tracing::Level>("log_level")
                .expect("provided"),
        }
    }
}
