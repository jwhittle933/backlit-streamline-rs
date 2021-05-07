use std::convert::TryInto;
use std::io::{Result, Write};
use super::{Informed, Stringer, Typed};
use super::info::Info;

#[derive(Debug)]
pub struct Ftyp {
    info: Info,
    major_brand: u32,
    minor_version: u32,
    compatible_brands: Vec<[u8; 4]>,
}

impl Informed for Ftyp {}

impl Typed for Ftyp {}

impl Stringer for Ftyp {}

impl Write for Ftyp {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.major_brand = u32::from_be_bytes(buf[0..4].try_into().expect("slice with incorrect length"));
        self.minor_version = u32::from_be_bytes(buf[4..8].try_into().expect(""));

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Ftyp {
    pub fn new(i: Info) -> Self {
        Ftyp {
            info: i,
            major_brand: 0,
            minor_version: 0,
            compatible_brands: Vec::new(),
        }
    }
}
