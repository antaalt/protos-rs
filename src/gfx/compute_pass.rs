use super::resource::{ResourceDataTrait, ResourceDescTrait};


#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ComputePassDescription {
    pub bind_group : Vec<Vec<wgpu::BindGroupLayoutEntry>>,
}
pub struct ComputePassData {
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout : Vec<wgpu::BindGroupLayout>
}

pub type ComputePass = Resource<ComputePassDescription, ComputePassData>;

impl ResourceDescTrait for ComputePassDescription {
    
}

impl ResourceDataTrait<ComputePassDescription> for ComputePassData {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, desc: &ComputePassDescription) -> anyhow::Result<Self> {
        anyhow::bail!("Nothing to do here");
    }
}

impl ComputePass {
    pub fn has_data(&self) -> bool {
        return self.data.is_some();
    }
    pub fn record_data(&self, device: &wgpu::Device, cmd: &mut wgpu::CommandEncoder) {
        let _ = device;
        let _ = cmd;
    }
}