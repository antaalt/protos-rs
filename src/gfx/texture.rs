use image::{GenericImageView};
use anyhow::*;

#[derive(Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TextureDescription {
    data: Vec<u8>,
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
}

impl Default for TextureDescription {
    fn default() -> Self {
        Self {
            data: Vec::new(),
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
        self.desc.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.desc.height = height;
    }

    pub fn update_data(&mut self, device: &wgpu::Device) {
        if self.data.is_some() {
            self.data = Some(TextureData::new(device, &self.desc))
        } else {
            self.data = Some(TextureData::new(device, &self.desc))
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
            data: Vec::from(rgba),
            width: dimensions.0,
            height: dimensions.1,
            label: String::from("UNKNOWN"),
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
        let texture_view_format = texture_format;
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some(desc.label.as_str()),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: texture_format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
                //view_formats: &[texture_view_format],
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
    fn write(&self, device: &wgpu::Device, queue: &wgpu::Queue, desc: &TextureDescription)
    {
        let size = wgpu::Extent3d {
            width: desc.width,
            height: desc.height,
            depth_or_array_layers: 1,
        };
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            desc.data.as_ref(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * desc.width),
                rows_per_image: std::num::NonZeroU32::new(desc.height),
            },
            size,
        );
    
    }
}