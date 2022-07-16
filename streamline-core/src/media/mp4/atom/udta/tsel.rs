use crate::media::mp4::Info;

/// ISO BMFF `tsel` box
///
/// The track selection box is contained in the user data box of the track it modifies.
#[derive(Debug, Clone)]
pub struct Tsel {
    pub info: Info,
    /// `version` is an integer that specifies the version of this box
    /// (0 or 1 in this specification)
    pub version: u8,
    pub flags: u32,
    /// switch_group is an integer that specifies a group or collection
    /// of tracks. If this field is 0 (default value) or if the Track
    /// Selection box is absent there is no information on whether the
    /// track can be used for switching during playing or streaming. If
    /// this integer is not 0 it shall be the same for tracks that can
    /// be used for switching between each other. Tracks that belong to
    /// the same switch group shall belong to the same alternate group.
    /// A switch group may have only one member.
    pub switch_group: i32,
    /// attribute_list is a list, to the end of the box, of attributes.
    /// The attributes in this list should be used as descriptions of
    /// tracks or differentiation criteria for tracks in the same alternate
    /// or switch group. Each differentiating attribute is associated with
    /// a pointer to the field or information that distinguishes the track.
    pub attribute_list: Vec<TselAttr>,
}

/// Descriptive attributes characterize the tracks they modify, whereas
/// differentiating attributes differentiate between tracks that belong
/// to the same alternate or switch groups. The pointer of a differentiating
/// attribute indicates the location of the information that differentiates
/// the track from other tracks with the same attribute.
///
/// In the bytestream, these values will be 32 bit, UTF-8 strings.
#[derive(Debug, Clone)]
pub enum TselAttr {
    /// `tesc`: The track can be temporally scaled.
    TemporalScalability,
    /// `fgsc`: The track can be scaled in terms of quality.
    FineGrainSNRScalability,
    /// `cgsc`:The track can be scaled in terms of quality.
    CourseGrainSNRScalability,
    /// `spsc`: The track can be spatially scaled.
    SpatialScalability,
    /// `resc`: The track can be region‐of‐interest scaled.
    RegionOfInterestScalability,
    /// `vwsc`: The track can be scaled in terms of number of views.
    ViewScalability,
    /// `cdec`: Sample Entry (in Sample Description box of media track)
    Codec,
    /// `scsz`: Width and height fields of Visual Sample Entries.
    ScreenSize,
    /// `mpsz`: Maxpacketsize field in RTP Hint Sample Entry
    MaxPacketSize,
    /// `mtyp`: Handlertype in Handler box (of media track)
    MediaType,
    /// `mela`: Language field in Media Header box
    MediaLangauge,
    /// `bitr`: Total size of the samples in the track divided by the duration
    /// in the track header box
    Bitrate,
    /// `frar`: Number of samples in the track divided by duration in the track header box
    FrameRate,
    /// `nvws`: Number of views in the sub track
    NumberOfViews,
}

impl std::string::ToString for TselAttr {
    fn to_string(&self) -> String {
        match self {
            Self::TemporalScalability => "tesc".to_string(),
            Self::FineGrainSNRScalability => "fgsc".to_string(),
            Self::CourseGrainSNRScalability => "cgsc".to_string(),
            Self::SpatialScalability => "spsc".to_string(),
            Self::RegionOfInterestScalability => "resc".to_string(),
            Self::ViewScalability => "vwsc".to_string(),
            Self::Codec => "cdec".to_string(),
            Self::ScreenSize => "scsz".to_string(),
            Self::MaxPacketSize => "mpsz".to_string(),
            Self::MediaType => "mtyp".to_string(),
            Self::MediaLangauge => "mela".to_string(),
            Self::Bitrate => "bitr".to_string(),
            Self::FrameRate => "frar".to_string(),
            Self::NumberOfViews => "nvws".to_string(),
        }
    }
}

impl From<String> for TselAttr {
    fn from(s: String) -> Self {
        match s.as_str() {
            "tesc" => Self::TemporalScalability,
            "fgsc" => Self::FineGrainSNRScalability,
            "cgsc" => Self::CourseGrainSNRScalability,
            "spsc" => Self::SpatialScalability,
            "resc" => Self::RegionOfInterestScalability,
            "vwsc" => Self::ViewScalability,
            "cdec" => Self::Codec,
            "scsz" => Self::ScreenSize,
            "mpsz" => Self::MaxPacketSize,
            "mtyp" => Self::MediaType,
            "mela" => Self::MediaLangauge,
            "bitr" => Self::Bitrate,
            "frar" => Self::FrameRate,
            "nvws" => Self::NumberOfViews,
            // TODO: move this to an option
            _ => Self::TemporalScalability,
        }
    }
}
