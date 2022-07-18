pub mod cprt;
pub mod hinf;
pub mod hnti;
pub mod ilst;
pub mod kind;
pub mod strk;
pub mod tsel;

use crate::mp4::Info;
use std::io::{Result, Write};

pub use self::{cprt::Cprt, hinf::Hinf, hnti::Hnti, ilst::Ilst, kind::Kind, tsel::Tsel};

/// `udta` User Data
///
/// This box contains objects that declare user information about the containing box and its data
/// (presentation or track).
///
/// The User Data Box is a container box for informative user‐data. The user data is formatted as
/// a set of boxes with more specific box types, which declare more precisely their content.
///
/// Can appear in [`Moov`], [`Trak`], [`Moof`], or [`Traf`]
#[derive(Debug, Clone)]
pub struct Udta {
    pub info: Info,
    pub cprt: Option<Cprt>,
    pub ilst: Option<Ilst>,
    pub kind: Option<Kind>,
    pub tsel: Option<Tsel>,
    pub hnti: Option<Hnti>,
    pub hinf: Option<Hinf>,
}

impl std::fmt::Display for Udta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl Write for Udta {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
