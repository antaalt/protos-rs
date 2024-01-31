use std::{any, sync::Arc};

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
        Ok(Self {
            target: Some(texture),
        })
    }
    fn record_data(&self, device: &wgpu::Device, cmd: &mut wgpu::CommandEncoder, desc: &BackbufferPassDescription) -> anyhow::Result<()> {
        if let Some(origin_locked) = &desc.origin {
            if let Some(target) = &self.target {
                let origin = origin_locked.lock().unwrap();
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
                    width: desc.width,
                    height: desc.height,
                    depth_or_array_layers:1,
                });
                Ok(())
            } else {
                anyhow::bail!("No target in backbuffer");
            }
        } else {
            anyhow::bail!("No origin in backbuffer");
        }
    }
}

impl BackbufferPass {
    pub fn set_origin(&mut self, origin: ResourceHandle<Texture>) {
        if let Some(old_origin) = &self.desc.origin {
            if !Arc::ptr_eq(old_origin, &origin) {
                self.desc.origin = Some(origin);
                self.dirty = true;
            }
        } else {
            self.desc.origin = Some(origin);
            self.dirty = true;
        }
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
        if let Some(data) = &self.data {
            if let Some(target) = &data.target {
                target.get_view_handle()
            } else {
                anyhow::bail!("No data")
            }
        } else {
            anyhow::bail!("No data")
        }
    }
}