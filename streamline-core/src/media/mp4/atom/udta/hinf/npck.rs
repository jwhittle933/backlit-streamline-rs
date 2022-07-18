use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Packets Sent box.
///
/// Duplicate of [`Nump`], but a different box type label
/// and field size.
#[derive(Debug, Clone)]
pub struct Npck {
    pub info: Info,
    /// total bytes sent, not including RTP headers
    pub packetssent: u32,
}
