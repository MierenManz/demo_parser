use super::packet::Packet;
use super::utils::read_to_le_bytes;

use crate::reader::Reader;

use std::convert::TryInto;
use std::error::Error as StdError;

pub struct Frame {
    server_frame: i32,
    client_frame: i32,
    size_of_packet: i32,
    // Reader has size of `size_of_packet`
    reader: Reader,
    packet: Packet,
}

impl Frame {
    pub fn new(r: &mut Reader) -> Result<Frame, Box<dyn StdError>> {
        let server_frame = i32::from_le_bytes(read_to_le_bytes(r)?);
        let client_frame = i32::from_le_bytes(read_to_le_bytes(r)?);
        let size_of_packet = i32::from_le_bytes(read_to_le_bytes(r)?);
        let reader = Reader::new(r.read::<Vec<u8>>(size_of_packet.try_into()?));
        let packet = Packet::new(r)?;
        // Remove 40 null bytes
        r.remove(40);

        Ok(Frame {
            server_frame,
            client_frame,
            size_of_packet,
            reader,
            packet,
        })
    }

    pub fn server_frame(self) -> i32 {
        self.server_frame
    }

    pub fn client_frame(self) -> i32 {
        self.client_frame
    }

    pub fn size_of_packet(self) -> i32 {
        self.size_of_packet
    }

    pub fn reader(self) -> Reader {
        self.reader
    }

    pub fn packet(self) -> Packet {
        self.packet
    }
}
