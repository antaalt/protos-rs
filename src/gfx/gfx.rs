use std::{default, collections::{HashMap, btree_map::Range}};


// --------------------------- HANDLES -------------------------------
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextureHandle(u64); // wgpu::Texture

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ImageHandle(u64); // wgpu::Texture

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct RawBufferHandle(u64); // wgpu::Buffer

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConstantBufferHandle(u64); // wgpu::Buffer

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct GraphicPassHandle(u64);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

// --------------------------- PIPELINE -------------------------------
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

// --------------------------- BUFFER -------------------------------
#[derive(Debug)]
pub struct BufferDescription {

}
#[derive(Debug)]
pub struct BufferData {

}
#[derive(Debug)]
pub struct Buffer {
    desc: BufferDescription,
    data: Option<BufferData>,
}

impl Default for BufferDescription {
    fn default() -> Self {
        Self {}
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Self {
            desc: BufferDescription::default(),
            data: None,
        }
    }
}

// --------------------------- TEXTURE -------------------------------
#[derive(Debug)]
pub struct TextureDescription {

}
#[derive(Debug)]
pub struct TextureData {

}
#[derive(Debug)]
pub struct Texture {
    desc: TextureDescription,
    data: Option<TextureData>,
}

impl Default for TextureDescription {
    fn default() -> Self {
        Self {}
    }
}
impl Default for Texture {
    fn default() -> Self {
        Self {
            desc: TextureDescription::default(),
            data: None,
        }
    }
}

// --------------------------- CAMERA -------------------------------
pub struct CameraDescription {

}
pub struct CameraData {

}
pub struct Camera {
    desc: CameraDescription,
    data: Option<CameraData>,
}

impl Default for CameraDescription {
    fn default() -> Self {
        Self {}
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            desc: CameraDescription::default(),
            data: None,
        }
    }
}

// --------------------------- BUFFER -------------------------------
pub struct MeshDescription {
    
}
pub struct MeshData {

}
pub struct Mesh {
    desc: MeshDescription,
    data: Option<MeshData>,
}

impl Default for MeshDescription {
    fn default() -> Self {
        Self {}
    }
}
impl Default for Mesh {
    fn default() -> Self {
        Self {
            desc: MeshDescription::default(),
            data: None,
        }
    }
}

// --------------------------- GRAPHIC PASS -------------------------------
pub struct GraphicPassDescription {
    pub bind_group : Vec<Vec<wgpu::BindGroupLayoutEntry>>,
    //pub vertex_buffer_layout : Vec<wgpu::VertexBufferLayout>,
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
        Self { bind_group: Vec::new() }
    }
}
impl GraphicPass {
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
    pub fn create_data(&mut self, device : &wgpu::Device) {
        if self.data.is_some() {
            // TODO handle if there is change here.
        } else {
            self.data = Some(GraphicPassData::new(device, &self.desc))
        }
    }
}
impl GraphicPassData {
    fn new(device : &wgpu::Device, description : &GraphicPassDescription) -> Self {
        let mut bind_group_layout : Vec<wgpu::BindGroupLayout> = vec![];
        //let mut i = 0;
        for i in 0..description.bind_group.len() {
            let bind_group = &description.bind_group[i];
        //for bind_group in description.bind_group {
            //i = i + 1;
            let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: bind_group.as_ref(),
                label: Some(format!("bind_group_layout{}", i).as_str()),
            });
            bind_group_layout.push(bgl);
        }
        let mut bind_group_layout_ref : Vec<&wgpu::BindGroupLayout> = vec![];
        for bind_group in &bind_group_layout {
            bind_group_layout_ref.push(bind_group);
        }

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()), // TODO: dynamic shader.
        });
        
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bind_group_layout_ref.as_ref(),
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    StaticVertex::desc()
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
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

// --------------------------- COMPUTE PASS -------------------------------
pub struct ComputePassDescription {
    pub bind_group : Vec<Vec<wgpu::BindGroupLayoutEntry>>,
}
pub struct ComputePassData {
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout : Vec<wgpu::BindGroupLayout>
}

pub struct ComputePass {
    desc: ComputePassDescription,
    data: Option<GraphicPassData>
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


// --------------------------- RENDER GRAPH -------------------------------
/*#[derive(Default)]
pub struct RenderGraph {
    graphic_pipeline: HashMap<GraphicPassHandle, GraphicPass>,
}

impl RenderGraph {
    pub fn new() -> Self
    {
        Self {
            graphic_pipeline: HashMap::new()
        }
    }
    pub fn create_graphic_pass(&mut self, device : &wgpu::Device, _description : GraphicPassDescription) -> GraphicPassHandle
    {
        let mut bind_group_layout : Vec<wgpu::BindGroupLayout> = vec![];
        //let mut i = 0;
        for i in 0.._description.bind_group.len() {
            let bind_group = &_description.bind_group[i];
        //for bind_group in _description.bind_group {
            //i = i + 1;
            let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: bind_group.as_ref(),
                label: Some(format!("bind_group_layout{}", i).as_str()),
            });
            bind_group_layout.push(bgl);
        }
        let mut bind_group_layout_ref : Vec<&wgpu::BindGroupLayout> = vec![];
        for bind_group in &bind_group_layout {
            bind_group_layout_ref.push(bind_group);
        }

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()), // TODO: dynamic shader.
        });
        
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bind_group_layout_ref.as_ref(),
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    StaticVertex::desc()
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
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

        let handle = GraphicPassHandle(0); //TODO: generate valid data
        self.graphic_pipeline.entry(handle).or_insert(GraphicPass {
            desc: _description,
            data: Some(GraphicPassData { render_pipeline: render_pipeline, bind_group_layout: bind_group_layout }),
        });
        return handle;
    }
}*/