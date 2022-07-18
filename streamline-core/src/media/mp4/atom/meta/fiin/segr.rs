use crate::mp4::atom::Info;
/// ISO BMFF FD Session Group Box.
///
/// The FD session group box is optional, although it is mandatory for files containing more than one FD
/// hint track. It contains a list of sessions as well as all file groups and hint tracks that belong to each
/// session. An FD session sends simultaneously over all FD hint tracks (channels) that are listed in the FD
/// session group box for a particular FD session.
///
/// Only one session group should be processed at any time. The first listed hint track in a session group
/// specifies the base channel. If the server has no preference between the session groups, the default
/// choice should be the first session group. The group IDs of all file groups containing the files referenced
/// by the hint tracks shall be included in the list of file groups. The file group IDs can in turn be translated
/// into file group names (using the group ID to name box) that can be included by the server in FDTs.
#[derive(Debug, Clone)]
pub struct Segr {
    pub info: Info,
    /// num_session_groups specifies the number of session groups.
    pub num_session_groups: u16,
    pub groups: Vec<Group>,
}

#[derive(Debug, Clone)]
pub struct Group {
    /// entry_count gives the number of entries in the following list comprising all file groups that the
    /// session group complies with. The session group contains all files included in the listed file
    /// groups as specified by the item information entry of each source file. Note that the FDT for the
    /// session group should only contain those groups that are listed in this structure.
    /// group_ID indicates a file group that the session group complies with.
    pub entry_count: u8,
    pub entries: Vec<u32>,
    /// num_channels_in_session_groups specifies the number of channels in the session group.
    /// The value of num_channels_in_session_groups shall be a positive integer.
    pub num_channels_in_session_group: u16,
    /// Specifies the track ID of the FD hint track belonging to a particular session group.
    /// Note that one FD hint track corresponds to one LCT channel.
    pub hint_tracks: Vec<u32>,
}
