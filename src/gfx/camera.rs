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
    fn new(_device: &wgpu::Device, _queue: &wgpu::Queue, _desc: &CameraDescription) -> anyhow::Result<Self> {
        Ok(Self {
            
        })
    }
    fn record_data(&self, _device: &wgpu::Device, _cmd: &mut wgpu::CommandEncoder, _desc: &CameraDescription) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Camera {
    
}