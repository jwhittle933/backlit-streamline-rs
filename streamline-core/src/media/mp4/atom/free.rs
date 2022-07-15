use super::info::Info;
use super::Typed;
use crate::io::Sized;
use std::fmt;
use std::io::{Result, Write};

#[derive(Debug)]
pub struct Free {
    pub offset: u64,
    pub size: u64,
    pub data: Vec<u8>,
}

impl Typed for Free {
    fn t(&self) -> String {
        String::from("ftyp")
    }
}

impl fmt::Display for Free {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[free] offset={}, size={}, data={:?}",
            self.offset, self.size, self.data
        )
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
        self.size
    }
}

impl Free {
    pub fn new(i: Info) -> Self {
        Free {
            offset: i.offset,
            size: i.size - 8,
            data: Vec::new(),
        }
    }
}
