use crate::mp4::Info;

/// ISO BMFF Data Information Box
///
/// The data information box contains objects that declare the location of
/// the media information in a track.
#[derive(Debug, Clone)]
pub struct Dinf {
    pub info: Info,
}
