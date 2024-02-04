use std::{f32::consts::PI, mem, path::PathBuf};

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

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum MeshShape {
    Sphere{ segment_count: u32, ring_count: u32, radius: f32 },
    Cube{ size: f32 }
}
impl MeshShape {
    pub fn default_sphere() -> Self {
        MeshShape::Sphere { segment_count: 16, ring_count: 16, radius: 0.5 }
    }
    pub fn default_cube() -> Self {
        MeshShape::Cube { size: 1.0, }
    }
    pub fn get_shape_name(&self) -> &str {
        match self {
            MeshShape::Sphere { .. } => "Sphere",
            MeshShape::Cube { .. } => "Cube",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum MeshSource {
    None,
    Shape(MeshShape),
    Path(PathBuf)
}
impl MeshSource {
    pub fn get_source_name(&self) -> &str {
        match self {
            MeshSource::None => "None",
            MeshSource::Shape(..) => "Shape",
            MeshSource::Path(..) => "Path",
        }
    }
}

impl Default for MeshSource {
    fn default() -> Self {
        MeshSource::None
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct MeshDescription {
    pub(crate) source: MeshSource,
}
#[derive(Debug)]
pub struct MeshData {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) index_count: u32,
}

pub type Mesh = Resource<MeshDescription, MeshData>;

impl ResourceDescTrait for MeshDescription {
    
}

impl ResourceDataTrait<MeshDescription> for MeshData {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, desc: &MeshDescription) -> anyhow::Result<Self> {
        let (vertices, indices) = match &desc.source {
            MeshSource::Shape(shape) => {
                match shape {
                    MeshShape::Cube{ size } => {
                        let mut vertices = Vec::new();
                        let mut indices = Vec::new();
                        let l = size / 2 as f32;
                        let origin = 0.0;
                        for i in 0..8 {
                            vertices.push(StaticVertex {
                                position: [
                                    if (i & 4) != 0 { l } else { -l } + origin,
                                    if (i & 2) != 0 { l } else { -l } + origin,
                                    if (i & 1) != 0 { l } else { -l } + origin
                                ],
                                // With only 8 vertices, cant store normal... Fix this.
                                normal: [
                                    1.0,
                                    0.0,
                                    0.0
                                ],
                                tangent: [0.0, 0.0, 1.0], // TODO:
                                bitangent: [0.0, 0.0, 1.0], // TODO:
                                tex_coords:[0.0, 1.0], // Same than normal...
                                color:[1.0, 1.0, 1.0, 1.0],
                            });
                        }

                        for i in 0..3 {
                            let v1 = 1 << i;
                            let v2 = if v1 == 4 { 1 } else { v1 << 1 };
                            indices.push(0);
                            indices.push(v1);
                            indices.push(v2);
                            indices.push(v1 + v2);
                            indices.push(v2);
                            indices.push(v1);
                            indices.push(7);
                            indices.push(7 - v2);
                            indices.push(7 - v1);
                            indices.push(7 - (v1 + v2));
                            indices.push(7 - v1);
                            indices.push(7 - v2);
                            // i'm using [7 - ] instead of [~] because the only bits
                            // that need to be affected are the least relevant three
                            // and in C#, that' s the only way to do that.
                        }
                        (vertices, indices)
                    }
                    MeshShape::Sphere{segment_count, ring_count, radius} => {
                        let mut vertices = Vec::new();
                        let mut indices = Vec::new();
                        // http://www.songho.ca/opengl/gl_sphere.html
                        let sector_step = 2.0 * PI / (*ring_count as f32);
                        let stack_step = PI / (*segment_count as f32);

                        for i in 0..=*segment_count {
                            let segment_angle = PI / 2.0 - i as f32 * stack_step; // starting from pi/2 to -pi/2
                            let xy = radius * segment_angle.cos();
                            let z = radius * segment_angle.sin();

                            // add (ring_count+1) vertices per segment
                            // the first and last vertices have same position and normal, but different uv
                            for j in 0..=*ring_count {
                                let ring_angle = j as f32 * sector_step; // starting from 0 to 2pi

                                vertices.push(StaticVertex {
                                    position: [
                                        xy * ring_angle.cos(),
                                        xy * ring_angle.sin(),
                                        z
                                    ],
                                    normal: [
                                        (xy * ring_angle.cos()) / radius,
                                        (xy * ring_angle.sin()) / radius,
                                        z / radius
                                    ],
                                    tangent: [0.0, 0.0, 1.0], // TODO:
                                    bitangent: [0.0, 0.0, 1.0], // TODO:
                                    tex_coords:[(j / ring_count) as f32, (i / segment_count) as f32],
                                    color:[1.0, 1.0, 1.0, 1.0],
                                });
                            }
                        }
                        for i in 0..*segment_count {
                            let mut k1 = i * (ring_count + 1);     // beginning of current stack
                            let mut k2 = k1 + ring_count + 1;      // beginning of next stack

                            for _ in 0..*ring_count {
                                // 2 triangles per sector excluding first and last stacks
                                // k1 => k2 => k1+1
                                if i != 0
                                {
                                    indices.push(k1);
                                    indices.push(k2);
                                    indices.push(k1 + 1);
                                }
                                // k1+1 => k2 => k2+1
                                if i != (segment_count - 1)
                                {
                                    indices.push(k1 + 1);
                                    indices.push(k2);
                                    indices.push(k2 + 1);
                                }
                                k1 += 1;
                                k2 += 1;
                            }
                        }
                        // TODO create sphere
                        (vertices, indices)
                    }
                }
            },
            MeshSource::Path(_) => {
                // TODO: mesh obj & co
                unimplemented!("TODO:");
            } 
            _ => { anyhow::bail!("Invalid mesh source") }
        };
        
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertices"),
            size: vertices.len() as wgpu::BufferAddress * mem::size_of::<StaticVertex>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("indices"),
            size: indices.len() as wgpu::BufferAddress * mem::size_of::<u32>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(vertices.as_slice()));
        queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(indices.as_slice()));
        Ok(Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        })
    }
    fn record_data(&self, _device: &wgpu::Device, _cmd: &mut wgpu::CommandEncoder, _desc: &MeshDescription) -> anyhow::Result<()> {
        Ok(()) // Nothing to do here
    }
}

impl Mesh {
    
}