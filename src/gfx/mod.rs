mod resource;
mod texture;
mod buffer;
mod shader;
mod graphic_pass;
mod compute_pass;
mod backbuffer;

mod camera;
mod mesh;

use std::sync::Arc;
use std::sync::Mutex;

pub type ResourceHandle<Type> = Arc<Mutex<Type>>;

// TODO: should be custom struct instead instead.
pub fn visit_resource<Type>(data: ResourceHandle<Type>, f: impl FnOnce(&mut Type)) {
    let resource = &mut data.lock().expect("Could not lock resource");
    f(resource)
}

pub use self::backbuffer::BackbufferPass;
pub use self::graphic_pass::GraphicPass;
pub use self::compute_pass::ComputePass;
pub use self::buffer::Buffer;
pub use self::texture::Texture;
pub use self::camera::Camera;
pub use self::mesh::Mesh;
pub use self::shader::Shader;

pub use self::backbuffer::BackbufferPassDescription;
pub use self::graphic_pass::GraphicPassDescription;
pub use self::graphic_pass::AttachmentDescription;
pub use self::compute_pass::ComputePassDescription;
pub use self::buffer::BufferDescription;
pub use self::texture::TextureDescription;
pub use self::camera::CameraDescription;
pub use self::mesh::MeshDescription;