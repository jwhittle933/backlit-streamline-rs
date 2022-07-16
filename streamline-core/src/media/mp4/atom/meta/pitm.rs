use crate::mp4::Info;

/// ISO BMFF Primary Item Box
///
/// For a given handler, the primary data may be one of the referenced items when it is desired that it
/// be stored elsewhere, or divided into extents; or the primary metadata may be contained in the meta‐box
/// (e.g.in an XML box). Either this box must occur, or there must be a box within the meta‐box (e.g. an XML box)
/// containing the primary information in the format required by the identified handler.
#[derive(Debug, Clone)]
pub struct Pitm {
    pub info: Info,
    /// If version == 0, `item_id` will be 16-bit. Version 1 will be 32-bit.
    pub version: u8,
    pub flags: u32,
    /// item_ID is the identifier of the primary item. Version 1 should only be used when large item_ID
    /// values (exceeding 65535) are required or expected to be required.
    pub item_id: u32, // can be 16 bit or 32 bit
}
