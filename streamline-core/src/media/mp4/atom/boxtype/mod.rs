use std::str;

#[derive(Debug)]
pub struct BoxType {
    src: [u8; 4],
}

impl BoxType {
    pub fn new(src: [u8; 4]) -> Self {
        BoxType { src }
    }
    pub fn string(&self) -> &str {
        match str::from_utf8(&self.src) {
            Ok(s) => s,
            Err(e) => panic!("Invalid box type: {}", e)
        }
    }
}
