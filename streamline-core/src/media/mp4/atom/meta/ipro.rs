use crate::mp4::atom::{full_box, Info, Sinf};

/// ISO BMFF Item Protection Box
///
/// The item protection box provides an array of item protection information, for use by the Item
/// Information Box.
#[full_box]
#[derive(Debug, Clone)]
pub struct Ipro {
    pub info: Info,
    pub protection_count: u16,
    pub sinf: Vec<Sinf>,
}
