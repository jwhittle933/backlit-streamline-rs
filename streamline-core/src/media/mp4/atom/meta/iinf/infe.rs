use crate::mp4::atom::{full_box, Info};

#[full_box]
#[derive(Debug, Clone)]
pub struct Infe {
    pub info: Info,
}
