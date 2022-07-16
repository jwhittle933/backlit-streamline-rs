use crate::mp4::atom::Info;

/// ISO BMFF Scheme Information Box.
///
/// The item protection box provides an array of item protection information, for use by the Item
/// Information Box.
#[derive(Debug, Clone)]
pub struct Schi {
    pub info: Info,
    // TODO: pub boxes: Vec<Box>
}
