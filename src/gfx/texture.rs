use std::{fs, path::PathBuf};

use image::GenericImageView;
use anyhow::*;

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

#[derive(Debug, Clone)]
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
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}
#[derive(Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Texture {
    desc: TextureDescription,
    #[cfg_attr(feature = "persistence", serde(skip_serializing, skip_deserializing))]
    data: Option<TextureData>,
    dirty: bool
}

impl Default for TextureDescription {
    fn default() -> Self {
        Self {
            source: TextureSource::default(),
            width:0,
            height:0,
            label: String::from(""),
            srgb:false,
        }
    }
}
impl Default for Texture {
    fn default() -> Self {
        Self {
            desc: TextureDescription::default(),
            data: None,
            dirty: true, // Not created.
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
    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        let src = TextureSource::Bytes(bytes);
        if self.desc.source != src {
            self.desc.source = src;
            self.dirty = true;
        }
    }

    pub fn update_data(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<()> {
        if self.data.is_none() || self.dirty {
            self.dirty = false;
            let data = TextureData::new(device, &self.desc);
            data.write_data(device, queue, &self.desc)?;
            self.data = Some(data);
        }
        Ok(())
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
impl TextureData {
    fn new(device :&wgpu::Device, desc: &TextureDescription) -> Self {
        let size = wgpu::Extent3d {
            width: desc.width,
            height: desc.height,
            depth_or_array_layers: 1,
        };
        let texture_format = if desc.srgb {
            wgpu::TextureFormat::Rgba8Unorm
        } else {
            wgpu::TextureFormat::Rgba8UnormSrgb
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some(desc.label.as_str()),
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
        
        Self { texture, view, sampler }
    }

    fn get_description_from_source(desc : &TextureDescription) -> Result<TextureDescription> {
        match &desc.source {
            TextureSource::None => {
                Ok(desc.clone())
            }
            TextureSource::Bytes(bytes) => {
                assert!((desc.width * desc.height * 4) as usize == bytes.len());
                Ok(desc.clone())
            }
            TextureSource::Path(path) => {
                let bytes = fs::read(path)?;
                Ok(TextureDescription::from_bytes(&bytes, "", false)?)
            }
        }
    }

    fn write_data(&self, device: &wgpu::Device, queue: &wgpu::Queue, desc: &TextureDescription) -> Result<()>
    {
        // Check if we have a source before writing
        if let TextureSource::None = desc.source {
            return Ok(());
        }
        let desc_from_source = Self::get_description_from_source(desc)?;
        let size = wgpu::Extent3d {
            width: desc_from_source.width,
            height: desc_from_source.height,
            depth_or_array_layers: 1,
        };
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            match &desc_from_source.source {
                TextureSource::Bytes(bytes) => bytes.as_ref(),
                _ => unreachable!("Should not reach here")
            },
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * desc.width),
                rows_per_image: std::num::NonZeroU32::new(desc.height),
            },
            size,
        );
        Ok(())
    }
}