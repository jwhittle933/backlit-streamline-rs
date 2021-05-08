use core::media::mp4::{
    MP4,
    atom::{
        info::Info,
        ftyp::Ftyp,
        Stringer
    }
};
use std::fs::File;
use std::io::{Result, BufReader, Write};

fn main() -> Result<()> {
    let mp4 = MP4::from_file_path("./examples/sample.mp4")?;
    println!("MP4: {:?}", mp4);

    let f = File::open("./examples/sample.mp4")?;
    let mut r = BufReader::new(f);
    let info = Info::scan(&mut r)?;

    let ft = Ftyp::new(info);
    println!("{}", ft.string());

    // let buf = [0; ft.info.size as usize];

    Ok(())
}
