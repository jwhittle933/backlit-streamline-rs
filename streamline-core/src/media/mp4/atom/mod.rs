pub mod boxtype;
pub mod ftyp;
pub mod moov;
pub mod info;

use boxtype::BoxType;
use std::io::{Result, Write};

const SMALL_HEADER: u64 = 8;
const LARGE_HEADER: u64 = 16;

pub trait Versioned {}

pub trait Flagged {}

pub trait Typed {}

pub trait Informed {}

pub trait Stringer {}

pub trait Boxed: Typed + Informed + Stringer + Write {}
impl<T> Boxed for T where T: Typed + Informed + Stringer + Write {}

