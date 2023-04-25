
#[derive(Copy, Clone, Debug)]
pub struct TextureHandle(u64); // wgpu::Texture

#[derive(Copy, Clone, Debug)]
pub struct ImageHandle(u64); // wgpu::Texture

#[derive(Copy, Clone, Debug)]
pub struct RawBufferHandle(u64); // wgpu::Buffer

#[derive(Copy, Clone, Debug)]
pub struct ConstantBufferHandle(u64); // wgpu::Buffer

#[derive(Copy, Clone, Debug)]
pub struct GraphicPassHandle(u64);

#[derive(Copy, Clone, Debug)]
pub struct ComputePassHandle(u64);

impl TextureHandle {
    pub fn new() -> Self { Self { 0: 0 } }
    pub fn invalid() -> Self { Self { 0: !0 } }
}
impl ImageHandle {
    pub fn new() -> Self { Self { 0: 0 } }
    pub fn invalid() -> Self { Self { 0: !0 } }
}
impl RawBufferHandle {
    pub fn new() -> Self { Self { 0: 0 } }
    pub fn invalid() -> Self { Self { 0: !0 } }
}
impl ConstantBufferHandle {
    pub fn new() -> Self { Self { 0: 0 } }
    pub fn invalid() -> Self { Self { 0: !0 } }
}



#[derive(Default, Debug)]
pub struct RenderGraph {


}

impl RenderGraph {
    pub fn new() -> Self
    {
        Self {
            
        }
    }
    pub fn create_graphic_pass(&self) -> GraphicPassHandle
    {
        /*let mut bind_group_layout : Vec<wgpu::BindGroupLayout> = vec![];
        for bind_group in _description.bind_group {
            let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: bind_group.as_ref(),
                label: Some("bind_group_layout"), // TODO: add index for debug
            });
            bind_group_layout.push(bgl);
        }
        let mut bind_group_layout_ref : Vec<&wgpu::BindGroupLayout> = vec![];
        for bind_group in &bind_group_layout {
            bind_group_layout_ref.push(bind_group);
        }

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bind_group_layout_ref.as_ref(),
            push_constant_ranges: &[],
        });

        let depth_texture = texture::Texture::create_depth_texture(&device, &_device.config, "depth_texture");
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    scene::StaticVertex::desc(),
                    scene::SceneModelInstanceData::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: _device.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        GraphicPipeline {
            //desc: _description,
            render_pipeline,
            depth_texture,
            bind_group_layout,
        }*/
        return GraphicPassHandle(0);
    }
}