use super::super::error::DemoError;
use super::super::error::ErrorType;
use super::super::reader::Reader;

use std::convert::TryInto;
use std::error::Error as StdError;

pub struct Header {
    header: String,
    demo_protocol: u32,
    network_protocol: u32,
    server_name: String,
    client_name: String,
    map_name: String,
    game_directory: String,
    // Time in seconds
    playback_time: f32,
    ticks: u32,
    frames: u32,
    sign_on_length: u32,
}

impl Header {
    pub fn new(mut r: Reader) -> Result<(), Box<dyn StdError>> {
        let bytes: Vec<u8> = r.read(7);
        let header_string = String::from_utf8_lossy(&bytes).to_string();
        if header_string != "HL2DEMO" {
            return Err(Box::new(DemoError::new(
                ErrorType::Header,
                format!("Invalid header {}", header_string),
            )));
        }
        let f: [u8; 4] = match r.read::<Vec<u8>>(4).try_into() {
            Ok(v) => v,
            Err(_) => {
                return Err(Box::new(DemoError::new(
                    ErrorType::Parsing,
                    "Could not parse [u8; 4] from bytes",
                )))
            }
        };
        println!("{:?}", f);
        // let header = Header {
        //   header: header_string,
        //   demo_protocol: u32::from_le_bytes(r.read::<Vec<u8>>(4).try_into())
        // }
        Ok(())
    }
}
