pub mod pipe;

use std::io::{Read, Seek, Write, Result, copy};

pub trait Sized {
    fn size(&self) -> u64 { 0 }
}

pub trait ReadSeeker: Read + Seek {}

impl<T> ReadSeeker for T where T: Read + Seek {}

pub trait ReadWriteSeeker: Read + Write + Seek {}

impl<T> ReadWriteSeeker for T where T: Read + Write + Seek {}

pub fn read_write_seeker<T: Read + Write + Seek>(t: T) -> impl ReadWriteSeeker {
    t
}

pub fn copy_n<W: Write, R: Read>(mut dst: &mut W, src: &mut R, n: u64) -> Result<u64> {
    let mut taken = src.take(n);
    copy(&mut taken, &mut dst)
}

pub fn copy_sized<W: Write + Sized, R: Read>(dst: &mut W, src: &mut R) -> Result<u64> {
    copy_n(dst, src, dst.size())
}
