use crate::mp4::atom::{full_box, Info};

/// ISO BMFF File Partition Box.
///
/// The File Partition box identifies the source file and provides a partitioning of
/// that file into source blocks and symbols. Further information about the source file,
/// e.g., filename, content location and group IDs, is contained in the Item Information
/// box ('iinf'), where the Item Information entry corresponding to the item ID of the
/// source file is of version 1 and includes a File Delivery Item Information Extension
/// ('fdel'). Version 1 of FilePartitionBox should only be used when support for large
/// item_ID or entry_count values (exceeding 65535) is required or expected to be required.
#[full_box]
#[derive(Debug, Clone)]
pub struct Fpar {
    pub info: Info,
    /// item_ID references the item in the item location box ('iloc')
    /// that the file partitioning applies to.
    ///
    /// If Version 0, 16-bit integer. Otherwise 32-bit.
    pub item_id: u32,
    /// packet_payload_size gives the target ALC/LCT or FLUTE packet payload
    /// size of the partitioning algorithm. Note that UDP packet payloads are larger,
    /// as they also contain ALC/LCT or FLUTE headers.
    pub packet_payload_size: u16,
    reserved: u8,
    /// FEC_encoding_ID identifies the FEC encoding scheme and is subject to IANA registration
    /// (see RFC 5052). Note that i) value zero corresponds to the "Compact No‐Code FEC scheme"
    /// also known as "Null‐FEC" (RFC 3695); ii) value one corresponds to the “MBMS FEC”
    /// (3GPP TS 26.346); iii) for values in the range of 0 to 127, inclusive, the FEC scheme is
    /// Fully‐Specified, whereas for values in the range of 128 to 255, inclusive, the FEC scheme
    /// is Under‐Specified.
    pub fec_encoding_id: u8,
    /// FEC_instance_ID provides a more specific identification of the FEC encoder being used for
    /// an Under‐Specified FEC scheme. This value should be set to zero for Fully‐Specified FEC
    /// schemes and shall be ignored when parsing a file with FEC_encoding_ID in the range of 0
    /// to 127, inclusive. FEC_instance_ID is scoped by the FEC_encoding_ID. See RFC 5052 for
    /// further details.
    pub fec_instance_id: u16,
    /// max_source_block_length gives the maximum number of source symbols per source block.
    pub max_source_block_length: u16,
    /// encoding_symbol_length gives the size (in bytes) of one encoding symbol. All encoding
    /// symbols of one item have the same length, except the last symbol which may be shorter.
    pub encoding_symbol_length: u16,
    /// max_number_of_encoding_symbols gives the maximum number of encoding symbols that can be generated for a source block for those FEC schemes in which the maximum number of encoding symbols is relevant, such as FEC encoding ID 129 defined in RFC 5052. For those FEC schemes in which the maximum number of encoding symbols is not relevant, the semantics of
    /// this field is unspecified.
    pub max_number_of_encoding_symbols: u16,
    /// scheme_specific_info is a base64‐encoded null‐terminated string of the scheme‐specific
    /// object transfer information (FEC‐OTI‐Scheme‐Specific‐Info). The definition of the information depends on the FEC encoding ID.
    pub scheme_specific_info: String,
    /// entry_count gives the number of entries in the list of (block_count, block_size) pairs
    /// that provides a partitioning of the source file. Starting from the beginning of the file,
    /// each entry indicates how the next segment of the file is divided into source blocks and
    /// source symbols.
    ///
    /// If Version 0, 16-bit integer. Otherwise, 32-bit.
    pub entry_count: u32,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    /// block_count indicates the number of consecutive source blocks of size block_size.
    pub block_count: u16,
    /// block_size indicates the size of a block (in bytes). A block_size that is not a
    /// multiple of the encoding_symbol_length symbol size indicates with Compact No‐Code
    /// FEC that the last source symbols includes padding that is not stored in the item.
    /// With MBMS FEC (3GPP TS 26.346) the padding may extend across multiple symbols but
    /// the size of padding should never be more than encoding_symbol_length.
    pub block_size: u32,
}
