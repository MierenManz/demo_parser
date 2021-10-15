use super::frame::Frame;
use super::header::Header;

use crate::error::DemoError;
use crate::error::ErrorType;

use crate::reader::Reader;
use std::convert::TryInto;
use std::error::Error as StdError;

pub struct Demo {
    header: Header,
    frames: Vec<Frame>,
}

impl Demo {
    pub fn new(r: &mut Reader) -> Result<Demo, Box<dyn StdError>> {
        let header = Header::new(r)?;

        let frame_count: usize = match header.frames().try_into() {
            Ok(v) => v,
            Err(_) => {
                return Err(Box::new(DemoError::new(
                    ErrorType::Parsing,
                    "could not parse frame count",
                )))
            }
        };

        let mut frames: Vec<Frame> = Vec::new();
        for f in 0..frame_count {
            frames.push(Frame::new(r)?);
        }

        Ok(Demo { header, frames })
    }

    pub fn header(self) -> Header {
        self.header
    }

    // pub fn frames(self) -> Vec<Frame> {
    //     self.frames
    // }
}
