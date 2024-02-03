use egui::Vec2;
use egui_node_graph::NodeId;

use super::{core::ProtosGraph, node::{ProtosNode, OutputsCache}, ProtosDataType, ProtosValueType};

use crate::gfx;

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct MeshNode {
    mesh: gfx::ResourceHandle<gfx::Mesh>
}

impl ProtosNode for MeshNode {
    fn get_name(&self) -> &str {
        "Mesh"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        
        graph.add_output_param(node_id, "Geometry".to_string(), ProtosDataType::Mesh);
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
        
        let mut mesh = self.mesh.lock().unwrap();
        // TODO set path & shape ?
        mesh.update_data(device, queue)?;
        self.populate_output(graph, node_id, "Geometry", ProtosValueType::Mesh { value: Some(self.mesh.clone()) }, outputs_cache);
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