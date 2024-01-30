use super::resource::{Resource, ResourceDataTrait, ResourceDescTrait};


#[derive(Debug, Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BufferDescription {

}
#[derive(Debug, Default)]
pub struct BufferData {

}

pub type Buffer = Resource<BufferDescription, BufferData>;

impl ResourceDescTrait for BufferDescription {
    
}

impl ResourceDataTrait<BufferDescription> for BufferData {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, desc: &BufferDescription) -> anyhow::Result<Self> {
        Ok(Self {
            
        })
    }
}

impl Buffer {
    
}
