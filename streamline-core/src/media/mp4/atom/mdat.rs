use crate::media::mp4::Info;
use std::io::{Result, Write};

/// ISO BMFF mdat box type
#[derive(Debug)]
pub struct Mdat {
    pub info: Info,
    pub data: Vec<u8>,
}

impl std::fmt::Display for Mdat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, datalength={}", self.info, self.data.len())
    }
}

impl Write for Mdat {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
