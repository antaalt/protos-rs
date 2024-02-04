use core::fmt;

use egui::Vec2;
use egui_node_graph::NodeId;

use crate::{gfx, graph::{core::ProtosGraph, node::OutputsCache, ProtosDataType, ProtosNode, ProtosValueType}};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct ShaderNode {
    shader: gfx::ResourceHandle<gfx::Shader>
}
pub enum ShaderNodeOutput {
    FragmentShader, // TODO ouput single one & fix this
    VertexShader,
}
impl fmt::Display for ShaderNodeOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShaderNodeOutput::FragmentShader => write!(f, "FragmentShader"),
            ShaderNodeOutput::VertexShader => write!(f, "VertexShader"),
        }
    }
}


impl ProtosNode for ShaderNode {
    fn get_name(&self) -> &str {
        "Shader"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_output_param(node_id, ShaderNodeOutput::VertexShader.to_string(), ProtosDataType::Shader);
        graph.add_output_param(node_id, ShaderNodeOutput::FragmentShader.to_string(), ProtosDataType::Shader);
    }
    fn ui(&self, _graph: &ProtosGraph, _node_id: NodeId, ui: &mut egui::Ui) {
        gfx::visit_resource_mut(&self.shader, |shader| {
            shader.visit_desc_mut(|desc| {
                let language = "wgsl";
                let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
                egui_extras::syntax_highlighting::code_view_ui(ui, &theme, desc.shader.as_str(), language);
                ui.text_edit_multiline(&mut desc.shader).changed()
            });
        });
    }
    fn evaluate(
        &self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &ProtosGraph,
        node_id: NodeId,
        _available_size: Vec2,
        outputs_cache: &mut OutputsCache
    ) -> anyhow::Result<()> {
        
        let mut mesh = self.shader.lock().unwrap();
        // TODO set path & shape ?
        mesh.update_data(device, queue)?;
        // TODO workaround this, having a select for shader type ? Or two shader node...
        self.populate_output(graph, node_id, ShaderNodeOutput::VertexShader.to_string(), ProtosValueType::Shader(Some(self.shader.clone())), outputs_cache);
        self.populate_output(graph, node_id, ShaderNodeOutput::FragmentShader.to_string(), ProtosValueType::Shader(Some(self.shader.clone())), outputs_cache);
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