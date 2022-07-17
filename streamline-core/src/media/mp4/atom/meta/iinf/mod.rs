pub mod fdel;
pub mod infe;

use crate::mp4::atom::{full_box, Info};

/// ISO BMFF Item Information Box.
///
/// The Item information box provides extra information about selected items, including symbolic (‘file’)
/// names. It may optionally occur, but if it does, it must be interpreted, as item protection or content
/// encoding may have changed the format of the data in the item. If both content encoding and protection
/// are indicated for an item, a reader should first un‐protect the item, and then decode the item’s content
/// encoding. If more control is needed, an IPMP sequence code may be used.
///
/// This box contains an array of entries, and each entry is formatted as a box. This array is sorted by
/// increasing item_ID in the entry records.
///
/// Four versions of the item info entry are defined. Version 1 includes additional information to version 0
/// as specified by an extension type. For instance, it shall be used with extension type 'fdel' for items
/// that are referenced by the file partition box ('fpar'), which is defined for source file partitionings and
/// applies to file delivery transmissions. Versions 2 and 3 provide an alternative structure in which
/// metadata item types are indicated by a 32‐bit (typically 4‐character) registered or defined code; two of
/// these codes are defined to indicate a MIME type or metadata typed by a URI. Version 2 supports 16‐bit
/// item_ID values, whereas version 3 supports 32‐bit item_ID values.
///
/// If no extension is desired, the box may terminate without the extension_type field and the
/// extension; if, in addition, content_encoding is not desired, that field also may be absent and the box
/// terminate before it. If an extension is desired without an explicit content_encoding, a single null
/// byte, signifying the empty string, must be supplied for the content_encoding, before the indication
/// of extension_type.
///
/// If file delivery item information is needed and a version 2 or 3 ItemInfoEntry is used, then the file
/// delivery information is stored as a separate item of type ‘fdel’ that is also linked by an item reference
/// from the item, to the file delivery information, of type ‘fdel’. There must be exactly one such reference if
/// file delivery information is needed.
///
/// It is possible that there are valid URI forms for MPEG‐7 metadata (e.g. a schema URI with a fragment
/// identifying a particular element), and it may be possible that these structures could be used for MPEG‐7.
/// However, there is explicit support for MPEG‐7 in ISO base media file format family files, and this explicit
/// support is preferred as it allows, among other things:
///
/// a) incremental update of the metadata (logically, I/P coding, in video terms) whereas this draft is
///    ‘I‐frame only’;
/// b) binarization and thus compaction;
/// c) the use of multiple schemas.
///
/// Therefore, the use of these structures for MPEG‐7 is deprecated (and undocumented).
#[full_box]
#[derive(Debug, Clone)]
pub struct Iinf {
    pub info: Info,
    /// If version is 0, `entry_count` will be 16 bits. Otherwise, 32 bits.
    pub entry_count: u32,
    pub entries: Vec<infe::Infe>,
}
