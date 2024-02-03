use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use crate::{gfx, graph::{core::ProtosGraph, node::OutputsCache, ProtosDataType, ProtosNode, ProtosValueType}};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ComputePassNode {
    handle: gfx::ResourceHandle<gfx::ComputePass>
}

impl ProtosNode for ComputePassNode {
    fn get_name(&self) -> &str {
        "Compute pass"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        
        // TODO for loop
        graph.add_input_param(
            node_id,
            "SRV0".to_string(),
            ProtosDataType::Texture,
            ProtosValueType::Texture(None),
            InputParamKind::ConnectionOnly,
            true,
        );
        // TODO for loop
        graph.add_output_param(node_id, "RT0".to_string(), ProtosDataType::Texture);
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
        Ok(())
    }
}