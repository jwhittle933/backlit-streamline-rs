use core::media::mp4::MP4;
use io::Result;
use std::io;

fn main() -> Result<()> {
    let mut mp4 = MP4::from_file_path("./examples/sample.mp4")?;
    println!("{}", mp4.read_box().unwrap());
    println!("{}", mp4.read_box().unwrap());
    println!("{}", mp4.read_box().unwrap());

    Ok(())
}
