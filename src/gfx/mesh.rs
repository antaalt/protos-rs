use std::mem;

use super::resource::{Resource, ResourceDataTrait, ResourceDescTrait};


pub enum VertexFactory {
    Static, // Vertex layout for static mesh.
}
pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
    fn factory() -> VertexFactory;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
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

#[derive(Debug, Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct MeshDescription {
    pub(crate) vertices: Vec<StaticVertex>,
    pub(crate) indices: Vec<u32>,
}
#[derive(Debug)]
pub struct MeshData {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
}

pub type Mesh = Resource<MeshDescription, MeshData>;

impl ResourceDescTrait for MeshDescription {
    
}

impl ResourceDataTrait<MeshDescription> for MeshData {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, desc: &MeshDescription) -> anyhow::Result<Self> {
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("indices"),
            size: desc.vertices.len() as wgpu::BufferAddress * mem::size_of::<StaticVertex>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("indices"),
            size: desc.indices.len() as wgpu::BufferAddress * mem::size_of::<u32>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::INDEX,
            mapped_at_creation: false,
        });
        Ok(Self {
            vertex_buffer,
            index_buffer,
        })
    }
    fn record_data(&self, _device: &wgpu::Device, _cmd: &mut wgpu::CommandEncoder, _desc: &MeshDescription) -> anyhow::Result<()> {
        Ok(()) // Nothing to do here
    }
}

impl Mesh {
    
}