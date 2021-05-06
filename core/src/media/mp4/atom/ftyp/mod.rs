use crate::media::mp4::atom::{Boxed, Typed, Informed, Stringer, Info};
use std::io::{Write, Result};
use std::convert::TryInto;

pub struct Ftyp<'a> {
    info: &'a Info,
    major_brand: [u8; 4],
    minor_version: u32,
    compatible_brands: Vec<[u8; 4]>,
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
        self.major_brand = buf[0..4].try_into().expect("slice with incorrect length");
        self.minor_version = buf[4..8].to_be_bytes();

        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<'a> Ftyp {
    pub fn new(info: &'a Info) -> &Self {
        &Ftyp{info, major_brand: [0, 0, 0, 0], minor_version: 0, compatible_brands: Vec::new() }
    }
}


