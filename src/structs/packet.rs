use super::utils::read_to_le_bytes;

use crate::reader::Reader;

use std::convert::TryInto;
use std::error::Error as StdError;

pub struct Packet {
    cmd_type: char,
    unknown: i32,
    tick_count: i32,
    size_of_packet: i32,
    reader: Reader,
}

impl Packet {
    pub fn new(r: &mut Reader) -> Result<Packet, Box<dyn StdError>> {
        let cmd_type = r.read_byte() as char;
        let unknown = i32::from_le_bytes(read_to_le_bytes(r)?);
        let tick_count = i32::from_le_bytes(read_to_le_bytes(r)?);
        let size_of_packet = i32::from_le_bytes(read_to_le_bytes(r)?);
        let reader = Reader::new(r.read::<Vec<u8>>(size_of_packet.try_into()?));

        Ok(Packet {
            cmd_type,
            unknown,
            tick_count,
            size_of_packet,
            reader,
        })
    }

    pub fn cmd_type(self) -> char {
        self.cmd_type
    }

    pub fn unknown(self) -> i32 {
        self.unknown
    }

    pub fn tick_count(self) -> i32 {
        self.tick_count
    }

    pub fn size_of_packet(self) -> i32 {
        self.size_of_packet
    }

    pub fn reader(self) -> Reader {
        self.reader
    }
}
