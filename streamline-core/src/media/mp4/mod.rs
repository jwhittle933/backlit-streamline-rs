pub mod atom;

use std::fs::File;
use std::option::Option;
use std::io::{BufReader, Result, SeekFrom, Read, Write, Seek};
use crate::io::{ReadWriteSeeker};
use atom::ftyp::Ftyp;

#[derive(Debug)]
pub struct MP4<'a, T: ReadWriteSeeker> {
    r: BufReader<T>,
    ftyp: Option<Ftyp<'a>>,
}

impl<'a, T: ReadWriteSeeker> MP4<'a, T> {
    pub fn new(r: T) -> MP4<'a, T> {
        MP4 {
            r: BufReader::new(r),
            ftyp: Option::None,
        }
    }
}

impl<'a> MP4<'a, File> {
    pub fn from_file(f: File) -> MP4<'a, File> {
        MP4 {
            r: BufReader::new(f),
            ftyp: Option::None,
        }
    }

    pub fn from_file_path(path: &str) -> Result<MP4<File>> {
        match File::open(path) {
            Ok(f) => Ok(MP4 {
                r: BufReader::new(f),
                ftyp: Option::None,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn valid(self) -> bool {
        match self.ftyp {
            Some(_) => true,
            None => false
        }
    }

    fn read_full() {
        // read all boxes and data
    }

    fn read_boxes() {
        // read info for every box
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
