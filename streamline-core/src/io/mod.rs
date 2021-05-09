use std::io::{Read, Seek, Write};
use std::str;

pub trait ReadSeeker: Read + Seek {}
impl<T> ReadSeeker for T where T: Read + Seek {}

pub trait ReadWriteSeeker: Read + Write + Seek {}
impl<T> ReadWriteSeeker for T where T: Read + Write + Seek {}

pub fn read_write_seeker<T: Read + Write + Seek>(t: T) -> impl ReadWriteSeeker {
    t
}

pub fn copy_n<W: Write, R: Read>(dst: &W, src: &mut R, n: u64) {
    let taken = src.take(n);
}