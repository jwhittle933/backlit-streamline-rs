use std::convert::TryInto;
use std::io::{Result, Write};
use super::{Informed, Stringer, Typed};
use super::info::Info;
use crate::io::ReadSeeker;

#[derive(Debug)]
pub struct Ftyp {
    pub info: Info,
    pub major_brand: u32,
    pub minor_version: u32,
    pub compatible_brands: Vec<[u8; 4]>,
    written: bool,
}

impl Informed for Ftyp {}

impl Typed for Ftyp {
    fn t(self) -> String {
        String::from(self.info.t.string())
    }
}

impl Stringer for Ftyp {
    fn string(&self) -> String {
        String::from(
            format!(
                "{}, majorbrand={}, minorversion={}, written={}",
                self.info.string(),
                self.major_brand,
                self.minor_version,
                self.written,
            )
        )
    }
}

impl Write for Ftyp {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.major_brand = u32::from_be_bytes(buf[0..4].try_into().expect("could not convert buffer to u32"));
        self.minor_version = u32::from_be_bytes(buf[4..8].try_into().expect("count not convert buffer to u32"));

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
            written: false,
        }
    }
}
