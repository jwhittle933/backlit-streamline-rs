pub mod atom;

use crate::io as coreio;
use atom::{ftyp::Ftyp, info::Info, mdat::Mdat, moov::Moov, sidx::Sidx, Atom};
use coreio::ReadWriteSeeker;
use std::fs::File;
use std::io::{BufReader, Result};
use std::option::Option;

/// An MP4 handle around a reader.
#[derive(Debug)]
pub struct MP4<T: ReadWriteSeeker> {
    r: BufReader<T>,
    pub ftyp: Option<Ftyp>,
    pub moov: Option<Moov>,
    pub mdat: Option<Mdat>,
    pub sidx: Option<Sidx>,
}

impl<T: ReadWriteSeeker> MP4<T> {
    pub fn new(r: T) -> MP4<T> {
        MP4 {
            r: BufReader::new(r),
            ftyp: None,
            moov: None,
            mdat: None,
            sidx: None,
        }
    }
}

impl MP4<File> {
    pub fn from_file(f: File) -> MP4<File> {
        MP4 {
            r: BufReader::new(f),
            ftyp: None,
            moov: None,
            mdat: None,
            sidx: None,
        }
    }

    pub fn from_file_path(path: &str) -> Result<MP4<File>> {
        match File::open(path) {
            Ok(f) => Ok(MP4 {
                r: BufReader::new(f),
                ftyp: None,
                moov: None,
                mdat: None,
                sidx: None,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn read_full(&mut self) -> Result<Ftyp> {
        // TODO: return full read
        let ft = Ftyp::new(Info::scan(&mut self.r)?);

        Ok(ft)
        // match coreio::copy_sized(&mut ft, &mut self.r) {
        //     Ok(_) => Ok(ft),
        //     Err(e) => Err(e),
        // }
    }

    pub fn read_box(&mut self) -> Result<Atom> {
        let b = Self::box_from(Info::scan(&mut self.r)?)?;

        Ok(b)
        // match coreio::copy_sized(&mut b, &mut self.r) {
        //     Ok(_) => Ok(b),
        //     Err(e) => Err(e),
        // }
    }

    fn box_from(i: Info) -> Result<Atom> {
        match i.t.string().as_str() {
            "ftyp" => Ok(Atom::Ftyp(Ftyp::new(i))),
            //             "free" => Ok(Free::new(i)),
            _ => panic!("box type unknown"),
        }
    }
}
