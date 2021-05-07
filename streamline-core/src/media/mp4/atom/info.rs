use std::io::{Result, SeekFrom};
use super::{boxtype::BoxType, Stringer};
use crate::io::ReadSeeker;

#[derive(Debug)]
pub struct Info {
    offset: u64,
    size: u64,
    pub t: BoxType,
    header_size: u64,
    extend_to_eof: bool,
}

impl Stringer for Info {}

impl Info {
    // TODO: detect 8 bytes or 16 byte size header
    pub fn scan<T: ReadSeeker>(r: &mut T) -> Result<Info> {
        let offset = r.seek(SeekFrom::Current(0))?;

        let mut size = [0; 4];
        r.read_exact(&mut size)?; // handle this error?

        let mut name = [0; 4];
        r.read_exact(&mut name)?; // handle this error?

        Ok(Info{
            offset,
            size: u32::from_be_bytes(size) as u64,
            t: BoxType::new([name[0], name[1], name[2], name[3]]),
            header_size: super::SMALL_HEADER,
            extend_to_eof: false
        })
    }
}
