
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct MeshDescription {
    
}
pub struct MeshData {

}
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Mesh {
    desc: MeshDescription,
    #[cfg_attr(feature = "persistence", serde(skip_serializing, skip_deserializing))]
    data: Option<MeshData>,
}

impl Default for MeshDescription {
    fn default() -> Self {
        Self {}
    }
}
impl Default for Mesh {
    fn default() -> Self {
        Self {
            desc: MeshDescription::default(),
            data: None,
        }
    }
}