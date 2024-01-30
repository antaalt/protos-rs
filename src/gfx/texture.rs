use std::{fs, path::PathBuf};

use image::GenericImageView;
use anyhow::*;

use super::resource::{ResourceDataTrait, ResourceDescTrait, Resource};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
enum TextureSource {
    None,
    Bytes(Vec<u8>),
    Path(PathBuf)
}
impl Default for TextureSource {
    fn default() -> Self {
        TextureSource::None
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TextureDescription {
    source: TextureSource,
    width: u32,
    height: u32,
    label: String,
    srgb: bool, // TODO: flags
}

#[derive(Debug)]
pub struct TextureData {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

pub type Texture = Resource<TextureDescription, TextureData>;

impl ResourceDescTrait for TextureDescription {
    
}


impl ResourceDataTrait<TextureDescription> for TextureData {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, desc: &TextureDescription) -> anyhow::Result<Self> {
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let desc_from_src : TextureDescription = match &desc.source {
            TextureSource::None => {
                desc.clone()
            }
            TextureSource::Bytes(bytes) => {
                assert!((desc.width * desc.height * 4) as usize == bytes.len());
                desc.clone()
            }
            TextureSource::Path(path) => {
                let bytes = fs::read(path)?;
                TextureDescription::from_bytes(&bytes, "", false)?
            }
        };
        let size = wgpu::Extent3d {
            width: desc_from_src.width,
            height: desc_from_src.height,
            depth_or_array_layers: 1,
        };
        let texture_format = if desc_from_src.srgb {
            wgpu::TextureFormat::Rgba8Unorm
        } else {
            wgpu::TextureFormat::Rgba8UnormSrgb
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some(desc_from_src.label.as_str()),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: texture_format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            }
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );
        // TODO: should handle async
        let validation = pollster::block_on(device.pop_error_scope()).and_then(|err| {
            Some(err)
        });
        if validation.is_some() {
            anyhow::bail!(validation.unwrap().to_string())
        } else {
            if let TextureSource::None = desc.source {
                Ok(Self {
                    texture,
                    view,
                    sampler,
                })
            } else {
                let size = wgpu::Extent3d {
                    width: desc_from_src.width,
                    height: desc_from_src.height,
                    depth_or_array_layers: 1,
                };
                queue.write_texture(
                    wgpu::ImageCopyTexture {
                        aspect: wgpu::TextureAspect::All,
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                    },
                    match &desc_from_src.source {
                        TextureSource::Bytes(bytes) => bytes.as_ref(),
                        _ => unreachable!("Should not reach here")
                    },
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: std::num::NonZeroU32::new(4 * desc_from_src.width),
                        rows_per_image: std::num::NonZeroU32::new(desc_from_src.height),
                    },
                    size,
                );
                Ok(Self {
                    texture,
                    view,
                    sampler,
                })
            }
        }
    }
}

impl Texture {    
    pub fn get_handle(&self) -> anyhow::Result<&wgpu::Texture> {
        if self.data.is_some() {
            Ok(&self.data.as_ref().unwrap().texture)
        } else {
            Err(anyhow!("No data"))
        }
    }
    pub fn get_view_handle(&self) -> anyhow::Result<&wgpu::TextureView> {
        if self.data.is_some() {
            Ok(&self.data.as_ref().unwrap().view)
        } else {
            Err(anyhow!("No data"))
        }
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.set_width(width);
        self.set_height(height);
    }
    pub fn set_width(&mut self, width: u32) {
        if self.desc.width != width {
            self.desc.width = width;
            self.dirty = true;
        }
    }
    pub fn set_height(&mut self, height: u32) {
        if self.desc.height != height {
            self.desc.height = height;
            self.dirty = true;
        }
    }
    pub fn set_path(&mut self, path: PathBuf) {
        let src = TextureSource::Path(path);
        if self.desc.source != src {
            self.desc.source = src;
            self.dirty = true;
        }
    }
    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        let src = TextureSource::Bytes(bytes);
        if self.desc.source != src {
            self.desc.source = src;
            self.dirty = true;
        }
    }
}
impl TextureDescription {

    pub fn from_bytes(
        bytes: &[u8], 
        label: &str,
        srgb: bool,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(&img, Some(label), srgb)
    }

    pub fn from_image(
        img: &image::DynamicImage,
        label: Option<&str>,
        srgb: bool,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        Self::from_raw_memory(&rgba, dimensions, label, srgb)
    }
    pub fn from_raw_memory(
        rgba: &[u8],
        dimensions: (u32, u32),
        label: Option<&str>,
        srgb: bool,
    ) -> Result<Self> {
        Ok(Self {
            source: TextureSource::Bytes(Vec::from(rgba)),
            width: dimensions.0,
            height: dimensions.1,
            label: String::from(if label.is_some() { label.unwrap() } else { "UNKNOWN" }),
            srgb,
        })
    }
    pub fn default_black_texture() -> Result<Self> {
        // TODO cache output.
        let rgba = vec![0, 0, 0, 255];
        let dimensions = (1, 1);
        Self::from_raw_memory(&rgba[..], dimensions, "DefaultBlackTexture".into(), true)
    }
    
    pub fn default_white_texture() -> Result<Self> {
        // TODO cache output.
        let rgba = vec![255, 255, 255, 255];
        let dimensions = (1, 1);
        Self::from_raw_memory(&rgba[..], dimensions, "DefaultBlackTexture".into(), true)
    }

    pub fn default_missing_texture() -> Result<Self> {
        // TODO cache output.
        let rgba = vec![255, 0, 255, 255];
        let dimensions = (1, 1);
        Self::from_raw_memory(&rgba[..], dimensions, "DefaultBlackTexture".into(), true)
    }
    
    pub fn default_normal_texture() -> Result<Self> {
        // TODO cache output.
        let rgba = vec![0, 127, 0, 255];
        let dimensions = (1, 1);
        Self::from_raw_memory(&rgba[..], dimensions, "DefaultNormalTexture".into(), false)
    }
}