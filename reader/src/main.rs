use core::media::mp4::MP4;
use std::io;
use io::{Result};
use core::media::mp4::atom::Stringer;

fn main() -> Result<()> {
    let mut mp4= MP4::from_file_path("./examples/sample.mp4")?;
    println!("{}", mp4.read_box().unwrap().string());

    Ok(())
}
