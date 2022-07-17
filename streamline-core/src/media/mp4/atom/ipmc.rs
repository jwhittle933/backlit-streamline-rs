use crate::mp4::atom::{full_box, Info};

/// ISO BMFF IPMP Control Box.
///
/// The IPMP Control Box may contain IPMP descriptors which may be referenced by any stream in the file.
///
/// The IPMP_ToolListDescriptor is defined in 14496-1, which conveys the list of IPMP tools required to access
/// the media streams in an ISO Base Media File or meta-box, and may include a list of alternate IPMP tools or
/// parametric descriptions of tools required to access the content.
///
/// The presence of IPMP Descriptor in this IPMPControlBox indicates that media streams within the file or metabox
/// are protected by the IPMP Tool described in the IPMP Descriptor. More than one IPMP Descriptors can
/// be carried here, if there are more than one IPMP Tools providing the global governance.
#[full_box]
#[derive(Debug, Clone)]
pub struct Ipmc {
    pub info: Info,
    // TODO: The spec here is abismal. Work it out.

    // pub toollist: String,
    // pub no_of_descriptors: u8,
    // pub ipmp_descriptors: Vec<u8>,
}
