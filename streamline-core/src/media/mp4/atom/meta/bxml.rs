use crate::mp4::atom::{full_box, Info};

/// ISO BMFF Binary XML Box.
///
/// When the primary data is in XML format and it is desired that the XML be stored directly in
/// the meta‐ box, one of these forms may be used. The Binary XML Box may only be used when there
/// is a single well‐ defined binarization of the XML for that defined format as identified by the
/// handler.
///
/// Within an XML box the data is in UTF‐8 format unless the data starts with a byte‐order‐mark (BOM),
/// which indicates that the data is in UTF‐16 format.
#[full_box]
#[derive(Debug, Clone)]
pub struct Bxml {
    pub info: Info,
    pub xml: String,
}

impl Bxml {
    pub const NAME: &'static str = "bxml";
}
