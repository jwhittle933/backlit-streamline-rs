use super::boxtype::BoxType;
use crate::io::ReadSeeker;
use std::fmt;
use std::io::{Result, SeekFrom};

pub const SMALL_HEADER: u64 = 8;
pub const LARGE_HEADER: u64 = 16;

#[derive(Clone, Copy, Debug)]
pub struct Info {
    pub offset: u64,
    pub size: u64,
    pub t: BoxType,
    pub header_size: u64,
    extend_to_eof: bool,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] offset={}, size={}",
            self.t.string(),
            self.offset,
            self.size,
        )
    }
}

impl Info {
    pub fn scan<T: ReadSeeker>(mut r: &mut T) -> Result<Info> {
        let offset = r.seek(SeekFrom::Current(0))?;
        let size = Self::scan_size(&mut r)?;
        let name = Self::scan_name(&mut r)?;

        Ok(Info {
            offset,
            size,
            t: name,
            header_size: SMALL_HEADER, // this may be LARGE_HEADER
            extend_to_eof: false,
        })
    }

    pub fn seek_payload<T: ReadSeeker>(&self, r: &mut T) -> Result<u64> {
        r.seek(SeekFrom::Start(self.offset))
    }

    fn scan_size<T: ReadSeeker>(mut r: &mut T) -> Result<u64> {
        match Self::scan_small(&mut r) {
            Ok(0) => Self::scan_large(&mut r),
            Ok(size) => Ok(size),
            Err(e) => Err(e),
        }
    }

    fn scan_small<T: ReadSeeker>(r: &mut T) -> Result<u64> {
        let mut size = [0; 4];
        r.read_exact(&mut size)
            .map(|_op| u32::from_be_bytes(size) as u64)
    }

    fn scan_large<T: ReadSeeker>(r: &mut T) -> Result<u64> {
        let mut size = [0; 8];
        r.read_exact(&mut size).map(|_op| u64::from_be_bytes(size))
    }

    fn scan_name<T: ReadSeeker>(r: &mut T) -> Result<BoxType> {
        let mut name = [0; 4];
        r.read_exact(&mut name)
            .map(|_op| BoxType::new([name[0], name[1], name[2], name[3]]))
    }
}

pub trait FromInfo {
    fn from_info(i: Info) -> Self;
}
