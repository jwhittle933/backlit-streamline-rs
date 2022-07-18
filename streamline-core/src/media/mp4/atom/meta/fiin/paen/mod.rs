pub mod fecr;
pub mod fire;
pub mod fpar;

pub use self::{fecr::Fecr, fire::Fire, fpar::Fpar};
use crate::mp4::atom::Info;

#[derive(Debug, Clone)]
pub struct Paen {
    pub info: Info,
    pub blocks_and_symbols: Fpar,
    pub fec_symbol_locations: Option<Fecr>,
    pub file_symbol_locations: Option<Fire>,
}
