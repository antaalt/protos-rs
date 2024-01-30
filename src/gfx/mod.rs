mod resource;
mod texture;
mod buffer;
mod graphic_pass;
mod compute_pass;
mod backbuffer;

mod camera;
mod mesh;

use std::sync::Arc;
use std::sync::Mutex;

// TODO: unused & duplicated
// Should be replaced by some kind of handle describing the resource within the node.
// We end up with Rc of Rc... but is this an issue ?
pub type ResourceHandle<Type> = Arc<Mutex<Type>>;

pub use self::backbuffer::BackbufferPass;
pub use self::graphic_pass::GraphicPass;
pub use self::compute_pass::ComputePass;
pub use self::buffer::Buffer;
pub use self::texture::Texture;
pub use self::camera::Camera;
pub use self::mesh::Mesh;

pub use self::backbuffer::BackbufferPassDescription;
pub use self::graphic_pass::GraphicPassDescription;
pub use self::graphic_pass::AttachmentDescription;
pub use self::compute_pass::ComputePassDescription;
pub use self::buffer::BufferDescription;
pub use self::texture::TextureDescription;
pub use self::camera::CameraDescription;
pub use self::mesh::MeshDescription;