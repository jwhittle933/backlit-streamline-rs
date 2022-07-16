use crate::{
    io::Sized,
    media::mp4::{atom::Typed, Info},
};
use std::fmt;
use std::io::{Result, Write};

/// ISO BMFF free box
#[derive(Debug)]
pub struct Free {
    pub info: Info,
    pub data: Vec<u8>,
}

impl Typed for Free {
    fn t(&self) -> String {
        String::from("ftyp")
    }
}

impl fmt::Display for Free {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[free] {}, data={:?}", self.info, self.data)
    }
}

impl Write for Free {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Sized for Free {
    fn size(&self) -> u64 {
        self.info.size
    }
}

impl Free {
    pub fn new(i: Info) -> Self {
        Free {
            info: i,
            data: Vec::new(),
        }
    }
}
