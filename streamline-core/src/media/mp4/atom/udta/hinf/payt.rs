use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Payload ID box.
#[derive(Debug, Clone)]
pub struct Payt {
    pub info: Info,
    pub payload_id: u32,
    pub count: u8,
    pub trpmap_string: Vec<char>,
}
