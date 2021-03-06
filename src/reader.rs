use std::fs;
use std::io::Error as IOError;
use std::path::Path;

pub struct Reader {
    underlying_buffer: Vec<u8>,
}

impl Reader {
    pub fn new<T: Into<Vec<u8>>>(buffer: T) -> Reader {
        Reader {
            underlying_buffer: buffer.into(),
        }
    }

    pub fn new_from_path<T: AsRef<Path>>(path: T) -> Result<Reader, IOError> {
        Ok(Reader {
            underlying_buffer: fs::read(path)?,
        })
    }

    pub fn read<T: From<Vec<u8>>>(&mut self, length: usize) -> T {
        let buff = self
            .underlying_buffer
            .drain(..length)
            .take(length)
            .collect::<Vec<u8>>()
            .into();

        self.underlying_buffer.shrink_to_fit();

        buff
    }

    pub fn read_byte(&mut self) -> u8 {
        let byte = self.underlying_buffer.remove(0);
        self.underlying_buffer.shrink_to_fit();

        byte
    }

    pub fn remove(&mut self, length: usize) {
        if length == 1 {
            self.underlying_buffer.remove(0);
        } else {
            for indx in 0..length {
                self.underlying_buffer.remove(indx);
            }
        }
        self.underlying_buffer.shrink_to_fit();
    }
}
