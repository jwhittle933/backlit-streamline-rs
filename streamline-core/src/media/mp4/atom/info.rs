use std::io::{Result, SeekFrom};
use super::{boxtype::BoxType, Stringer};
use crate::io::ReadWriteSeeker;

pub struct Info {
    offset: u64,
    size: u64,
    t: BoxType,
    header_size: u64,
    extend_to_eof: bool,
}

impl Stringer for Info {}

impl Info {
    pub fn scan<T: ReadWriteSeeker>(mut r: T) -> Result<()> {
        let off = r.seek(SeekFrom::Current(0));
        Ok(())
    }
}
