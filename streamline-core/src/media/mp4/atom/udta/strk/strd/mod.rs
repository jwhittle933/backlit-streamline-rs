pub mod stsg;

use crate::mp4::atom::Info;

#[derive(Debug, Clone)]
pub struct Strd {
    pub info: Info,
}
