use super::demo_header::DemoHeader;
use super::frame::Frame;

use std::fs;
use std::io;
struct Demo<'a> {
    header: DemoHeader<'a>,
    frames: Vec<Frame<'a>>,
}

impl Demo<'_> {
    pub fn new<'a, T>(data: T) -> Demo<'a>
    where
        T: Into<&'a [u8]>,
    {
        let buffer = data.into();
        Demo {
            header: DemoHeader::new(buffer),
            frames: vec![],
        }
    }

    pub fn new_from_path<'a, T>(path: T) -> Result<Demo<'a>, io::Error>
    where
        T: Into<String>,
    {
        let buff: &[u8] = &fs::read(path.into())?;
        Ok(Demo::new(buff))
    }
}
