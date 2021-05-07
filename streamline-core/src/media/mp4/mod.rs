pub mod atom;

use std::fs::File;
use std::io::{Read, Write, Seek, BufReader, Result};
use crate::io::{ReadWriteSeeker, read_write_seeker};

pub struct MP4<T: ReadWriteSeeker> {
    r: BufReader<T>,
}

impl<T: ReadWriteSeeker> MP4<T> {
    pub fn new(r: T) -> MP4<T> {
        MP4 {
            r: BufReader::new(r),
        }
    }

    pub fn from_file(path: &str) -> Result<MP4<File>> {
        match File::open(path) {
            Ok(f) => Ok(MP4 {
                r: BufReader::new(f),
            }),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
