use std::io::{Read, Seek, Write};

pub trait ReadSeeker: Read + Seek {}
impl<T> ReadSeeker for T where T: Read + Seek {}

pub trait ReadWriteSeeker: Read + Write + Seek {}
impl<T> ReadWriteSeeker for T where T: Read + Write + Seek {}

pub fn read_write_seeker<T: Read + Write + Seek>(t: T) -> impl ReadWriteSeeker {
    t
}