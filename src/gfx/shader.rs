use super::resource::{Resource, ResourceDataTrait, ResourceDescTrait};

#[derive(Debug, Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ShaderDescription {
    pub(crate) shader: String,
}
#[derive(Debug)]
pub struct ShaderData {
    shader: wgpu::ShaderModule,
}

pub type Shader = Resource<ShaderDescription, ShaderData>;

impl ResourceDescTrait for ShaderDescription {
    
}

impl ResourceDataTrait<ShaderDescription> for ShaderData {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, desc: &ShaderDescription) -> anyhow::Result<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(desc.shader.as_str())),
        });
        Ok(Self {
            shader,
        })
    }
    fn record_data(&self, _device: &wgpu::Device, _cmd: &mut wgpu::CommandEncoder, _desc: &ShaderDescription) -> anyhow::Result<()> {
        let _ = self.shader;
        Ok(()) // Nothing to do here
    }
}