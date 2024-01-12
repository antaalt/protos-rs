
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct CameraDescription {

}
pub struct CameraData {

}
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Camera {
    desc: CameraDescription,
    #[cfg_attr(feature = "persistence", serde(skip_serializing, skip_deserializing))]
    data: Option<CameraData>,
}

impl Default for CameraDescription {
    fn default() -> Self {
        Self {}
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            desc: CameraDescription::default(),
            data: None,
        }
    }
}