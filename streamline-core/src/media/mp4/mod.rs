pub mod atom;

use crate::io as coreio;
use atom::{
    free::Free, ftyp::Ftyp, info::Info, mdat::Mdat, moov::Moov, sidx::Sidx, skip::Skip, Boxed,
};
use coreio::ReadWriteSeeker;
use std::fs::File;
use std::io::{BufReader, Result};
use std::option::Option;

#[derive(Debug)]
pub enum Atom {
    Ftyp(Ftyp),
    Moov(Moov),
    Free(Free),
    Skip(Skip),
}

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

    pub fn header(&self) -> Option<&Ftyp> {
        self.ftyp.as_ref()
    }

    pub fn valid(self) -> bool {
        match self.ftyp {
            Some(_) => true,
            None => false,
        }
    }

    pub fn read_full(&mut self) -> Result<Ftyp> {
        // TODO: return full read
        let mut ft = Ftyp::new(Info::scan(&mut self.r)?);

        match coreio::copy_sized(&mut ft, &mut self.r) {
            Ok(_) => Ok(ft),
            Err(e) => Err(e),
        }
    }

    pub fn read_box(&mut self) -> Result<impl Boxed> {
        let mut b = Self::box_from(Info::scan(&mut self.r)?)?;

        match coreio::copy_sized(&mut b, &mut self.r) {
            Ok(_) => Ok(b),
            Err(e) => Err(e),
        }
    }

    fn box_from(i: Info) -> Result<impl Boxed> {
        match i.t.string().as_str() {
            "ftyp" => Ok(Ftyp::new(i)),
            //             "free" => Ok(Free::new(i)),
            _ => panic!("box type unknown"),
        }
    }
}
