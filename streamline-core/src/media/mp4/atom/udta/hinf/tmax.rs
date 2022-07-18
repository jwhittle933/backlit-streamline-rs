use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Max Relative Time box.
#[derive(Debug, Clone)]
pub struct Tmax {
    pub info: Info,
    /// largest relative transmission time, milliseconds
    pub time: i32,
}
