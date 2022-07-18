use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Min Relative Time box.
#[derive(Debug, Clone)]
pub struct Tmin {
    pub info: Info,
    /// smallest relative transmission time, milliseconds.
    pub time: i32,
}
