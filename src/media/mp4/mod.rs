pub mod atom;

use std::fs::File;
use std::io::{BufReader, Result};

pub struct MP4 {
    r: BufReader<File>,
}

impl MP4 {
    pub fn new(f: File) -> MP4 {
        MP4 {
            r: BufReader::new(f),
        }
    }

    pub fn from_file(path: &str) -> Result<MP4> {
        match File::open(path) {
            Ok(f) => Ok(MP4 {
                r: BufReader::new(f),
            }),
            Err(e) => Err(e),
        }
    }
}
