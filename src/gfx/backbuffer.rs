use std::sync::Arc;

use super::{resource::{Resource, ResourceDataTrait, ResourceDescTrait,}, Texture, ResourceHandle};


#[derive(Debug, Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BackbufferPassDescription {
    #[cfg_attr(feature = "persistence", serde(skip))]
    origin : Option<ResourceHandle<Texture>>, // TODO: serialize this ?
    width: u32,
    height: u32,
}
#[derive(Debug, Default)]
pub struct BackbufferPassData {
    target : Option<Texture>,
}

pub type BackbufferPass = Resource<BackbufferPassDescription, BackbufferPassData>;

impl ResourceDescTrait for BackbufferPassDescription {
    
}

impl ResourceDataTrait<BackbufferPassDescription> for BackbufferPassData {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, desc: &BackbufferPassDescription) -> anyhow::Result<Self> {
        let mut texture = Texture::default();
        texture.set_size(desc.width, desc.height);
        texture.update_data(device, queue)?;
        println!("Backbuffer created.");
        Ok(Self {
            target: Some(texture),
        })
    }
}

impl BackbufferPass {
    pub fn set_origin(&mut self, origin: Option<ResourceHandle<Texture>>) {
        if self.desc.origin.is_some() && origin.is_some() {
            let old_origin = self.desc.origin.as_ref().unwrap();
            let new_origin = origin.as_ref().unwrap();
            if !Arc::ptr_eq(old_origin, new_origin) {
                self.dirty = true;
            }
            self.desc.origin = origin;
        } else {
            self.desc.origin = origin;
            self.dirty = true;
        }
    }
    pub fn clear_origin(&mut self) {
        self.set_origin(None);
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        if self.desc.width != width || self.desc.height != height {
            self.desc.width = width;
            self.desc.height = height;
            self.dirty = true;
        }
    }
    pub fn get_width(&self) -> u32 {
        self.desc.width
    }
    pub fn get_height(&self) -> u32 {
        self.desc.height
    }
    pub fn get_view_handle(&self) -> anyhow::Result<&wgpu::TextureView> {
        if self.data.is_some() {
            let d = self.data.as_ref().unwrap();
            if d.target.is_some() {
                let dd = d.target.as_ref().unwrap();
                dd.get_view_handle()
            } else {
                anyhow::bail!("No data")
            }
        } else {
            anyhow::bail!("No data")
        }
    }
    pub fn has_data(&self) -> bool {
        return self.data.is_some();
    }
    // TODO: move this in node_backbuffer
    pub fn record_data(&self, device: &wgpu::Device, cmd: &mut wgpu::CommandEncoder) {
        if self.desc.origin.is_some() {
            let origin_locked = self.desc.origin.as_ref().unwrap().lock();
            let origin = origin_locked.unwrap();
            let target_locked = self.data.as_ref().unwrap().target.as_ref();
            let target = target_locked.unwrap();
            // Copy target to final storage.
            let src = wgpu::ImageCopyTexture{ 
                texture: origin.get_handle().expect("Origin not loaded"),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            };
            let dst = wgpu::ImageCopyTexture{ 
                texture: target.get_handle().expect("Target not loaded"),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            };
            
            cmd.copy_texture_to_texture(src, dst, wgpu::Extent3d {
                width: self.desc.width,
                height: self.desc.height,
                depth_or_array_layers:1,
            });
        }
    }
}