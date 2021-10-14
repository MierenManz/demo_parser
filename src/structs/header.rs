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
    pub fn new(mut r: Reader) -> Result<Header, Box<dyn StdError>> {
        let bytes: Vec<u8> = r.read(7);
        let header_string = String::from_utf8_lossy(&bytes).to_string();
        if header_string != "HL2DEMO" {
            return Err(Box::new(DemoError::new(
                ErrorType::Header,
                format!("Invalid header {}", header_string),
            )));
        }
        // Removes the null byte
        r.remove(1);

        Ok(Header {
            header: header_string,
            demo_protocol: u32::from_le_bytes(read_to_le_bytes(&mut r)?),
            network_protocol: u32::from_le_bytes(read_to_le_bytes(&mut r)?),
            server_name: String::from_utf8(r.read(260))?,
            client_name: String::from_utf8(r.read(260))?,
            map_name: String::from_utf8(r.read(260))?,
            game_directory: String::from_utf8(r.read(260))?,
            // Time in seconds
            playback_time: f32::from_le_bytes(read_to_le_bytes(&mut r)?),
            ticks: u32::from_le_bytes(read_to_le_bytes(&mut r)?),
            frames: u32::from_le_bytes(read_to_le_bytes(&mut r)?),
            sign_on_length: u32::from_le_bytes(read_to_le_bytes(&mut r)?),
        })
    }

    #[inline(always)]
    pub fn header(self) -> String {
        self.header
    }

    #[inline(always)]
    pub fn demo_protocol(self) -> u32 {
        self.demo_protocol
    }

    #[inline(always)]
    pub fn network_protocol(self) -> u32 {
        self.network_protocol
    }

    #[inline(always)]
    pub fn server_name(self) -> String {
        self.server_name
    }

    #[inline(always)]
    pub fn client_name(self) -> String {
        self.client_name
    }

    #[inline(always)]
    pub fn map_name(self) -> String {
        self.map_name
    }

    #[inline(always)]
    pub fn game_directory(self) -> String {
        self.game_directory
    }

    #[inline(always)]
    pub fn playback_time(self) -> f32 {
        self.playback_time
    }

    #[inline(always)]
    pub fn ticks(self) -> u32 {
        self.ticks
    }

    #[inline(always)]
    pub fn frames(self) -> u32 {
        self.frames
    }

    #[inline(always)]
    pub fn sign_on_length(self) -> u32 {
        self.sign_on_length
    }
}

#[inline(always)]
fn read_to_le_bytes(r: &mut Reader) -> Result<[u8; 4], DemoError> {
    match r.read::<Vec<u8>>(4).try_into() {
        Ok(v) => Ok(v),
        Err(_) => Err(DemoError::new(
            ErrorType::Parsing,
            "Could not read 4 bytes into array",
        )),
    }
}
