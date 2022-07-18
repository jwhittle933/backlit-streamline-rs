use crate::mp4::atom::info::Info;

#[derive(Debug, Clone)]
pub struct Aart {
    pub info: Info,
    pub data: Vec<u8>,
}
