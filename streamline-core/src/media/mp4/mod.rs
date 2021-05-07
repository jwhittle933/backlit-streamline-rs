pub mod atom;

use std::fs::File;
use std::io::{BufReader, Result, SeekFrom, Read, Write, Seek};
use crate::io::{ReadWriteSeeker, read_write_seeker};

#[derive(Debug)]
pub struct MP4<T: ReadWriteSeeker> {
    r: BufReader<T>,
}

impl<T: ReadWriteSeeker> MP4<T> {
    pub fn new(r: T) -> MP4<T> {
        MP4 {
            r: BufReader::new(r),
        }
    }
}

impl MP4<File> {
    pub fn from_file(f: File) -> MP4<File> {
        MP4 {
            r: BufReader::new(f),
        }
    }

    pub fn from_file_path(path: &str) -> Result<MP4<File>> {
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
