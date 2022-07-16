use crate::{
    io::Sized,
    media::mp4::atom::{info::Info, Typed},
};
use std::convert::TryInto;
use std::fmt;
use std::io::{Result, Write};
use std::str;

/// ISO BMFF `ftyp` box
///
/// Files written to this version of this specification must contain a file‐type box.
/// For compatibility with an earlier version of this specification, files may be
/// conformant to this specification and not contain a file‐ type box. Files with
/// no file‐type box should be read as if they contained an FTYP box with Major_brand='mp41',
/// minor_version=0, and the single compatible brand 'mp41'.
///
/// A media‐file structured to this part of this specification may be compatible with
/// more than one detailed specification, and it is therefore not always possible to
/// speak of a single ‘type’ or ‘brand’ for the file. This means that the utility of
/// the file name extension and Multipurpose Internet Mail Extension (MIME) type are
/// somewhat reduced.
///
/// This box must be placed as early as possible in the file (e.g. after any obligatory
/// signature, but before any significant variable‐size boxes such as a Movie Box, Media
/// Data Box, or Free Space). It identifies which specification is the ‘best use’ of the
/// file, and a minor version of that specification; and also a set of other specifications
/// to which the file complies. Readers implementing this format should attempt to read files
/// that are marked as compatible with any of the specifications that the reader implements.
/// Any incompatible change in a specification should therefore register a new ‘brand’ identifier
/// to identify files conformant to the new specification.
///
/// The minor version is informative only. It does not appear for compatible‐brands, and must not
/// be used to determine the conformance of a file to a standard. It may allow more precise
/// identification of the major specification, for inspection, debugging, or improved decoding.
///
/// Files would normally be externally identified (e.g. with a file extension or mime type) that
/// identifies the ‘best use’ (major brand), or the brand that the author believes will provide
/// the greatest compatibility.
///
/// This section of this specification does not define any brands. However, see subclause 6.3 below
/// for brands for files conformant to the whole specification and not just this section. All file
/// format brands defined in this specification are included in Annex E with a summary of which
/// features they require.
#[derive(Debug, Clone)]
pub struct Ftyp {
    pub offset: u64,
    pub size: u64,
    pub major_brand: [u8; 4],
    pub minor_version: u32,
    pub compatible_brands: Vec<[u8; 4]>,
    written: bool,
}

impl Typed for Ftyp {
    fn t(&self) -> String {
        String::from("ftyp")
    }
}

impl fmt::Display for Ftyp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ftyp] size={}, majorbrand={}, minorversion={}, written={}, compatible_brands={}",
            self.size,
            str::from_utf8(&self.major_brand).expect("failed to convert [u8; 4] to string"),
            self.minor_version,
            self.written,
            vec_to_strings(&self.compatible_brands)
        )
    }
}

impl Write for Ftyp {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.major_brand = buf[0..4]
            .try_into()
            .expect("could not covert slice to array");
        self.minor_version = u32::from_be_bytes(
            buf[4..8]
                .try_into()
                .expect("count not convert buffer to u32"),
        );

        self.compatible_brands = Vec::new();

        buf[8..].chunks(4).for_each(|c| {
            let mut copy: [u8; 4] = [0; 4];
            copy.clone_from_slice(&c);
            self.compatible_brands.push(copy);
        });

        self.written = true;

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Sized for Ftyp {
    fn size(&self) -> u64 {
        self.size
    }
}

impl Ftyp {
    pub fn new(i: Info) -> Self {
        Ftyp {
            offset: i.offset,
            size: i.size - 8,
            major_brand: [0; 4],
            minor_version: 0,
            compatible_brands: Vec::new(),
            written: false,
        }
    }
}

fn vec_to_strings(buf: &Vec<[u8; 4]>) -> String {
    let mut out: Vec<&str> = Vec::new();

    buf.into_iter().for_each(|x| {
        out.push(u8_to_string(x));
    });

    out.join(",")
}

fn u8_to_string(buf: &[u8; 4]) -> &str {
    match str::from_utf8(buf) {
        Ok(s) => s,
        Err(_) => "unknown",
    }
}
