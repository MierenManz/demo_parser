pub(crate) struct Frame<'a> {
    server_frame: u64,
    client_frame: u64,
    sub_packet_size: u64,
    packet: Packet<'a>,
}

struct Packet<'a> {
    cmd_type: char,
    unknown: u64,
    tick_count: Option<u64>,
    size_of_packet: u64,
    buff: &'a [u8],
}

impl Frame<'_> {
    pub fn new(data: &[u8]) {}
}

impl Packet<'_> {
    pub fn new(data: &[u8]) {}
}
