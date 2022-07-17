use crate::mp4::atom::{full_box, Info};

#[full_box]
#[derive(Debug, Clone)]
pub struct Fdel {
    pub info: Info,
    /// content_location is a null‐terminated string in UTF‐8 characters containing the URI of the file
    /// as defined in HTTP/1.1 (RFC 2616).
    pub content_location: String,
    /// content_MD5 is a null‐terminated string in UTF‐8 characters containing an MD5 digest of the file.
    /// See HTTP/1.1 (RFC 2616) and RFC 1864.
    pub content_md5: String,
    /// content_length gives the total length (in bytes) of the (un‐encoded) file.
    pub content_length: u64,
    /// transfer_length gives the total length (in bytes) of the (encoded) file. Note that transfer length
    /// is equal to content length if no content encoding is applied (see above).
    pub transfer_length: u64,
    /// entry_count provides a count of the number of entries in the following array.
    pub entry_count: u8,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    /// group_ID indicates a file group to which the file item (source file) belongs. See 3GPP TS 26.346
    /// for more details on file groups.
    pub group_id: u32,
}
