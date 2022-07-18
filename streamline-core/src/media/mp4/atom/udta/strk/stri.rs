use crate::mp4::atom::{full_box, Info};

#[full_box]
#[derive(Debug, Clone)]
pub struct Stri {
    pub info: Info,
    /// switch_group is an integer that specifies a group or collection of tracks and/or
    /// sub tracks. If this field is 0 (default value), then there is no information on
    /// whether the sub track can be used for switching during playing or streaming. If
    /// this integer is not 0 it shall be the same for tracks and/or sub tracks that can be
    /// used for switching between each other. Tracks that belong to the same switch group
    /// shall belong to the same alternate group. A switch group may have only one member.
    pub switch_group: u16,
    /// alternate_group is an integer that specifies a group or collection of tracks and/or
    /// sub tracks. If this field is 0 (default value), then there is no information on
    /// possible relations to other tracks/sub‐tracks. If this field is not 0, it should be
    /// the same for tracks/sub‐tracks that contain alternate data for one another and different
    /// for tracks/sub‐tracks belonging to different such groups. Only one track/sub‐track
    /// within an alternate group should be played or streamed at any one time.
    pub alternate_group: i16,
    /// sub_track_ID is an integer. A non‐zero value uniquely identifies the sub track locally
    /// within the track. A zero value (default) means that sub track ID is not assigned.
    pub sub_track_id: u32,
    /// attribute_list is a list, to the end of the box, of attributes. The attributes in this
    /// list should be used as descriptions of sub tracks or differentiating criteria for tracks
    /// and sub tracks in the same alternate or switch group.
    pub attribute_list: Vec<Attr>,
}

#[derive(Debug, Clone)]
pub enum Attr {
    TemporalScalability,
    FineGrainSNRScalability,
    CourseGrainSNVScalability,
    SpatialScalability,
    RegionOfInterestScalability,
    ViewScalability,
    Bitrate,
    FrameRate,
    NumberOfViews,
}
