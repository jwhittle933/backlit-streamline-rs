pub mod boxtype;
pub mod ftyp;
pub mod moov;
pub mod info;

use std::io::{Write, Result};
use crate::io::ReadSeeker;
use info::Info;

pub trait Versioned {}

pub trait Flagged {}

pub trait Typed {
    fn t(self) -> String;
}

pub trait Informed {}

pub trait Stringer {
    fn string(&self) -> String;
}

pub trait Boxed: Typed + Informed + Stringer + Write {}
impl<T> Boxed for T where T: Typed + Informed + Stringer + Write {}

pub fn see_payload<T: ReadSeeker>(mut r: &mut T, i: &Info) -> Result<u64> {
    i.seek_payload(&mut r)
}