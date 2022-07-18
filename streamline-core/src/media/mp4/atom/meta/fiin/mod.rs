pub mod gitn;
pub mod paen;
pub mod segr;

pub use self::{gitn::Gitn, paen::Paen, segr::Segr};

use crate::mp4::atom::{full_box, Info};

/// ISO BMFF FD Item Information Box.
///
/// The FD item information box is optional, although it is mandatory for
/// files using FD hint tracks. It provides information on the partitioning
/// of source files and how FD hint tracks are combined into FD sessions. Each
/// partition entry provides details on a particular file partitioning, FEC
/// encoding and associated File and FEC reservoirs. It is possible to provide
/// multiple entries for one source file (identified by its item ID) if alternative
/// FEC encoding schemes or partitionings are used in the file. All partition entries
/// are implicitly numbered and the first entry has number 1.
#[full_box]
#[derive(Debug, Clone)]
pub struct Fiin {
    pub info: Info,
    pub entry_count: u16,
    pub partition_entries: Vec<Paen>,
    pub session_info: Option<Segr>,
    pub group_id_to_name: Option<Gitn>,
}
