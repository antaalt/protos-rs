use super::resource::{Resource, ResourceDataTrait, ResourceDescTrait};


#[derive(Debug, Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BufferDescription {
    size: u32,
    format: u32, // TODO enum here
}
#[derive(Debug, Default)]
pub struct BufferData {

}

pub type Buffer = Resource<BufferDescription, BufferData>;

impl ResourceDescTrait for BufferDescription {
    
}

impl ResourceDataTrait<BufferDescription> for BufferData {
    fn new(_device: &wgpu::Device, _queue: &wgpu::Queue, _desc: &BufferDescription) -> anyhow::Result<Self> {
        Ok(Self {
            
        })
    }
    fn record_data(&self, _device: &wgpu::Device, _cmd: &mut wgpu::CommandEncoder, _desc: &BufferDescription) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Buffer {
    pub fn set_size(&mut self, size: u32) {
        self.desc.size = size;
    }
    pub fn set_format(&mut self, format: u32) {
        self.desc.format = format;
    }
}
