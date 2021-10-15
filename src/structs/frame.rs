use super::packet::Packet;
use super::utils::read_to_le_bytes;

use crate::reader::Reader;

use std::convert::TryInto;
use std::error::Error as StdError;

pub struct Frame {
    server_frame: i32,
    client_frame: i32,
    sub_packet_size: i32,
    // Reader has size of `sub_packet_size`
    reader: Reader,
    packet: Packet,
}

impl Frame {
    pub fn new(r: &mut Reader) -> Result<Frame, Box<dyn StdError>> {
        let server_frame = i32::from_le_bytes(read_to_le_bytes(r)?);
        let client_frame = i32::from_le_bytes(read_to_le_bytes(r)?);
        let sub_packet_size = i32::from_le_bytes(read_to_le_bytes(r)?);
        let sub_packet_usize: usize = sub_packet_size.try_into()?;
        let reader = Reader::new(r.read::<Vec<u8>>(sub_packet_usize));
        let packet = Packet::new(r)?;
        // Remove 40 null bytes
        r.remove(40);

        Ok(Frame {
            server_frame,
            client_frame,
            sub_packet_size,
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

    pub fn sub_packet_size(self) -> i32 {
        self.sub_packet_size
    }

    pub fn reader(self) -> Reader {
        self.reader
    }

    pub fn packet(self) -> Packet {
        self.packet
    }
}
