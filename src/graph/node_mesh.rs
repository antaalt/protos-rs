use egui::Vec2;
use egui_node_graph::NodeId;

use super::{core::ProtosGraph, node::{ProtosNode, OutputsCache}};

use crate::gfx;

#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct MeshNode {
    mesh: gfx::ResourceHandle<gfx::Mesh>
}

impl MeshNode {
    pub fn new(mesh: gfx::ResourceHandle<gfx::Mesh>) -> Self {
        Self {
            mesh
        }
    }
}

impl ProtosNode for MeshNode {
    fn get_name(&self) -> &str {
        "Mesh"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
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