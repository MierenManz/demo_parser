use demo_parser::reader::Reader;
use demo_parser::structs::header::Header;
#[test]
fn new() {
    let mut buff: Vec<u8> = b"HL2DEMO".to_vec();
    let mut le: Vec<u8> = 32_u32.to_le_bytes().to_vec();
    buff.append(&mut le);
    let reader = Reader::new(buff);
    Header::new(reader).unwrap();
}
