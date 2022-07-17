pub mod idat;
pub mod iinf;
pub mod iloc;
pub mod ipro;
pub mod iref;
pub mod pitm;
pub mod xml;

pub use self::{idat::Idat, iinf::Iinf, iloc::Iloc, ipro::Ipro, iref::Iref, pitm::Pitm};
use crate::mp4::{
    atom::{full_box, Dinf, Hdlr, Ipmc},
    Info,
};
use std::io::{Result, Write};

///  ISO BMFF Meta box
///
/// A meta box contains descriptive or annotative metadata. The 'meta' box is required to contain a
/// ‘hdlr’ box indicating the structure or format of the ‘meta’ box contents. That metadata is located
/// either within a box within this box (e.g. an XML box), or is located by the item identified by
/// primary item box.
///
/// All other contained boxes are specific to the format specified by the handler box.
///
/// The other boxes defined here may be defined as optional or mandatory for a given format. If they
/// are used, then they must take the form specified here. These optional boxes include a data‐information
/// box, which documents other files in which metadata values (e.g. pictures) are placed, and a item
/// location box, which documents where in those files each item is located (e.g. in the common case of
/// multiple pictures stored in the same file). At most one meta box may occur at each of the file level,
/// movie level, or track level, unless they are contained in an additional metadata container box (‘meco’).
///
/// If an Item Protection Box occurs, then some or all of the meta‐data, including possibly the primary
/// resource, may have been protected and be un‐readable unless the protection system is taken into account.
///
/// Child of [`Moov`], [`Trak`], [`Meco`], [`Moof`], or [`Traf`].
#[full_box]
#[derive(Debug, Clone)]
pub struct Meta {
    pub info: Info,
    /// The structure or format of the metadata is declared by the handler.
    /// In the case that the primary data is identified by a primary item,
    /// and that primary item has an item information entry with an item_type,
    /// the handler type may be the same as the item_type.
    pub hdlr: Option<Hdlr>,
    pub pitm: Option<Pitm>,
    pub dinf: Option<Dinf>,
    pub iloc: Option<Iloc>,
    pub ipro: Option<Ipro>,
    pub ipmc: Option<Ipmc>,
    pub iref: Option<Iref>,
    pub idat: Option<Idat>,
    /// Other Boxes
    pub boxes: Vec<u8>,
}

impl Write for Meta {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
