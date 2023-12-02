pub struct MeshDescription {
    
}
pub struct MeshData {

}
pub struct Mesh {
    desc: MeshDescription,
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