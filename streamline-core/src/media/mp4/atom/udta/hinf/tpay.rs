use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Bytes Sent box.
///
/// Duplicate of [`Tpyl`], but different label and
/// field size.
#[derive(Debug, Clone)]
pub struct Tpay {
    pub info: Info,
    /// total bytes sent, not including RTP headers
    pub bytessent: u32,
}
