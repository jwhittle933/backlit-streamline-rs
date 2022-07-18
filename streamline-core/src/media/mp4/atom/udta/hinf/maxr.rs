use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Max Rate box.
#[derive(Debug, Clone)]
pub struct Maxr {
    pub info: Info,
    /// in milliseconds.
    pub period: u32,
    /// max bytes sent in any period ‘period’ long
    /// including RTP headers.
    pub bytes: u32,
}
