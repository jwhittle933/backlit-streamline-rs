pub mod dimm;
pub mod dmax;
pub mod dmed;
pub mod drep;
pub mod maxr;
pub mod npck;
pub mod nump;
pub mod payt;
pub mod pmax;
pub mod tmax;
pub mod tmin;
pub mod totl;
pub mod tpay;
pub mod tpyl;
pub mod trpy;

pub use self::{
    dimm::Dimm, dmax::Dmax, dmed::Dmed, drep::Drep, maxr::Maxr, npck::Npck, nump::Nump, payt::Payt,
    pmax::Pmax, tmax::Tmax, tmin::Tmin, totl::Totl, tpay::Tpay, tpyl::Tpyl, trpy::Trpy,
};

use crate::media::mp4::atom::Info;

/// ISO BMFF Movie Hint Statistics box.
///
/// In addition to the statistics in the hint media header, the hinter may place extra data in a hint
/// statistics box, in the track user‐data box. This is a container box with a variety of sub‐boxes
/// that it may contain.
///
/// NOTE Not all these sub‐boxes may be present, and that there may be multiple ‘maxr’ boxes, covering
/// different periods.
#[derive(Debug, Clone)]
pub struct Hinf {
    pub info: Info,
    pub trpy: Option<Trpy>,
    pub nump: Option<Nump>,
    pub tpyl: Option<Tpyl>,
    pub totl: Option<Totl>,
    pub tpay: Option<Tpay>,
    pub npck: Option<Npck>,
    pub maxr: Option<Vec<Maxr>>, // Can be many
    pub dmed: Option<Dmed>,
    pub dimm: Option<Dimm>,
    pub drep: Option<Drep>,
    pub tmin: Option<Tmin>,
    pub tmax: Option<Tmax>,
    pub pmax: Option<Pmax>,
    pub dmax: Option<Dmax>,
    pub payt: Option<Payt>,
}
