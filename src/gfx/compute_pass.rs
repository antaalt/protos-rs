use super::resource::{Resource, ResourceDataTrait, ResourceDescTrait};


#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ComputePassDescription {
    pub bind_group : Vec<Vec<wgpu::BindGroupLayoutEntry>>,
}
pub struct ComputePassData {
    compute_pipeline: wgpu::ComputePipeline,
}

pub type ComputePass = Resource<ComputePassDescription, ComputePassData>;

impl ResourceDescTrait for ComputePassDescription {
    
}

impl ResourceDataTrait<ComputePassDescription> for ComputePassData {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, _desc: &ComputePassDescription) -> anyhow::Result<Self> {
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("module"), 
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(""))
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None,
                }
            ]
        });
        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
            label: Some("layout"), 
            bind_group_layouts: &[&bind_group_layout], 
            push_constant_ranges: &[] 
        });
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute_pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &module,
            entry_point: "main",
        });
        // TODO: should handle async
        let validation = pollster::block_on(device.pop_error_scope()).and_then(|err| {
            Some(err)
        });
        if let Some(v) = validation {
            anyhow::bail!(v.to_string())
        } else {
            Ok(Self {
                compute_pipeline,
            })
        }
    }
    fn record_data(&self, _device: &wgpu::Device, cmd: &mut wgpu::CommandEncoder, _desc: &ComputePassDescription) -> anyhow::Result<()> {
        let mut compute_pass = cmd.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.compute_pipeline);
        Ok(())
    }
}