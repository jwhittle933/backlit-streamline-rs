pub mod boxtype;
pub mod ftyp;
pub mod moov;

use boxtype::BoxType;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::Read;

pub trait Versioned {
    fn write_version(self, src: Vec<u8>) -> usize { src.len() }
}

pub trait Flagged {
    fn flags(self) -> u32 { 0 }
}

pub trait Typed {
    fn t(self) -> String {
        String::from("unknown")
    }
}

pub trait Informed {
    fn info(self) -> &Info {
        &Info{
            offset: 0,
            size: 0,
            t: BoxType::new([0, 0, 0, 0]),
            header_size: 0,
            extend_to_eof: false
        }
    }
}

pub trait Stringer {
    fn string(self) -> String {
        String::from("unimplemented")
    }
}

pub trait Boxed: Typed + Informed + Stringer + io::Write {}
impl<T> Boxed for T where T: Typed + Informed + Stringer + io::Write {}

pub struct Info {
    offset: u64,
    size: u64,
    t: BoxType,
    header_size: u64,
    extend_to_eof: bool,
}

impl Stringer for Info {
    fn string(&self) -> String {
        sprintf(
            "offset={}, size={}, header_size={}, eof={}",
            self.offset,
            self.size,
            self.header_size,
            self.extend_to_eof
        )
    }
}

pub fn scan_info(mut f: &File, info: &Info) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}