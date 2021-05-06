use crate::media::mp4::atom::{Info, Informed, Stringer, Typed};
use std::convert::TryInto;
use std::io::{Result, Write};

pub struct Ftyp<'a> {
    info: &'a Info,
    major_brand: [u8; 4],
    minor_version: u32,
    compatible_brands: Vec<[u8; 4]>,
}

impl<'a> Informed for Ftyp<'a> {}

impl<'a> Typed for Ftyp<'a> {}

impl<'a> Stringer for Ftyp<'a> {}

impl<'a> Write for Ftyp<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.major_brand = buf[0..4].try_into().expect("slice with incorrect length");
        self.minor_version = u32::from_be_bytes(buf[4..8].try_into().expect(""));

        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<'a> Ftyp<'a> {}
