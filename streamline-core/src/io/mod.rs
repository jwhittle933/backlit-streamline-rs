pub mod pipe;

use std::io::{copy, Read, Result, Seek, Write};

pub trait Sized {
    fn size(&self) -> u64 {
        0
    }
}

pub trait ReadSeeker: Read + Seek {}
impl<T> ReadSeeker for T where T: Read + Seek {}

pub trait ReadWriteSeeker: Read + Write + Seek {}
impl<T> ReadWriteSeeker for T where T: Read + Write + Seek {}

/// Copies `n` number of bytes to `dst` from `R`.
pub fn copy_n<W: Write, R: Read>(mut dst: &mut W, src: &mut R, n: u64) -> Result<u64> {
    let mut taken = src.take(n);
    copy(&mut taken, &mut dst)
}

/// Copies `n` number of bytes from `R` where `n` is equal to `dst.size()`.
pub fn copy_sized<W: Write + Sized, R: Read>(dst: &mut W, src: &mut R) -> Result<u64> {
    copy_n(dst, src, dst.size())
}
