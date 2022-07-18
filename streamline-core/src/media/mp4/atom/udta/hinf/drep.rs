use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Immediate Bytes Sent box.
#[derive(Debug, Clone)]
pub struct Drep {
    pub info: Info,
    /// total bytes in repeated packets.
    pub bytessent: u64,
}
