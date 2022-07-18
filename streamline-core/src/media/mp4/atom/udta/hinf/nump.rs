use crate::media::mp4::atom::Info;

/// ISO BMFF Hint Packets Sent box.
#[derive(Debug, Clone)]
pub struct Nump {
    pub info: Info,
    /// total packets sent
    pub packetssent: u64,
}
