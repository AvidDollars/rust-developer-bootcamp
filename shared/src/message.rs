use crate::constants::{FILES_FOLDER, IMAGES_FOLDER};
use crate::errors::AppError;
use crate::utils::*;

use image::io::Reader as ImageReader;
use image::ImageOutputFormat;
use serde::{Deserialize, Serialize};
pub use serde_cbor::{self, Result as CborResult};

use std::fmt::{self, Debug, Display};
use std::fs;
use std::io::Cursor;
use std::io::{self, prelude::*, ErrorKind};
use std::net::{SocketAddr, TcpStream};
use std::result::Result;
use std::str;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub type_: MessageType,
    sender: Option<SocketAddr>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageType {
    Text { content: String },
    Image { name: String, content: Vec<u8> },
    File { name: String, content: Vec<u8> },
    QuitSignal,
    NewConnection,
}

impl Display for Message {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use MessageType::*;
        let sender = self
            .sender
            .map(|address| address.to_string())
            .unwrap_or_else(|| "unknown".into());

        match &self.type_ {
            Text { content } => {
                write!(formatter, "{}", format!("[{}]: {}", sender, content))
            }
            Image { name, .. } | File { name, .. } => {
                write!(formatter, "[{}]: receiving {}", sender, name)
            }
            QuitSignal => write!(formatter, "[DISCONNECTED]: {}", sender),
            NewConnection => write!(formatter, "[CONNECTED]: {}", sender),
        }
    }
}

impl Debug for Message {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use MessageType::*;

        match &self.type_ {
            Text { content } => {
                write!(formatter, "{}", format!("{:?}: {}", self.sender, content))
            }
            Image { name, content } | File { name, content } => {
                let content_len = content.len();
                let len = if content_len < 10 { content_len } else { 10 };
                write!(
                    formatter,
                    "{:?}: name: {}, content: {:?}",
                    self.sender,
                    name,
                    &content[..len]
                )
            }
            QuitSignal => write!(formatter, "{:?} is quitting", self.sender),
            NewConnection => write!(formatter, "{:?} is now connected", self.sender),
        }
    }
}

impl Message {
    pub fn quit_signal(sender: Option<SocketAddr>) -> Self {
        Self {
            type_: MessageType::QuitSignal,
            sender,
        }
    }

    pub fn new_connection(sender: Option<SocketAddr>) -> Self {
        Self {
            type_: MessageType::NewConnection,
            sender,
        }
    }

    pub fn is_empty(&self) -> bool {
        use MessageType::*;

        match &self.type_ {
            Text { content } => content.trim().is_empty(),
            Image { content, .. } | File { content, .. } => content.is_empty(),
            QuitSignal | NewConnection => false,
        }
    }

    pub fn is_quit(&self) -> bool {
        self.type_ == MessageType::QuitSignal
    }

    pub fn set_sender(&mut self, address: SocketAddr) {
        self.sender = Some(address)
    }

    pub fn get_sender(&self) -> &Option<SocketAddr> {
        &self.sender
    }

    pub fn has_sender(&self) -> bool {
        self.sender.is_some()
    }

    pub fn is_image_and_png(&self) -> Result<bool, AppError> {
        let png_header = [137_u8, 80, 78, 71, 13, 10, 26, 10];
        use MessageType::*;

        match &self.type_ {
            Image { content, .. } => Ok(content[..png_header.len()] == png_header),
            _ => Err(AppError::UnsupportedConversion),
        }
    }

    pub fn convert_image_to_png(&mut self) -> Result<(), AppError> {
        use MessageType::*;

        if let Image {
            ref mut content, ..
        } = self.type_
        {
            let img = ImageReader::new(Cursor::new(&content))
                .with_guessed_format()?
                .decode()?;

            img.write_to(&mut Cursor::new(content), ImageOutputFormat::Png)?;
            Ok(())
        } else {
            Err(AppError::UnsupportedConversion)
        }
    }

    pub fn save_file(&self) -> Result<(), AppError> {
        use AppError::*;
        use MessageType::*;

        match &self.type_ {
            // only images & files will be saved
            Text { .. } | NewConnection | QuitSignal => return Err(UnsupportedConversion),
            Image { name, content } => {
                let mut file = fs::File::create(format!("./{}/{}", IMAGES_FOLDER, name))?;
                file.write_all(&content)?;
                Ok(())
            }
            File { name, content } => {
                let mut file = fs::File::create(format!("./{}/{}", FILES_FOLDER, name))?; // if file exists it will be overwritten
                file.write_all(content)?;
                Ok(())
            }
        }
    }
}

impl Message {
    pub fn encode(&self) -> CborResult<(u32, Vec<u8>)> {
        let encoded_content = serde_cbor::to_vec(self)?;
        let encoded_length = encoded_content.len() as u32;
        Ok((encoded_length, encoded_content))
    }

    pub fn decode_from_slice(slice: &[u8]) -> CborResult<Self> {
        let message: Self = serde_cbor::from_slice(slice)?;
        Ok(message)
    }
}

// try to create Message from CLI input
impl TryFrom<&str> for Message {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut splitted = value.split(' ');
        let first_arg = splitted.next().expect("will contain &str").trim();

        if first_arg == ".quit" {
            return Ok(Message {
                type_: MessageType::QuitSignal,
                sender: None,
            });
        }

        let second_arg = splitted.next();

        let path = match second_arg {
            // text message
            None => {
                let type_ = MessageType::Text {
                    content: first_arg.into(),
                };
                return Ok(Self {
                    type_,
                    sender: None,
                });
            }

            // .file | .image
            Some(value) => value.trim(),
        };

        match first_arg {
            type_ @ (".file" | ".image") => {
                let content = fs::read(path).map_err(|error| error.to_string())?;

                if type_ == ".file" {
                    let name = path.to_string();
                    let type_ = MessageType::File { name, content };
                    Ok(Self {
                        type_,
                        sender: None,
                    })
                } else {
                    let name = current_timestamp().unwrap_or_else(|_| path.into()) + ".png"; // if timestamp cannot be used as a name, use path
                    let type_ = MessageType::Image { name, content };
                    Ok(Self {
                        type_,
                        sender: None,
                    })
                }
            }
            _ => {
                let type_ = MessageType::Text {
                    content: format!("{} {} {}", first_arg, path, splitted.collect::<String>()),
                };
                return Ok(Self {
                    type_,
                    sender: None,
                });
            }
        }
    }
}

impl TryFrom<&mut TcpStream> for Message {
    type Error = io::Error;

    // retarded, but couldn't do any better... at least it works xD
    fn try_from(stream: &mut TcpStream) -> Result<Self, io::Error> {
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
                    } else if index == 4 && content_len == 0 {
                        content_len = u32::from_be_bytes(len_bytes) as usize;
                        content = Vec::with_capacity(content_len);
                        content.push(byte);
                        content_populated += 1;
                    } else if content_populated < content_len - 1 {
                        content.push(byte);
                        content_populated += 1;
                    }
                    // last iteration
                    else {
                        content.push(byte);
                        break;
                    }
                }

                Err(error) => match error.kind() {
                    ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted => return Err(error),
                    ErrorKind::WouldBlock => continue,
                    error => {
                        eprintln!("Error during reading stream: {error}");
                        continue;
                    }
                },
            };
        }

        let message = Message::decode_from_slice(&content)
            .map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?;

        return Ok(message);
    }
}
