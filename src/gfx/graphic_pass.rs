use super::core::*;
use super::texture::*;

pub enum VertexFactory {
    Static, // Vertex layout for static mesh.
}
pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
    fn factory() -> VertexFactory;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct StaticVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex for StaticVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<StaticVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { // pos
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { // uv
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute { // normal
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { // tangent
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { // bitangent
                    offset: mem::size_of::<[f32; 11]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { // color
                    offset: mem::size_of::<[f32; 14]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
    fn factory() -> VertexFactory {
        VertexFactory::Static
    }
}


pub struct GraphicPassDescription {
    bind_group : Vec<Vec<wgpu::BindGroupLayoutEntry>>,
    render_target: Vec<Option<ResourceHandle<Texture>>>,
    shader_resource_view: Vec<Option<ResourceHandle<Texture>>>,
    //vertex_buffer_layout : Vec<wgpu::VertexBufferLayout>,
}
pub struct GraphicPassData {
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout : Vec<wgpu::BindGroupLayout>
}
pub struct GraphicPass {
    desc: GraphicPassDescription,
    data: Option<GraphicPassData>
}

impl Default for GraphicPassDescription {
    fn default() -> Self {
        Self { 
            bind_group: Vec::new(),
            render_target: Vec::new(),
            shader_resource_view: Vec::new(),
        }
    }
}
impl GraphicPassDescription {
    pub fn new() -> Self {
        Self { 
            bind_group: Vec::new(),
            render_target: Vec::new(),
            shader_resource_view: Vec::new(),
        }
    }
}
fn default_bind_group_entry(index : u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility:wgpu::ShaderStages::NONE,
        ty: wgpu::BindingType::Texture { 
            sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
            view_dimension: wgpu::TextureViewDimension::D2, 
            multisampled: false
        },
        count: None
    }
}
impl GraphicPass {
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
    pub fn update_data(&mut self, device : &wgpu::Device) {
        if self.data.is_some() {
            // TODO: check if this is enough & resources are correctly cleared...
            self.data = Some(GraphicPassData::new(device, &self.desc))
        } else {
            self.data = Some(GraphicPassData::new(device, &self.desc))
        }
    }
    pub fn record_data(&self, device : &wgpu::Device) {
        // TODO: record command list here ?
    }
    pub fn set_shader_resource_view(&mut self, index: u32, srv : Option<ResourceHandle<Texture>>) {
        // TODO: resize should be done by desc data ? or data is built by shader resource view that are set ? 
        if index as usize >= self.desc.shader_resource_view.len() {
            self.desc.shader_resource_view.resize(index as usize + 1, None);
            self.desc.bind_group.resize(1, Vec::new());
            self.desc.bind_group[0].resize(index as usize + 1, default_bind_group_entry(index));
        }
        if srv.is_some() {
            let texture_locked = srv.clone().unwrap();
            let texture = texture_locked.lock().unwrap();            
            self.desc.bind_group[0][index as usize].binding = index;
            self.desc.bind_group[0][index as usize].count = None;
            self.desc.bind_group[0][index as usize].ty = wgpu::BindingType::Texture { 
                sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                view_dimension: wgpu::TextureViewDimension::D2, // TODO: desc fitting texture 
                multisampled: false
            };
            self.desc.bind_group[0][index as usize].visibility = wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT;
        } else {
            self.desc.bind_group[0][index as usize] = default_bind_group_entry(index);
        }

        self.desc.shader_resource_view[index as usize] = srv;
    }
    pub fn clear_shader_resource_view(&mut self, index: u32) {
        self.set_shader_resource_view(index, None);
    }
    pub fn set_render_target(&mut self, index: u32, rt : ResourceHandle<Texture>) {
        self.register_render_target(index);
        self.desc.render_target[index as usize] = Some(rt);
    }
    pub fn clear_render_target(&mut self, index: u32) {
        self.register_render_target(index);
        self.desc.render_target[index as usize] = None;
    }
    pub fn get_render_target(&self, index: u32) -> Option<ResourceHandle<Texture>> {
        self.desc.render_target[index as usize].clone()
    }
    fn register_render_target(&mut self, index: u32) {
        if index as usize >= self.desc.render_target.len() {
            self.desc.render_target.resize(index as usize + 1, None);
        }
    }
}
impl GraphicPassData {
    fn new(device : &wgpu::Device, description : &GraphicPassDescription) -> Self {
        // Create bind groups.
        let mut bind_group_layout : Vec<wgpu::BindGroupLayout> = vec![];
        for (i, bind_group) in description.bind_group.iter().enumerate() {
            bind_group_layout.push(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: bind_group.as_ref(),
                label: Some(format!("bind_group_layout{}", i).as_str()),
            }));
        }
        let mut bind_group_layout_ref : Vec<&wgpu::BindGroupLayout> = vec![];
        for bind_group in &bind_group_layout {
            bind_group_layout_ref.push(bind_group);
        }

        // Create attachments
        let mut render_targets : Vec<Option<wgpu::ColorTargetState>> = vec![];
        for (i, render_target) in description.render_target.iter().enumerate() {
            render_targets.push(Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8Unorm,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL
            }));
        }

        // Create shaders
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()), // TODO: dynamic shader.
        });

        // Create pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bind_group_layout_ref.as_ref(),
            push_constant_ranges: &[], // TODO: push constant
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    StaticVertex::desc(),
                    //Instance::desc() // TODO: get this from rust-engine
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: render_targets.as_ref(),
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
            depth_stencil: None/*Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            })*/,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        GraphicPassData { 
            render_pipeline: render_pipeline, 
            bind_group_layout: bind_group_layout 
        }
    }
}
impl Default for GraphicPass {
    fn default() -> Self {
        Self {
            desc: GraphicPassDescription::default(),
            data: None,
        }
    }
}