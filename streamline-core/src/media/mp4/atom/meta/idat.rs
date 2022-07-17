use crate::mp4::atom::Info;

/// ISO BMFF Item Data Box.
///
/// This box contains the data of metadata items that use the construction
/// method indicating that an item’s data extents are stored within this box.
#[derive(Debug, Clone)]
pub struct Idat {
    pub info: Info,
    pub data: Vec<u8>,
}
