use egui::Vec2;
use egui_node_graph::NodeId;

use crate::{gfx, graph::{core::ProtosGraph, node::OutputsCache, ProtosNode}};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct CameraNode {
    handle: gfx::ResourceHandle<gfx::Camera>
}

impl ProtosNode for CameraNode {
    fn get_name(&self) -> &str {
        "Camera"
    }
    fn build(&self, _graph: &mut ProtosGraph, _node_id: NodeId) {
    }
    fn ui(&self, _graph: &ProtosGraph, _node_id: NodeId, _ui: &mut egui::Ui) {
        
    }
    fn evaluate(
        &self, 
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _graph: &ProtosGraph,
        _node_id: NodeId,
        _available_size: Vec2,
        _outputs_cache: &mut OutputsCache
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn record(
        &self,
        _device: &wgpu::Device,
        _cmd: &mut wgpu::CommandEncoder,
        _graph: &ProtosGraph,
        _node_id: NodeId,
        _outputs_cache: &mut OutputsCache
    ) -> anyhow::Result<()> {
        Ok(()) // Nothing to record here
    }
}