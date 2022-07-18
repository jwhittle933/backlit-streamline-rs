use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Bytes Sent box.
#[derive(Debug, Clone)]
pub struct Tpyl {
    pub info: Info,
    /// total bytes sent, not including RTP headers
    pub bytessent: u64,
}
