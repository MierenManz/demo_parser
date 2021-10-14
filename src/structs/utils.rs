use super::super::error::DemoError;
use super::super::error::ErrorType;
use super::super::reader::Reader;
use std::convert::TryInto;

#[inline(always)]
pub fn read_to_le_bytes(r: &mut Reader) -> Result<[u8; 4], DemoError> {
    match r.read::<Vec<u8>>(4).try_into() {
        Ok(v) => Ok(v),
        Err(_) => Err(DemoError::new(
            ErrorType::Parsing,
            "Could not read 4 bytes into array",
        )),
    }
}
