use crate::media::mp4::atom::Info;

/// ISO BMFF RTP Movie Hint Information box.
///
/// At the track level, the structure is similar; however, we already know that this track is an
/// RTP hint track, from the sample description. Therefore the child box merely specifies the
/// description format.
#[derive(Debug, Clone)]
pub struct Stp {
    pub info: Info,
    pub sdptext: Vec<char>,
}
