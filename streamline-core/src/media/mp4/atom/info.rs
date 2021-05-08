use std::io::{Result, SeekFrom};
use super::{boxtype::BoxType, Stringer};
use crate::io::ReadSeeker;

const SMALL_HEADER: u64 = 4;
const LARGE_HEADER: u64 = 8;

#[derive(Debug)]
pub struct Info {
    offset: u64,
    pub size: u64,
    pub t: BoxType,
    header_size: u64,
    extend_to_eof: bool,
}

impl Stringer for Info {
    fn string(&self) -> String {
        String::from(
            format!(
                "[{}] offset={}, size={}",
                self.t.string(),
                self.offset,
                self.size,
            )
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
            header_size: SMALL_HEADER,
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
            Err(e) => Err(e)
        }
    }

    fn scan_small<T: ReadSeeker>(r: &mut T) -> Result<u64> {
        let mut size = [0; SMALL_HEADER as usize];
        match r.read_exact(&mut size) {
            Ok(_) => Ok(u32::from_be_bytes(size) as u64),
            Err(e) => Err(e)
        }
    }

    fn scan_large<T: ReadSeeker>(r: &mut T) -> Result<u64> {
        let mut size = [0; LARGE_HEADER as usize];
        match r.read_exact(&mut size) {
            Ok(_) => Ok(u64::from_be_bytes(size)),
            Err(e) => Err(e)
        }
    }

    fn scan_name<T: ReadSeeker>(r: &mut T) -> Result<BoxType> {
        let mut name = [0; 4];
        match r.read_exact(&mut name) {
            Ok(_) => Ok(BoxType::new([name[0], name[1], name[2], name[3]])),
            Err(e) => Err(e)
        }
    }
}
