
#[derive(Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BufferDescription {

}
#[derive(Debug)]
pub struct BufferData {

}
#[derive(Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Buffer {
    desc: BufferDescription,
    #[cfg_attr(feature = "persistence", serde(skip_serializing, skip_deserializing))]
    data: Option<BufferData>,
}

impl Default for BufferDescription {
    fn default() -> Self {
        Self {}
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Self {
            desc: BufferDescription::default(),
            data: None,
        }
    }
}
impl BufferData {
    fn new(device: &wgpu::Device, desc: &BufferDescription) -> Self {
        Self {
            
        }
    }
}
impl Buffer {
    pub fn update_data(&mut self, device: &wgpu::Device) {
        if self.data.is_some() {
            self.data = Some(BufferData::new(device, &self.desc));
        } else {
            self.data = Some(BufferData::new(device, &self.desc));
        }
    }
}
