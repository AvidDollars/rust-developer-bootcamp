use std::{path::Path, str::FromStr};
use serde::{Deserialize, Serialize};
use serde_cbor::{self, Result as CborResult};
use std::fs;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Text(String),
    Image { name: String, content: Vec<u8> },
    File { name: String, content: Vec<u8> },
}


impl Message {
    pub fn encode(&self) -> CborResult<(u32, Vec<u8>)> {
        //serde_cbor::to_writer(&mut writer, self)?;
        let encoded_content = serde_cbor::to_vec(self)?;
        let encoded_length = encoded_content.len() as u32;
        Ok((encoded_length, encoded_content))
    }

    pub fn decode_from_slice(slice: &[u8]) -> CborResult<Self> {
        let message: Self = serde_cbor::from_slice(slice)?;
        Ok(message)
    }
}

impl TryFrom<String> for Message {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut splitted = value.split(' ');
        let first_arg = splitted.next().expect("will contain &str");
        let second_arg = splitted.next();

        let path = match second_arg {
            // text message
            None => return Ok( Self::Text(first_arg.into())),

            // .file | .image
            Some(value) => value,
        };

        match first_arg {
            type_ @ (".file" | ".image") => {
                let content = fs::read(path)
                    .map_err(|error| error.to_string())?;

                if type_ == ".file" {
                    Ok( Self::File { name: path.into(), content})
                }

                else {
                    Ok( Self::Image { name: path.into(), content })
                }
            },
            _ => Err(format!("Unknown argument '{}'. Allowed are: '.file' | '.image'", first_arg)),
        }        
    }
}