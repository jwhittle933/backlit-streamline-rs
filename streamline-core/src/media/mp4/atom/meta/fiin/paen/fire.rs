use crate::mp4::atom::{full_box, Info};

/// ISO BMFF File Reservoir Box.
///
/// The File reservoir box associates the source file identified in the file partition box ('fpar') with File
/// reservoirs stored as additional items. It contains a list that starts with the first File reservoir associated
/// with the first source block of the source file and continues sequentially through the source blocks of the
/// source file. Version 1 of FileReservoirBox should only be used when support for large item_ID
/// or entry_count values (exceeding 65535) is required or expected to be required.
#[full_box]
#[derive(Debug, Clone)]
pub struct Fire {
    pub info: Info,
    /// entry_count gives the number of entries in the following list. An entry count here should match
    /// the total number or blocks in the corresponding file partition box.
    ///
    /// If Version 0, 16-bit integer. Otherwise, 32-bit.
    pub entry_count: u32,
}

#[derive(Debug, Clone)]
pub struct Entry {
    /// item_ID indicates the location of the File reservoir associated with a source block
    ///
    /// If Version 0, 16-bit integer. Otherwise, 32-bit.
    pub item_id: u32,
    /// symbol_count indicates the number of source symbols contained in the File reservoir
    pub symbol_count: u32,
}
