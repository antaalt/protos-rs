pub trait ResourceDescTrait : Sized {
    
}
pub trait ResourceDataTrait<Desc : ResourceDescTrait> : Sized {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, desc: &Desc) -> anyhow::Result<Self>;
    fn record_data(&self, device: &wgpu::Device, cmd: &mut wgpu::CommandEncoder, desc: &Desc) -> anyhow::Result<()>;
}

// Needed to avoid Default::default() being called on Data
fn default_data<Desc: ResourceDescTrait + Default, Data : ResourceDataTrait<Desc>>() -> Option<Data> {
    None
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
//#[cfg_attr(feature = "persistence", serde(from = "ResourceDeserialized<Desc>"))]
pub struct Resource<Desc: ResourceDescTrait + Default, Data : ResourceDataTrait<Desc>> {
    pub(super) desc: Desc,
    #[cfg_attr(feature = "persistence", serde(skip, default="default_data"))] pub(super) data: Option<Data>,
    #[cfg_attr(feature = "persistence", serde(skip))] pub(super) dirty: bool
}

/*#[derive(serde::Deserialize)]
struct ResourceDeserialized<Desc: ResourceDescTrait + Default> {
    pub(super) desc: Desc,
}

impl<Desc, Data> From<ResourceDeserialized<Desc>> for Resource<Desc, Data>
where
    Desc: ResourceDescTrait + Default,
    Data: ResourceDataTrait<Desc>
{
    fn from(tmp: ResourceDeserialized<Desc>) -> Self {
        Self {
            desc: tmp.desc,
            data: None,
            dirty: false
        }
    }
}*/

impl<Desc, Data> Default for Resource<Desc, Data> 
where
    Desc: ResourceDescTrait + Default,
    Data: ResourceDataTrait<Desc>
{
    fn default() -> Self {
        Self {
            desc: Default::default(),
            data: None,
            dirty: false,
        }
    }
}
impl<Desc, Data> Resource<Desc, Data> 
where
    Desc: ResourceDescTrait + Default,
    Data: ResourceDataTrait<Desc>,
{
    pub fn update_data(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> anyhow::Result<()> {
        if self.data.is_none() || self.dirty {
            self.dirty = false;
            self.data = Some(Data::new(device, queue, &self.desc)?);
            Ok(())
        } else {
            Ok(())
        }
    }
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
    pub fn record_data(&self, device: &wgpu::Device, cmd: &mut wgpu::CommandEncoder) -> anyhow::Result<()> {
        if let Some(data) = &self.data {
            data.record_data(device, cmd, &self.desc)
        } else {
            anyhow::bail!("No data")
        }
    }
    pub fn visit_desc(&self, f: impl FnOnce(&Desc)) {
        f(&self.desc)
    }
    pub fn visit_desc_mut(&mut self, f: impl FnOnce(&mut Desc)) {
        f(&mut self.desc)
    }
    pub fn visit_data(&self, f: impl FnOnce(&Data)) -> bool {
        if let Some(data) = &self.data {
            f(&data);
            true
        } else {
            false
        }
    }
}