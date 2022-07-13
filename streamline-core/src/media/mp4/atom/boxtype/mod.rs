use std::str;

#[derive(Debug, Clone)]
pub struct BoxType {
    src: [u8; 4],
}

impl BoxType {
    pub fn new(src: [u8; 4]) -> Self {
        BoxType { src }
    }

    pub fn string(&self) -> String {
        match str::from_utf8(&self.src) {
            Ok(s) => s.to_string(),
            Err(_e) => "unknown".to_string(),
        }
    }
}
