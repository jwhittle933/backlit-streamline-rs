use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Immediate Bytes Sent box.
#[derive(Debug, Clone)]
pub struct Dimm {
    pub info: Info,
    /// total bytes sent immediate mode.
    pub bytessent: u64,
}
