use super::utils::read_to_le_bytes;

use crate::error::DemoError;
use crate::error::ErrorType;
use crate::reader::Reader;

#[derive(Debug)]
pub struct Header {
    header: String,
    demo_protocol: i32,
    network_protocol: i32,
    server_name: String,
    client_name: String,
    map_name: String,
    game_directory: String,
    // Time in seconds
    playback_time: f32,
    ticks: i32,
    frames: i32,
    sign_on_length: i32,
}

impl Header {
    pub fn new(r: &mut Reader) -> Result<Header, DemoError> {
        let bytes: Vec<u8> = r.read(7);
        let header_string = String::from_utf8_lossy(&bytes).to_string();
        if header_string != "HL2DEMO" {
            return Err(DemoError::new(
                ErrorType::Header,
                format!("Invalid header {}", header_string),
            ));
        }
        println!("{}", header_string);
        // Removes the null byte
        r.remove(1);
        let demo_protocol = i32::from_le_bytes(read_to_le_bytes(r)?);
        let network_protocol = i32::from_le_bytes(read_to_le_bytes(r)?);
        let server_name = String::from_utf8_lossy(r.read::<Vec<u8>>(260).as_ref()).to_string();
        let client_name = String::from_utf8_lossy(r.read::<Vec<u8>>(260).as_ref()).to_string();
        let map_name = String::from_utf8_lossy(r.read::<Vec<u8>>(260).as_ref()).to_string();
        let game_directory = String::from_utf8_lossy(r.read::<Vec<u8>>(260).as_ref()).to_string();
        let playback_time = f32::from_le_bytes(read_to_le_bytes(r)?);
        let ticks = i32::from_le_bytes(read_to_le_bytes(r)?);
        let frames = i32::from_le_bytes(read_to_le_bytes(r)?);
        let sign_on_length = i32::from_le_bytes(read_to_le_bytes(r)?);
        Ok(Header {
            header: header_string,
            demo_protocol,
            network_protocol,
            server_name,
            client_name,
            map_name,
            game_directory,
            // Time in seconds
            playback_time,
            ticks,
            frames,
            sign_on_length,
        })
    }

    #[inline(always)]
    pub fn header(&self) -> String {
        (*self.header).to_string()
    }

    #[inline(always)]
    pub fn demo_protocol(&self) -> i32 {
        self.demo_protocol
    }

    #[inline(always)]
    pub fn network_protocol(&self) -> i32 {
        self.network_protocol
    }

    #[inline(always)]
    pub fn server_name(&self) -> String {
        (*self.server_name).to_string()
    }

    #[inline(always)]
    pub fn client_name(&self) -> String {
        (*self.client_name).to_string()
    }

    #[inline(always)]
    pub fn map_name(&self) -> String {
        (*self.map_name).to_string()
    }

    #[inline(always)]
    pub fn game_directory(&self) -> String {
        (*self.game_directory).to_string()
    }

    #[inline(always)]
    pub fn playback_time(&self) -> f32 {
        self.playback_time
    }

    #[inline(always)]
    pub fn ticks(&self) -> i32 {
        self.ticks
    }

    #[inline(always)]
    pub fn frames(&self) -> i32 {
        self.frames
    }

    #[inline(always)]
    pub fn sign_on_length(&self) -> i32 {
        self.sign_on_length
    }
}
