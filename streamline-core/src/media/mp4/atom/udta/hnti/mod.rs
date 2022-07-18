pub mod rtp;
pub mod stp;

pub use self::{rtp::Rtp, stp::Stp};

use crate::media::mp4::atom::Info;

/// ISO BMFF Movie Hint Information box.
#[derive(Debug, Clone)]
pub struct Hnti {
    pub info: Info,
    pub rtp: Option<Rtp>,
    pub stp: Option<Stp>,
}
