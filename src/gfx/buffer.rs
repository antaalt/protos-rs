
#[derive(Debug)]
pub struct BufferDescription {

}
#[derive(Debug)]
pub struct BufferData {

}
#[derive(Debug)]
pub struct Buffer {
    desc: BufferDescription,
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
