use egui::Vec2;
use egui_node_graph::NodeId;

use super::{core::ProtosGraph, node::{OutputsCache, ProtosNode}};

use crate::gfx;

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct CameraNode {
    handle: gfx::ResourceHandle<gfx::Camera>
}

impl ProtosNode for CameraNode {
    fn get_name(&self) -> &str {
        "Camera"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
    }
    fn ui(&self, graph: &ProtosGraph, node_id: NodeId, ui: &mut egui::Ui) {
        
    }
    fn evaluate(
        &self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &ProtosGraph,
        node_id: NodeId,
        available_size: Vec2,
        outputs_cache: &mut OutputsCache
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn record(
        &self,
        device: &wgpu::Device,
        cmd: &mut wgpu::CommandEncoder,
        graph: &ProtosGraph,
        node_id: NodeId,
        outputs_cache: &mut OutputsCache
    ) -> anyhow::Result<()> {
        Ok(()) // Nothing to record here
    }
}