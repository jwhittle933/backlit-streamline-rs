use crate::media::mp4::atom::{full_box, Info};

/// ISO BMFF `kind` box.
///
/// The Kind box labels a track with its role or kind.
///
/// It contains a URI, possibly followed by a value. If only a URI occurs,
/// then the kind is defined by that URI; if a value follows, then the
/// naming scheme for the value is identified by the URI. Both the URI
/// and the value are null‐terminated C strings.
///
/// More than one of these may occur in a track, with different contents but with
/// appropriate semantics (e.g. two schemes that both define a kind that indicates sub‐titles).
#[full_box]
#[derive(Debug, Clone)]
pub struct Kind {
    pub info: Info,
    /// schemeURI is a NULL‐terminated C string declaring either the identifier of the kind,
    /// if no value follows, or the identifier of the naming scheme for the following value.
    pub scheme_uri: String,
    /// value is a name from the declared scheme
    pub value: String,
}
