use crate::mp4::atom::{full_box, Info};

/// ISO BMFF Handler Box
///
/// This box within a Media Box declares media type of the track, and thus the process by which the media‐
/// data in  the  track is  presented.  For  example,  a  format  for  which  the  decoder  delivers  video  would  be
/// stored  in  a  video  track,  identified  by  being  handled  by  a  video  handler.  The  documentation  of  the
// storage of a media format identifies the media type which that format uses.
///
/// This box when present within a Meta Box, declares the structure or format of the 'meta' box contents.
///
/// There is  a  general  handler  for  metadata  streams  of  any  type;  the  specific  format is identified  by  the
/// sample entry, as for video or audio, for example.
#[full_box]
#[derive(Debug, Clone)]
pub struct Hdlr {
    pub info: Info,
    #[allow(dead_code)]
    pub(crate) pre_defined: u32,
    /// `handler_type`
    ///   – when present in a media box, contains a value as defined in clause 12, or a value from a derived
    ///     specification, or registration.
    ///   - when present in a meta box, contains an appropriate value to indicate the format of the meta
    ///     box contents. The value ‘null’ can be used in the primary meta box to indicate that it is
    ///     merely being used to hold resources.
    pub handler_type: u32,
    #[allow(dead_code)]
    pub(crate) reserved: [u32; 3],
    /// `name` is a null‐terminated string in UTF‐8 characters which gives a human‐readable name  for the
    /// track type (for debugging and inspection purposes).
    pub name: String,
}
