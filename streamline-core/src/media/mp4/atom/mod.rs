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
use std::io::Write;

#[derive(Debug, Clone)]
pub enum Atom {
    Ftyp(ftyp::Ftyp),
    Moov(moov::Moov),
    Free(free::Free),
    Skip(skip::Skip),
}

pub trait Typed {
    fn t(&self) -> String {
        "unknown".to_string()
    }
}

pub trait Boxed: Typed + fmt::Display + Write + coreio::Sized {}
impl<T> Boxed for T where T: Typed + fmt::Display + Write + coreio::Sized {}

impl Atom {
    const FLAGS_MASK: u32 = 0x00ffffff;

    fn version_and_flags(vf: u32) -> (u8, u32) {
        ((vf >> 24) as u8, vf & Atom::FLAGS_MASK)
    }
}
