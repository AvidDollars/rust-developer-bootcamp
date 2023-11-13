use crate::config::{DEFAULT_HOST, DEFAULT_PORT};

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
        let mut cli_args = env::args();
        cli_args.next(); // ignoring path of the executable

        let (mode, host, port);

        match cli_args.next() {
            None => mode = "server".to_string(),
            Some(value) => match &value[..] {
                "-s" | "--server" => mode = "server".into(),
                "-c" | "--client" => mode = "client".into(),
                _ => {
                    return Err(format!(
                        "Invalid mode '{value}'. Allowed are: '--server (-s)' | '--client (-c)'."
                    ))
                }
            },
        }

        match cli_args.next() {
            None => host = Ipv4Addr::from(DEFAULT_HOST),
            Some(ref value) => {
                host = value.parse::<Ipv4Addr>().map_err(|_error| {
                    format!("invalid address '{value}'. Use proper IPv4 address.")
                })?;
            }
        }

        match cli_args.next() {
            None => port = DEFAULT_PORT,
            Some(value) => {
                port = value.parse::<u16>().map_err(|_error| {
                    format!(
                        "invalid port '{value}'. Use port between {0} - {}",
                        u16::MAX
                    )
                })?;
            }
        }

        Ok(Self { mode, host, port })
    }
}
