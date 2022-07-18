use crate::mp4::atom::{full_box, Info};

/// ISO BMFF Sub Track Group Box.
///
/// This box defines a sub track as one or more sample groups by referring to the
/// corresponding sample group descriptions describing the samples of each group.
#[full_box]
#[derive(Debug, Clone)]
pub struct Stsg {
    pub info: Info,
    /// grouping_type is an integer that identifies the sample grouping.
    /// The value shall be the same as in the corresponding SampletoGroup
    /// and SampleGroupDescription boxes.
    pub grouping_type: u32,
    /// item_count counts the number of sample groups listed in this box.
    pub item_count: u16,
    /// group_description_index is an integer that gives the index of the
    /// sample group entry which describes the samples in the group.
    pub items: Vec<u32>,
}
