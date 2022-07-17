pub mod boxtype;
pub mod cdat;
pub mod dinf;
pub mod free;
pub mod frma;
pub mod ftyp;
pub mod hdlr;
pub mod info;
pub mod ipmc;
pub mod mdat;
pub mod meta;
pub mod moov;
pub mod schi;
pub mod schm;
pub mod sidx;
pub mod sinf;
pub mod skip;
pub mod udta;

pub use dinf::Dinf;
pub use frma::Frma;
pub use hdlr::Hdlr;
pub use ipmc::Ipmc;
pub use schi::Schi;
pub use schm::Schm;
pub use sinf::Sinf;
pub use streamline_macros::full_box;

use crate::io as coreio;
use info::Info;
use std::fmt;
use std::io::{Result, Write};

pub trait Typed {
    fn t(&self) -> String {
        "unknown".to_string()
    }
}

pub trait Boxed: Typed + fmt::Display + Write + coreio::Sized {}
impl<T> Boxed for T where T: Typed + fmt::Display + Write + coreio::Sized {}

pub fn seek_payload<T: coreio::ReadSeeker>(mut r: &mut T, i: &info::Info) -> Result<u64> {
    i.seek_payload(&mut r)
}

pub struct Atom {
    info: Info,
}

impl Atom {
    pub fn info(&self) -> Info {
        self.info.clone()
    }
}

pub struct FullAtom {
    pub info: Info,
    pub version: u8,
    pub flags: u32,
}

impl FullAtom {
    const FLAGS_MASK: u32 = 0x00ffffff;

    pub fn from_info(i: Info, vf: u32) -> Self {
        let (v, f) = FullAtom::version_and_flags(vf);

        Self {
            info: i,
            version: v,
            flags: f,
        }
    }

    fn version_and_flags(vf: u32) -> (u8, u32) {
        ((vf >> 24) as u8, vf & FullAtom::FLAGS_MASK)
    }
}
