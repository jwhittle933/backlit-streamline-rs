use crate::mp4::atom::{full_box, Info};

/// ISO BMFF Item Location Box
///
/// The item location box provides a directory of resources in this or other files, by locating their container,
/// their offset within that container, and their length. Placing this in binary format enables common
/// handling of this data, even by systems which do not understand the particular metadata system
/// (handler) used. For example, a system might integrate all the externally referenced metadata resources
/// into one place, re‐adjusting offsets and references accordingly.
///
/// The box starts with three or four values, specifying the size in bytes of the offset field, length
/// field, base_offset field, and, in versions 1 and 2 of this box, the extent_index fields, respectively.
/// These values must be from the set {0, 4, 8}.
///
/// The construction_method field indicates the ‘construction method’ for the item:
///  i) file_offset: by the usual absolute file offsets into the file at data_reference_index;
///     (construction_method == 0)
///  ii) idat_offset: by box offsets into the idat box in the same meta box; neither the
///     data_reference_index nor extent_index fields are used; (construction_method == 1)
///  iii) item_offset: by item offset into the items indicated by the extent_index field, which is only
///     used (currently) by this construction method. (construction_method == 2).
///
/// The extent_index is only used for the method item_offset; it indicates the 1‐based index of the item
/// reference with referenceType ‘iloc’ linked from this item. If index_size is 0, then the value 1 is implied;
/// the value 0 is reserved.
///
/// Items may be stored fragmented into extents, e.g. to enable interleaving. An extent is a contiguous
/// subset of the bytes of the resource; the resource is formed by concatenating the extents. If only one
/// extent is used (extent_count = 1) then either or both of the offset and length may be implied:
///   - If the offset is not identified (the field has a length of zero), then the beginning of the source
///     (offset 0) is implied.
///   - If the length is not specified, or specified as zero, then the entire length of the source is implied.
///     References into the same file as this metadata, or items divided into more than one extent,
///     should have an explicit offset and length, or use a MIME type requiring a different interpretation
///     of the file, to avoid infinite recursion.
///
/// The size of the item is the sum of the extent lengths.
/// NOTE Extents may be interleaved with the chunks defined by the sample tables of tracks.
///
/// The offsets are relative to a data origin. That origin is determined as follows:
///
/// 1) when the Meta box is in a Movie Fragment, and the construction_method specifies a file offset,
///    and the data reference indicates ‘same file’, the data origin is the first byte of the enclosing
///    Movie Fragment Box (as for the default‐base‐is‐moof flag in the Track Fragment Header);
///
/// 2) in all other cases when the construction_method specifies a file offset, the data origin is the
///    beginning of the file identified by the data reference;
///
/// 3) when the construction_method specifies offsets into the Item Data box, the data origin is the
///    beginning of data[] in the Item Data box;
///
/// 4) when the data reference specifies another item, the data origin is the first byte of the
///    concatenated data (of all the extents) of that item;
///
/// Note – There are offset calculations in other parts of this file format based on the beginning of a box header; in
/// contrast, item data offsets are calculated relative to the box contents.
///
/// The data‐reference index may take the value 0, indicating a reference into the same file as this
/// metadata, or an index into the data‐reference table.
///
/// Some referenced data may itself use offset/length techniques to address resources within it (e.g. an
/// MP4 file might be ‘included’ in this way). Normally such offsets in the item itself are relative to the
/// beginning of the containing file. The field ‘base offset’ provides an additional offset for offset
/// calculations within that contained data. For example, if an MP4 file is included within a file formatted to
/// this specification, then normally data‐offsets within that MP4 section are relative to the beginning of
/// file; the base offset adds to those offsets.
///
/// If an item is constructed from other items, and those source items are protected, the offset and length
/// information apply to the source items after they have been de‐protected. That is, the target item data is
/// formed from unprotected source data.
///
/// For maximum compatibility, version 0 of this box should be used in preference to version 1 with
/// construction_method == 0, or version 2 when possible. Similarly, version 2 of this box should only
/// be used when support for large item_ID values (exceeding 65535) is required or expected to be
/// required.
#[full_box]
#[derive(Debug, Clone)]
pub struct Iloc {
    pub info: Info,
    /// `offset_size` is taken from the set {0, 4, 8} and indicates the length in bytes of the offset field.
    /// 4-bit integer.
    pub offset_size: u8,
    /// `length_size` is taken from the set {0, 4, 8} and indicates the length in bytes of the length field.
    /// 4-bit integer.
    pub length_size: u8,
    /// `base_offset_size`  is  taken  from  the  set  {0,  4,  8}  and  indicates  the  length  in  bytes  of  the
    /// base_offset field.
    /// 4-bit integer.
    pub base_offset_size: u8,
    /// index_size is taken from the set {0, 4, 8} and indicates the length in bytes of the extent_index
    /// field.
    /// Only present if the version is 1 or 2.
    /// 4-bit integer.
    pub index_size: Option<u8>,
    /// Only present if the version is not 1 or 2.
    /// 4-bit integer.
    #[allow(dead_code)]
    pub(crate) reserved: Option<u8>,
    /// item_count counts the number of resources in the following array.
    ///
    /// If the version is less than 2, the field
    /// will be 16 bits. If version is equal to 2,
    /// it will be 32 bits. Field is set to 32 bits
    /// to accomodate both options.
    pub item_count: u32,

    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    /// item_ID is an arbitrary integer ‘name’ for this resource which can be used to refer to it (e.g. in a
    /// URL).
    ///
    /// If the version is less than 2, the field will be 16 bits.
    /// If the version is equal to 2, it will be 32 bits.
    pub item_id: u32,
    /// `reserved` is only present if version is 1 or 2.
    #[allow(dead_code)]
    pub(crate) reserved: Option<u16>,
    /// `construction_method` is taken from the set 0 (file), 1 (idat) or 2 (item)
    ///
    /// Only present if version is 1 or 2.
    pub construction_method: Option<u8>,
    /// `data-reference-index` is either zero (‘this file’) or a 1‐based index into the data references in
    /// the data information box
    pub data_reference_index: u16,
    /// `base_offset` provides a base value for offset calculations within the referenced data. If
    /// `base_offset_size` is 0, base_offset takes the value 0, i.e. it is unused.
    /// Option, since `base_offset` == 0 will zero this field.
    pub base_offset: Option<u64>,
    /// extent_count provides the count of the number of extents into which the resource is
    /// fragmented; it must have the value 1 or greater.
    pub extent_count: u16,
    pub extents: Vec<Extents>,
}

#[derive(Debug, Clone)]
pub struct Extents {
    /// extent_index provides an index as defined for the construction method.
    /// Only present if version is 1 or 2 and the `index_size` is greater than 0.
    pub extent_index: Option<u64>,
    /// extent_offset provides the absolute offset, in bytes from the data origin of the container, of this
    /// extent data. If offset_size is 0, extent_offset takes the value 0
    pub extent_offset: Option<u64>,
    /// extent_length provides the absolute length in bytes of this metadata item extent. If
    /// length_size is 0, extent_length takes the value 0. If the value is 0, then length of the extent
    /// is the length of the entire referenced container
    pub extent_length: Option<u64>,
}
