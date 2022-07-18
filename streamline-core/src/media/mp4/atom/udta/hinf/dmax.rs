use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Longest Packet box.
#[derive(Debug, Clone)]
pub struct Dmax {
    pub info: Info,
    /// longest packet duration, milliseconds.
    pub time: u32,
}
