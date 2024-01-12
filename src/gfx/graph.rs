

// Graph in rust are complex because of lifetime & mutability requirement. Check these links
// https://github.com/nrc/r4cppp/blob/master/graphs/README.md
// http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

// Should go with something such as daggy
// recurse all nodes.
// create graphic pass : require to read arguments
pub struct Node {

}

pub struct Graph {

}

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