pub mod aart;

use crate::mp4::atom::info::Info;

/// ISO BMFF Metadata Item List.
#[derive(Debug, Clone)]
pub struct Ilst {
    pub info: Info,
}
