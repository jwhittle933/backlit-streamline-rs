use crate::mp4::atom::Info;

/// ISO BMFF Original Format Box.
///
/// The Original Format Box ‘frma’ contains the four‐character‐code of the original un‐transformed
/// sample description:
#[derive(Debug, Clone)]
pub struct Frma {
    pub info: Info,
    /// data_format is the four‐character‐code of the original un‐transformed sample entry (e.g. ‘mp4v’
    /// if the stream contains protected or restricted MPEG‐4 visual material).
    pub data_format: u32,
}
