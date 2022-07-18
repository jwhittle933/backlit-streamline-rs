pub mod strd;
pub mod stri;

use crate::mp4::atom::Info;

#[derive(Debug, Clone)]
pub struct Strk {
    pub info: Info,
}
