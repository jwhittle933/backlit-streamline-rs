use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Largest Packet box.
#[derive(Debug, Clone)]
pub struct Pmax {
    pub info: Info,
    /// largest packet sent, including RTP header.
    pub bytes: u32,
}
