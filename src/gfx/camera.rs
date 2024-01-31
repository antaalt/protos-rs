use super::resource::{Resource, ResourceDataTrait, ResourceDescTrait};


#[derive(Debug, Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct CameraDescription {

}
#[derive(Debug, Default)]
pub struct CameraData {

}

pub type Camera = Resource<CameraDescription, CameraData>;

impl ResourceDescTrait for CameraDescription {
    
}

impl ResourceDataTrait<CameraDescription> for CameraData {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, desc: &CameraDescription) -> anyhow::Result<Self> {
        Ok(Self {
            
        })
    }
    fn record_data(&self, device: &wgpu::Device, cmd: &mut wgpu::CommandEncoder, desc: &CameraDescription) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Camera {
    
}