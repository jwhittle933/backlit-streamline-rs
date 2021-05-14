pub mod boxtype;
pub mod info;
pub mod ftyp;
pub mod moov;
pub mod free;
pub mod mdat;
pub mod skip;

use std::io::{Write, Result};
use crate::io::ReadSeeker;

pub trait Versioned {}

pub trait Flagged {}

pub trait Typed {
    fn t(self) -> String;
}

pub trait Informed {}

pub trait Stringer {
    fn string(&self) -> String;
}

pub trait Sized {
    fn size(&self) -> u64;
}

pub trait Boxed: Typed + Stringer + Write {}
impl<T> Boxed for T where T: Typed + Stringer + Write {}

pub fn see_payload<T: ReadSeeker>(mut r: &mut T, i: &info::Info) -> Result<u64> {
    i.seek_payload(&mut r)
}

#[derive(Debug)]
pub enum Atom {
    Ftyp(ftyp::Ftyp),
    Moov(moov::Moov),
    Free(free::Free),
    Skip(skip::Skip),
}