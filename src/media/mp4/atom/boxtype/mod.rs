pub struct BoxType {
    src: [u8; 4]
}

impl BoxType {
    pub fn new(src: [u8; 4]) -> Self {
        return BoxType{src}
    }
    pub fn string() -> String {
        return String::new()
    }
}