use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Bytes Sent box.
///
/// Duplicate of [`Trpy`], but a different box type label
/// and field size.
#[derive(Debug, Clone)]
pub struct Totl {
    pub info: Info,
    /// total bytes sent, not including RTP headers
    pub bytessent: u32,
}
