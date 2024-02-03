use egui::Vec2;
use egui_node_graph::NodeId;

use super::{core::ProtosGraph, node::{ProtosNode, OutputsCache}, ProtosDataType, ProtosValueType};

use crate::gfx::{self, ResourceHandle};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ShaderNode {
    shader: ResourceHandle<gfx::Shader>
}

impl ProtosNode for ShaderNode {
    fn get_name(&self) -> &str {
        "Shader"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_output_param(node_id, "shader".to_string(), ProtosDataType::Shader);
    }
    fn ui(&self, graph: &ProtosGraph, node_id: NodeId, ui: &mut egui::Ui) {
        gfx::visit_resource(self.shader.clone(), |shader| {
            shader.visit_desc_mut(|desc| {
                ui.text_edit_multiline(&mut desc.shader);
            });
        });
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
        
        let mut mesh = self.shader.lock().unwrap();
        // TODO set path & shape ?
        mesh.update_data(device, queue)?;
        // TODO workaround this, having a select for shader type ? Or two shader node...
        self.populate_output(graph, node_id, "Fragment shader", ProtosValueType::Shader { value: Some(self.shader.clone()) }, outputs_cache);
        self.populate_output(graph, node_id, "Vertex shader", ProtosValueType::Shader { value: Some(self.shader.clone()) }, outputs_cache);
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