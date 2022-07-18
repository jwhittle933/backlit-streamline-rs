use crate::media::mp4::atom::Info;

/// ISO BMFF RTP Movie Hint Information box.
///
/// The hint information box may contain information for multiple protocols; only RTP is defined here.
/// The RTP box may contain information for various description formats; only SDP is defined here. The
/// sdptext is correctly formatted as a series of lines, each terminated by <crlf>, as required by SDP.
#[derive(Debug, Clone)]
pub struct Rtp {
    pub info: Info,
    pub description_format: u32, // 4-byte String
    pub sdptext: Vec<char>,      // C-char
}
