use core::media::mp4::{MP4, atom::info::Info};
use std::fs::File;
use std::io::{Result, BufReader};

fn main() -> Result<()> {
    let mp4= MP4::from_file_path("./examples/sample.mp4").unwrap();
    println!("MP4: {:?}", mp4);

    let f = File::open("./examples/sample.mp4")?;
    let mut r = BufReader::new(f);
    let info = Info::scan(&mut r)?;
    println!("Info: {:?}", info);
    println!("Box Type: {}", info.t.string());

    Ok(())
}
