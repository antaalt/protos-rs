use std::sync::{Arc, Mutex};

pub type ResourceHandle<Type> = Arc<Mutex<Type>>;


pub trait Recordable {
    fn record_data(&self, device: &wgpu::Device, cmd: &wgpu::CommandEncoder);
}

pub trait Updatable {
    fn update_data(&self, device: &wgpu::Device, cmd: &wgpu::CommandEncoder);
}