use std::borrow::Cow;
use std::sync::Arc;
use std::sync::Mutex;

use wgpu::RenderPassDescriptor;

use super::mesh::StaticVertex;
use super::mesh::Vertex;
use super::resource::Resource;
use super::resource::ResourceDataTrait;
use super::resource::ResourceDescTrait;
use super::Mesh;
use super::ResourceHandle;
use super::texture::*;
use super::Shader;

fn default_bind_group_entry(index : u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding: index,
        visibility:wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture { 
            sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
            view_dimension: wgpu::TextureViewDimension::D2, 
            multisampled: false
        },
        count: None
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct AttachmentDescription {
    width: u32,
    height: u32,
}

impl AttachmentDescription {
    pub fn set_size(&mut self, width: u32, height: u32)  {
        self.width = width;
        self.height = height;
    }
}

#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct GraphicPassDescription {
    geometry: Option<ResourceHandle<Mesh>>,
    bind_group : Vec<Vec<wgpu::BindGroupLayoutEntry>>,
    render_target_desc: Vec<AttachmentDescription>,
    shader_resource_view: Vec<Option<ResourceHandle<Texture>>>,
    vertex_shader: Option<ResourceHandle<Shader>>,
    fragment_shader: Option<ResourceHandle<Shader>>,
}
pub struct GraphicPassData {
    render_pipeline: wgpu::RenderPipeline,
    bind_group : wgpu::BindGroup,
    render_targets: Vec<ResourceHandle<Texture>>,
}

pub type GraphicPass = Resource<GraphicPassDescription, GraphicPassData>;

impl ResourceDescTrait for GraphicPassDescription {
    
}

impl ResourceDataTrait<GraphicPassDescription> for GraphicPassData {
    fn new<'a>(device: &wgpu::Device, queue: &wgpu::Queue, desc: &GraphicPassDescription) -> anyhow::Result<Self> {
        // TODO: handle multiple bind group
        // TODO: handle other types that texture with an enum.
        // Create bind groups.
        let mut binding = 0;
		let mut bind_group_layout_entry = Vec::new();
		let mut bind_group_entry = Vec::new();
        // Check inputs.
		for resource in &desc.shader_resource_view {
            if resource.is_none() {
                anyhow::bail!("Resource binding not set")
            }
        }
        // Store locks to keep their lifetime for create_bind_group
        let resources = desc.shader_resource_view.iter().map(|value| value.as_ref().unwrap()).collect::<Vec<_>>();
        let resources_locked = resources.iter().map(|value| value.lock().unwrap()).collect::<Vec<_>>();
		for resource_locked in &resources_locked {
			bind_group_layout_entry.push(wgpu::BindGroupLayoutEntry {
				binding: binding,
				visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Texture { 
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                    view_dimension: wgpu::TextureViewDimension::D2, 
                    multisampled: false
                },
				count: None,
			});            
            bind_group_entry.push(wgpu::BindGroupEntry {
                binding: binding,
                resource: wgpu::BindingResource::TextureView(resource_locked.get_view_handle()?) 
            });
            binding += 1;
		}
		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("GraphicPassBindGroupLayout"),
			entries: bind_group_layout_entry.as_slice(),
		});
		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("GraphicPassBindGroup"),
			layout: &bind_group_layout,
			entries: bind_group_entry.as_slice(),
		});

        // Create attachments
        let mut render_targets = Vec::new();
        let mut render_targets_state = Vec::new();
        for render_target in &desc.render_target_desc {
            render_targets_state.push(Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL
            }));
            let mut attachment = Texture::default();
            attachment.set_size(render_target.width, render_target.height);
            attachment.update_data(device, queue).expect("Should not fail here");
            render_targets.push(Arc::new(Mutex::new(attachment)));
        }
        if desc.vertex_shader.is_none() {
            anyhow::bail!("No vertex shader")
        }
        if desc.fragment_shader.is_none() {
            anyhow::bail!("No fragment shader")
        }
        // Create shaders
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let (vertex_shader, fragment_shader) = {
            ({
                let vertex_shader_source_locked = desc.vertex_shader.as_ref().unwrap();
                let vertex_shader_source = vertex_shader_source_locked.lock().unwrap();
                device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("VertexShader"),
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(vertex_shader_source.desc.shader.as_str())),
                })
            }, {
                let fragment_shader_source_locked = desc.fragment_shader.as_ref().unwrap();
                let fragment_shader_source = fragment_shader_source_locked.lock().unwrap();
                device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("FragmentShader"),
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(fragment_shader_source.desc.shader.as_str())),
                })
            })
        };
        // Create pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[], // TODO: push constant
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[
                    StaticVertex::desc(),
                    //Instance::desc() // TODO: get this from rust-engine
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                targets: render_targets_state.as_ref(),
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

        let validation = pollster::block_on(device.pop_error_scope()).and_then(|err| {
            Some(err)
        });
        if let Some(v) = validation {
            anyhow::bail!(v.to_string())
        } else {
            Ok(Self { 
                render_pipeline, 
                render_targets,
                bind_group
            })
        }
        
    }
    fn record_data(&self, _device : &wgpu::Device, cmd: &mut wgpu::CommandEncoder, desc: &GraphicPassDescription) -> anyhow::Result<()> {

        // Store locks to keep their lifetime for create_bind_group
        let mut color_attachments = Vec::new();
        let resources_locked = self.render_targets.iter().map(|value| value.lock().unwrap()).collect::<Vec<_>>();
        for resource_locked in &resources_locked {
            let value = resource_locked.get_view_handle()?;
            color_attachments.push(Some(wgpu::RenderPassColorAttachment {
                view: value,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                }
            }));
        }
        if let Some(geometry) = &desc.geometry {
            let geo = geometry.lock().unwrap();
            if let Some(data) = &geo.data {
                let mut render_pass = cmd.begin_render_pass(&RenderPassDescriptor{
                    label: Some("render_pass_random"),
                    color_attachments: &color_attachments.as_ref(),
                    depth_stencil_attachment: None, /*Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),*/
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
                render_pass.set_index_buffer(data.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.set_vertex_buffer(0, data.vertex_buffer.slice(..));
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.draw_indexed(0..data.index_count, 0, 0..1);
                Ok(())
            } else {
                anyhow::bail!("No geometry data")
            }
        } else {
            anyhow::bail!("No geometry")
        }
    }
}

impl GraphicPass {
    pub fn set_shader_resource_view(&mut self, index: u32, srv : Option<ResourceHandle<Texture>>) {
        // TODO: resize should be done by desc data ? or data is built by shader resource view that are set ? 
        if index as usize >= self.desc.shader_resource_view.len() {
            self.desc.shader_resource_view.resize(index as usize + 1, None);
            self.desc.bind_group.resize(1, Vec::new());
            self.desc.bind_group[0].resize(index as usize + 1, default_bind_group_entry(index));
            self.dirty = true;
        }
        if srv.is_some() {          
            self.desc.bind_group[0][index as usize].binding = index;
            self.desc.bind_group[0][index as usize].count = None;
            self.desc.bind_group[0][index as usize].ty = wgpu::BindingType::Texture { 
                sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                view_dimension: wgpu::TextureViewDimension::D2, // TODO: desc fitting texture 
                multisampled: false
            };
            self.desc.bind_group[0][index as usize].visibility = wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT;
            //self.dirty = true; // TODO: set dirty accordingly.
        } else {
            let default = default_bind_group_entry(index);
            if self.desc.bind_group[0][index as usize] != default {
                self.desc.bind_group[0][index as usize] = default;
                self.dirty = true;
            }
        }
        self.desc.shader_resource_view[index as usize] = srv;
    }
    pub fn set_render_target(&mut self, index: u32, width : u32, height: u32) {
        let mut rt = AttachmentDescription::default();
        rt.set_size(width, height);
        if index as usize >= self.desc.render_target_desc.len() {
            self.desc.render_target_desc.resize(index as usize + 1, AttachmentDescription::default());
            self.dirty = true;
        }
        if self.desc.render_target_desc[index as usize] != rt {
            self.desc.render_target_desc[index as usize] = rt.clone();
            self.dirty = true;
        }
    }
    pub fn set_geometry(&mut self, geometry: ResourceHandle<Mesh>) {
        if self.desc.geometry.is_some() && Arc::ptr_eq(&self.desc.geometry.as_ref().unwrap(), &geometry)  {
            self.dirty = true;
        }
        self.desc.geometry = Some(geometry);
    }
    pub fn set_vertex_shader(&mut self, vertex_shader: ResourceHandle<Shader>) {
        if self.desc.vertex_shader.is_some() && Arc::ptr_eq(&self.desc.vertex_shader.as_ref().unwrap(), &vertex_shader)  {
            self.dirty = true;
        }
        self.desc.vertex_shader = Some(vertex_shader);
    }
    pub fn set_fragment_shader(&mut self, fragment_shader: ResourceHandle<Shader>) {
        if self.desc.fragment_shader.is_some() && Arc::ptr_eq(&self.desc.fragment_shader.as_ref().unwrap(), &fragment_shader)  {
            self.dirty = true;
        }
        self.desc.fragment_shader = Some(fragment_shader);
    }
    pub fn get_render_target(&self, index: u32) -> Option<ResourceHandle<Texture>> {
        if self.data.is_some() {
            Some(self.data.as_ref().unwrap().render_targets[index as usize].clone())
        } else {
            None
        }
    }
}