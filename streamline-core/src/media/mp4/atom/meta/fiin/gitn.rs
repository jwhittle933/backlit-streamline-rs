use crate::mp4::atom::{full_box, Info};

/// ISO BMFF Group ID to Name Box.
///
/// The Group ID to Name box associates file group names to file group IDs used in the version 1 item
/// information entries in the item information box ['Iinf'].
#[full_box]
#[derive(Debug, Clone)]
pub struct Gitn {
    pub info: Info,
    /// entry_count gives the number of entries in the following list.
    pub entry_count: u16,
    pub entries: Vec<Entry>,
}

/// Entry contains `group_id` and `group_name`.
///
/// group_ID indicates a file group.
/// group_name is a null‐terminated string in UTF‐8 characters containing a file group name.
#[derive(Debug, Clone)]
pub struct Entry(pub u32, pub String);
