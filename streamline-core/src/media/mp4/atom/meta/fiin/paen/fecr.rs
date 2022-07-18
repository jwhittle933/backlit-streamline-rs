use crate::mp4::atom::{full_box, Info};

/// ISO BMFF FEC Resevoir Box.
///
/// The FEC reservoir box associates the source file identified in the file
/// partition box ('fpar') with FEC reservoirs stored as additional items.
/// It contains a list that starts with the first FEC reservoir associated
/// with the first source block of the source file and continues sequentially
/// through the source blocks of the source file. Version 1 of FECReservoirBox
/// should only be used when support for large item_ID values and entry_count
/// (exceeding 65535) is required or expected to be required.
#[full_box]
#[derive(Debug, Clone)]
pub struct Fecr {
    pub info: Info,
    /// entry_count gives the number of entries in the following list. An entry
    /// count here should match the total number of blocks in the corresponding
    /// file partition box.
    ///
    /// If Version 0, 16-bit integer. Otherwise, 32-bit.
    pub entry_count: u32,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    /// item_ID indicates the location of the FEC reservoir associated
    /// with a source block.
    ///
    /// If Version 0, 16-bit integer. Otherwise, 32-bit.
    pub item_id: u32,
    /// symbol_count indicates the number of repair symbols contained in the FEC
    /// reservoir.
    pub symbol_count: u32,
}
