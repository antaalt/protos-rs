
pub struct CameraDescription {

}
pub struct CameraData {

}
pub struct Camera {
    desc: CameraDescription,
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