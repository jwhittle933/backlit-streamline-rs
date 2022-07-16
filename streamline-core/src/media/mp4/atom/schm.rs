use crate::mp4::atom::Info;

/// ISO BMFF Scheme Type Box.
///
/// The Scheme Type Box (‘schm’) identifies the protection or restriction scheme.
#[derive(Debug, Clone)]
pub struct Schm {
    pub info: Info,
    /// scheme_type is the code defining the protection or restriction scheme.
    pub scheme_type: u32,
    /// scheme_version is the version of the scheme (used to create the content)
    pub scheme_version: u32,
    /// scheme_URI allows for the option of directing the user to a web‐page if they do not have the
    /// scheme installed on their system. It is an absolute URI formed as a null‐terminated string in
    /// UTF‐8 characters.
    pub scheme_uri: Option<String>,
}
