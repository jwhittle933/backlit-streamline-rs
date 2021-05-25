use std::io::{Result, Write};
use super::{Stringer, Typed};
use super::info::Info;
use crate::io::Sized;

#[derive(Debug)]
pub struct Free {
    offset: u64,
    size: u64,
}

impl Typed for Free {
    fn t(self) -> String {
        String::from("ftyp")
    }
}

impl Stringer for Free {
    fn string(&self) -> String {
        String::from(
            format!(
                "[free]",
            )
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
        }
    }
}
