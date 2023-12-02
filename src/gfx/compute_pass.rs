
pub struct ComputePassDescription {
    pub bind_group : Vec<Vec<wgpu::BindGroupLayoutEntry>>,
}
pub struct ComputePassData {
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout : Vec<wgpu::BindGroupLayout>
}

pub struct ComputePass {
    desc: ComputePassDescription,
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