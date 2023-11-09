use std::net::{TcpStream, Ipv4Addr};
use std::{path::Path, str::FromStr};
use serde::{Deserialize, Serialize};
use serde_cbor::{self, Result as CborResult};
use std::fs;
use std::io::{self, ErrorKind};
use std::result::Result;
use crate::helpers::*;
use std::io::{prelude::*, BufReader};
use std::error::Error;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Message {
    Text { content: String, sender: Option<Ipv4Addr> },
    Image { name: String, content: Vec<u8>, sender: Option<Ipv4Addr> },
    File { name: String, content: Vec<u8>, sender: Option<Ipv4Addr> },
}

impl Message {

    // dunno... I guess totally retarded approach but I couldn't come up with anything better
    pub fn set_missing_ip(&mut self, ip: Ipv4Addr) {
        match self {
            Self::Text { sender, content } => {
                if sender.is_none() {
                    *self = Self::Text { content: content.to_string(), sender: Some(ip) } ;
                }
            },
            Self::Image { name, content, sender } => {
                if sender.is_none() {
                    *self = Self::Image { name: name.to_string(), content: content.to_vec(), sender: Some(ip) }
                }
            },
            Self::File { name, content, sender } => {
                if sender.is_none() {
                    *self = Self::Image { name: name.to_string(), content: content.to_vec(), sender: Some(ip) }
                }
            },
        }
    }

    pub fn encode(&self) -> CborResult<(u32, Vec<u8>)> {
        //serde_cbor::to_writer(&mut writer, self)?;
        let encoded_content = serde_cbor::to_vec(self)?;
        let encoded_length = encoded_content.len() as u32;
        Ok((encoded_length, encoded_content))
    }

    // probably redundant
    pub fn to_writer(&self, mut writer: impl io::Write) -> CborResult<()> {
        let encoded_content = serde_cbor::to_vec(&self)?;
        let encoded_length = encoded_content.len() as u32;
        serde_cbor::to_writer(&mut writer, &encoded_length)?;
        serde_cbor::to_writer(&mut writer, &encoded_content)?;
        Ok(())
    }

    pub fn decode_from_slice(slice: &[u8]) -> CborResult<Self> {
        let message: Self = serde_cbor::from_slice(slice)?;
        Ok(message)
    }
}


// From CLI input
impl TryFrom<String> for Message {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut splitted = value.split(' ');
        let first_arg = splitted.next().expect("will contain &str");
        let second_arg = splitted.next();

        let path = match second_arg {
            // text message
            None => return Ok( Self::Text{ content: first_arg.into(), sender: None}),

            // .file | .image
            Some(value) => value.trim(),
        };

        match first_arg {
            type_ @ (".file" | ".image") => {
                let content = fs::read(path)
                    .map_err(|error| error.to_string())?;

                if type_ == ".file" {
                    Ok( Self::File { name: path.into(), content, sender: None})
                }

                else {
                    Ok( Self::Image { name: path.into(), content, sender: None })
                }
            },
            _ => Err(format!("Unknown argument '{}'. Allowed are: '.file' | '.image'", first_arg)),
        }        
    }
}

impl TryFrom<&TcpStream> for Message {
    type Error = Box<dyn Error>;

    fn try_from(stream: &TcpStream) -> Result<Self, Box<dyn Error>> {
        let mut len_bytes = [0_u8; 4];
        let mut content_len = 0;
        let mut content_populated = 0;
        let mut index = 0;
        let mut content = vec![];

        for client_stream in buffered!(stream).bytes() {
            match client_stream {

                Ok(byte) => {

                    if index < 4 {
                        len_bytes[index] = byte;
                        index += 1;
                        
                    }

                    else if index == 4 && content_len == 0 {
                        content_len = u32::from_be_bytes(len_bytes) as usize;
                        content = Vec::with_capacity(content_len);
                        content.push(byte);
                        content_populated += 1;
                    }

                    else if content_populated  < content_len - 1 {
                            content.push(byte);
                            content_populated += 1;
                    }

                    // last iteration
                    else {
                        content.push(byte);
                        break;
                    }
                },
                
                Err(error) => {
                    match error.kind() {
                        ErrorKind::ConnectionReset => return Err(Box::new(error)),
                        ErrorKind::WouldBlock => continue,
                        error => {
                            eprintln!("Error during reading stream: {error}");
                            continue; // ?
                        },
                    }
                }
            };
        }

        let message = Message::decode_from_slice(&content)?;
        return Ok(message) 
    }
}