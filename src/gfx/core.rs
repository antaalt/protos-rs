use std::sync::{Arc, Mutex};

// --------------------------- HANDLES -------------------------------
/*#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
}*/

pub type ResourceHandle<Type> = Arc<Mutex<Type>>;
