use core::media::mp4::{
    MP4,
    atom::{
        info::Info,
        ftyp::Ftyp,
        Stringer,
    }
};
use std::fs::File;
use std::io;
use io::{Result, BufReader};
use core::media::mp4::atom::Sized;
use core::io::copy_n;

fn main() -> Result<()> {
    let mp4 = MP4::from_file_path("./examples/sample.mp4")?;
    println!("MP4: {:?}", mp4);
    println!("MP4 valid: {}", mp4.valid());

    let f = File::open("./examples/sample.mp4")?;
    let mut br = BufReader::new(f);
    let info = Info::scan(&mut br)?;

    let mut ft = Ftyp::new(&info);
    let copy_amt = info.size - info.header_size;
    // io::copy(&mut br, &mut ft)?;
    match core::io::copy_n(&mut ft, &mut br, copy_amt) {
        Ok(_) => println!("{}", ft.string()),
        Err(e) => println!("{}", e)
    };

    Ok(())
}
