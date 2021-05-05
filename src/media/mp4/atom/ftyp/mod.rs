use crate::media::mp4::atom::{Boxed, Typed, Informed, Stringer, Info};
use std::io::{Write, Result};

pub struct Ftyp<'a> {
    info: &'a Info,
    major_brand: [u8; 4],
    minor_version: u32,
    compatible_brands: Vec<Vec<[u8; 4]>>,
}

impl Informed for Ftyp<'_> {
    fn info(self) -> &Info { self.info }
}

impl Typed for Ftyp<'_> {
    fn t(self) -> String { String::from("ftyp") }
}

impl Stringer for Ftyp<'_> {
    fn string(self) -> String {
        sprintf("{}", self.info.string())
    }
}

impl Write for Ftyp<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // self.major_brand = buf[0..4];
        return Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

