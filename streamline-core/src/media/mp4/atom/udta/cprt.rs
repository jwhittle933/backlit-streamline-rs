use crate::media::mp4::atom::{full_box, Info};

/// `cprt` Copyright
///
/// The Copyright box contains a copyright declaration which applies to the entire presentation,
/// when contained within the Movie Box, or, when contained in a track, to that entire track.
/// There may be multiple copyright boxes using different language codes.
#[full_box]
#[derive(Debug, Clone)]
pub struct Cprt {
    pub info: Info,
    /// pad is exactly 1 bit
    pub pad: u8,
    /// language declares the language code for the following text. See ISO 639‐2/T
    /// for the set of three character codes. Each character is packed as the
    /// difference between its ASCII value and 0x60. The code is confined to being
    /// three lower‐case letters, so these values are strictly positive.
    pub language: String, // int(5)[3]
    /// notice is a null‐terminated string in either UTF‐8 or UTF‐16 characters,
    /// giving a copyright notice. If UTF‐16 is used, the string shall start with
    /// the BYTE ORDER MARK (0xFEFF), to distinguish it from a UTF‐8 string. This
    /// mark does not form part of the final string.
    pub notice: String,
}

mod tests {
    //
}
