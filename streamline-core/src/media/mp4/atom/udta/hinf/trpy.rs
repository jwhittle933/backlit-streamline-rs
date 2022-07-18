use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Bytes Sent box.
#[derive(Debug, Clone)]
pub struct Trpy {
    pub info: Info,
    /// total bytes sent, including 12-byte RTP headers
    pub bytessent: u64,
}
