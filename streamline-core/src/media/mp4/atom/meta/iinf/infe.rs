use crate::mp4::atom::{full_box, meta::iinf::fdel::Fdel, Info};

/// ISO BMFF Item Info Entry
#[full_box]
#[derive(Debug, Clone)]
pub struct Infe {
    pub info: Info,
    /// item_id contains either 0 for the primary resource (e.g., the XML contained in an ‘xml ‘ box) or
    /// the ID of the item for which the following information is defined.
    ///
    /// If version 0, 1, or 2, `item_id` will be a 16-bit. Version 3 will be 32 bits. Version > 3 will not be present.
    pub item_id: Option<u32>,
    /// item_protection_index contains either 0 for an unprotected item, or the one‐based index
    /// into the item protection box defining the protection applied to this item (the first box in the item
    /// protection box has the index 1)
    pub item_protection_index: u16,
    /// item_name is a null‐terminated string in UTF‐8 characters containing a symbolic name of the item
    /// (source file for file delivery transmissions).
    pub item_name: String,
    /// item_type is a 32‐bit value, typically 4 printable characters, that is a defined valid item type
    /// indicator, such as ‘mime’
    ///
    /// Only present if Version >= 2.
    pub item_type: Option<u32>,
    /// content_type is a null‐terminated string in UTF‐8 characters with the MIME type of the item. If
    /// the item is content encoded (see below), then the content type refers to the item after content
    /// decoding
    pub content_type: String,
    /// content_encoding is an optional null‐terminated string in UTF‐8 characters used to indicate
    /// that the binary file is encoded and needs to be decoded before interpreted. The values are as
    /// defined for Content‐Encoding for HTTP/1.1. Some possible values are “gzip”, “compress” and
    /// “deflate”. An empty string indicates no content encoding. Note that the item is stored after the
    /// content encoding has been applied
    pub content_encoding: Option<String>,
    /// extension_type is a printable four‐character code that identifies the extension fields of version
    /// 1 with respect to version 0 of the Item information entry.
    ///
    /// Only present for Version 1.
    pub extension_type: Option<u32>,
    pub item_info_extension: Option<ItemInfoBox>,
    /// item_uri_type is a string that is an absolute URI, that is used as a type indicator.
    ///
    /// Only present if Version is >= 2 and `item_type` is `uri `.
    pub item_uri_type: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ItemInfoBox {
    Known(Fdel),
    Unknown(Vec<u8>),
}
