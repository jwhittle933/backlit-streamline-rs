use std::convert::TryInto;
use std::io::{Result, Write};
use std::str;
use super::{Informed, Stringer, Typed, Sized};
use super::info::Info;

#[derive(Debug)]
pub struct Ftyp<'a> {
    pub info: &'a Info,
    pub major_brand: [u8; 4],
    pub minor_version: u32,
    pub compatible_brands: Vec<[u8; 4]>,
    written: bool,
}

impl<'a> Informed for Ftyp<'a> {}

impl<'a> Typed for Ftyp<'a> {
    fn t(self) -> String {
        String::from(self.info.t.string())
    }
}

impl<'a> Stringer for Ftyp<'a> {
    fn string(&self) -> String {
        String::from(
            format!(
                "{}, majorbrand={}, minorversion={}, written={}, compatible_brands={}",
                self.info.string(),
                str::from_utf8(&self.major_brand).expect("failed to convert [u8; 4] to string"),
                self.minor_version,
                self.written,
                vec_to_strings(&self.compatible_brands)
            )
        )
    }
}

impl<'a> Write for Ftyp<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.major_brand = buf[0..4].try_into().expect("could not covert slice to array");
        self.minor_version = u32::from_be_bytes(buf[4..8].try_into().expect("count not convert buffer to u32"));

        self.compatible_brands = Vec::new();

        buf[8..].chunks(4).for_each(|c| {
            let mut copy: [u8; 4] = [0; 4];
            copy.clone_from_slice(&c);
            self.compatible_brands.push(copy);
        });

        self.written = true;

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<'a> Sized for Ftyp<'a> {
    fn size(&self) -> u64 {
        self.info.size
    }
}

impl<'a> Ftyp<'a> {
    pub fn new(i: &'a Info) -> Self {
        Ftyp {
            info: i,
            major_brand: [0; 4],
            minor_version: 0,
            compatible_brands: Vec::new(),
            written: false,
        }
    }
}

fn vec_to_strings(buf: &Vec<[u8; 4]>) -> String {
    let mut out: Vec<&str> = Vec::new();

    buf.into_iter().for_each(|x| {
       out.push(u8_to_string(x));
    });

   out.join(",")
}

fn u8_to_string(buf: &[u8; 4]) -> &str {
    match str::from_utf8(buf) {
        Ok(s)  => s,
        Err(_) => "unknown"
    }
}