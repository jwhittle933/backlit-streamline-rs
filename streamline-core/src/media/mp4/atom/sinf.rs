use crate::mp4::atom::{Frma, Info, Schi, Schm};

/// ISO BMFF Protection Scheme Box.
///
/// The Protection Scheme Information Box contains all the information required both to understand the
/// encryption transform applied and its parameters, and also to find other information such as the kind
/// and location of the key management system. It also documents the original (unencrypted) format of the
/// media. The Protection Scheme Information Box is a container Box. It is mandatory in a sample entry
/// that uses a code indicating a protected stream.
///
/// When used in a protected sample entry, this box must contain the original format box to document the
/// original format. At least one of the following signalling methods must be used to identify the protection
/// applied:
///
/// a) MPEG‐4 systems with IPMP: no other boxes, when IPMP descriptors in MPEG‐4 systems
/// streams are used;
///
/// b) Scheme signalling: a SchemeTypeBox and SchemeInformationBox, when these are used
/// (either both must occur, or neither).
///
/// At least one protection scheme information box must occur in a protected sample entry. When more
/// than one occurs, they are equivalent, alternative, descriptions of the same protection. Readers should
/// choose one to process.
#[derive(Debug, Clone)]
pub struct Sinf {
    pub info: Info,
    pub original_format: Frma,
    pub scheme_type: Option<Schm>,
    pub scheme_info: Option<Schi>,
}
