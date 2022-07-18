use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Media Bytes Sent box.
#[derive(Debug, Clone)]
pub struct Dmed {
    pub info: Info,
    /// total bytes sent from media tracks.
    pub bytessent: u64,
}
