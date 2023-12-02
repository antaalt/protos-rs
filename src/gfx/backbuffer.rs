use std::sync::Arc;
use std::sync::Mutex;
use wgpu::{ImageCopyTexture, Extent3d};
use anyhow::*;

use super::{core::ResourceHandle, Texture};


pub struct BackbufferPassDescription {
    origin : Option<ResourceHandle<Texture>>,
    width: u32,
    height: u32,
}
pub struct BackbufferPassData {
    target : Option<Texture>,
    //render_pipeline: wgpu::RenderPipeline,
    //bind_group_layout : Vec<wgpu::BindGroupLayout>
}
pub struct BackbufferPass {
    desc: BackbufferPassDescription,
    data: Option<BackbufferPassData>
}

impl BackbufferPass {
    pub fn set_origin(&mut self, origin: Option<ResourceHandle<Texture>>) {
        self.desc.origin = origin;
    }
    pub fn clear_origin(&mut self) {
        self.set_origin(None);
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.desc.width = width;
        self.desc.height = height;
    }

    pub fn get_view_handle(&self) -> anyhow::Result<&wgpu::TextureView> {
        if self.data.is_some() {
            let d = self.data.as_ref().unwrap();
            if d.target.is_some() {
                let dd = d.target.as_ref().unwrap();
                dd.get_view_handle()
            } else {
                Err(anyhow!("No data"))
            }
        } else {
            Err(anyhow!("No data"))
        }
    }

    pub fn update_data(&mut self, device: &wgpu::Device) {
        self.data = Some(BackbufferPassData::new(device, &self.desc));
    }
    pub fn record_data(&self, device: &wgpu::Device, cmd: &mut wgpu::CommandEncoder) {
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

impl BackbufferPassData {
    pub fn new(device: &wgpu::Device, desc: &BackbufferPassDescription) -> Self {
        let mut texture = Texture::default();
        texture.set_size(desc.width, desc.height);
        texture.update_data(device);
        Self {
            target: Some(texture),
        }
    }
}

impl Default for BackbufferPassDescription {
    fn default() -> Self {
        Self {
            origin: None,
            width: 0,
            height: 0,
        }
    }
}
impl Default for BackbufferPass {
    fn default() -> Self {
        Self {
            desc: BackbufferPassDescription::default(),
            data: None,
        }
    }
}