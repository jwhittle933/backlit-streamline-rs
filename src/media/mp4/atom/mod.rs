pub mod boxtype;
pub mod ftyp;
pub mod moov;

use boxtype::BoxType;
use std::io::{Result, Write};

const SMALL_HEADER: u64 = 8;
const LARGE_HEADER: u64 = 16;

pub trait Versioned {}

pub trait Flagged {}

pub trait Typed {}

pub trait Informed {}

pub trait Stringer {}

pub trait Boxed: Typed + Informed + Stringer + Write {}
impl<T> Boxed for T where T: Typed + Informed + Stringer + Write {}

pub struct Info {
    offset: u64,
    size: u64,
    t: BoxType,
    header_size: u64,
    extend_to_eof: bool,
}

impl Stringer for Info {}

pub fn scan_info(mut r: std::io::Cursor<&[u8]>) -> Result<()> {
    Ok(())
}
