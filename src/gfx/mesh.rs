use super::resource::{Resource, ResourceDataTrait, ResourceDescTrait};


#[derive(Debug, Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct MeshDescription {

}
#[derive(Debug)]
pub struct MeshData {

}

pub type Mesh = Resource<MeshDescription, MeshData>;

impl ResourceDescTrait for MeshDescription {
    
}

impl ResourceDataTrait<MeshDescription> for MeshData {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, desc: &MeshDescription) -> anyhow::Result<Self> {
        Ok(Self {
            
        })
    }
}

impl Mesh {
    
}