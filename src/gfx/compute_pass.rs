
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ComputePassDescription {
    pub bind_group : Vec<Vec<wgpu::BindGroupLayoutEntry>>,
}
pub struct ComputePassData {
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout : Vec<wgpu::BindGroupLayout>
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ComputePass {
    desc: ComputePassDescription,
    #[cfg_attr(feature = "persistence", serde(skip))]
    data: Option<ComputePassData>
}

impl Default for ComputePassDescription {
    fn default() -> Self {
        Self { bind_group: Vec::new() }
    }
}
impl Default for ComputePass {
    fn default() -> Self {
        Self {
            desc: ComputePassDescription::default(),
            data: None,
        }
    }
}

impl ComputePass {
    pub fn has_data(&self) -> bool {
        return self.data.is_some();
    }
    pub fn update_data(&mut self, device: &wgpu::Device) {
        let _ = device;
    }
    pub fn record_data(&self, device: &wgpu::Device, cmd: &mut wgpu::CommandEncoder) {
        let _ = device;
        let _ = cmd;
    }
}