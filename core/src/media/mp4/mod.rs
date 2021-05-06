pub mod atom;

use std::io::prelude::*;
use std::fs::File;

pub struct MP4<W: Read + Seek> {
    w: W
}

impl<W: Read + Seek> MP4<W> {
    pub fn new(w: W) -> &Self { &MP4{w} }
    pub fn from_file_path(path: String) -> &Self { &MP4{w: File::open(path)} }
}