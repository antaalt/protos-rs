use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, node::{ProtosNode, OutputsCache}};

use crate::gfx;

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
            ProtosValueType::Texture { value: None },
            InputParamKind::ConnectionOnly,
            true,
        );
        // TODO for loop
        graph.add_output_param(node_id, "RT0".to_string(), ProtosDataType::Texture);
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
        Ok(())
    }
}