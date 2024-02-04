use core::fmt;

use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use crate::{gfx, graph::{core::ProtosGraph, node::OutputsCache, ProtosDataType, ProtosNode, ProtosValueType}};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct TextureResourceNode {
    handle: gfx::ResourceHandle<gfx::Texture>
}

pub enum TextureResourceNodeInput {
    Dimensions,
}
pub enum TextureResourceNodeOutput {
    Texture,
}
impl fmt::Display for TextureResourceNodeInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextureResourceNodeInput::Dimensions => write!(f, "Dimensions"),
        }
    }
}
impl fmt::Display for TextureResourceNodeOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextureResourceNodeOutput::Texture => write!(f, "texture"),
        }
    }
}

impl ProtosNode for TextureResourceNode {
    fn get_name(&self) -> &str {
        "ResourceTexture"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_input_param(
            node_id.clone(),
            TextureResourceNodeInput::Dimensions.to_string(),
            ProtosDataType::Vec2,
            ProtosValueType::Vec2([100.0, 100.0]),
            InputParamKind::ConstantOnly,
            true,
        );
        graph.add_output_param(
            node_id.clone(), 
            TextureResourceNodeOutput::Texture.to_string(),
            ProtosDataType::Texture
        );
    }
    fn ui(&self, _graph: &ProtosGraph, _node_id: NodeId, _ui: &mut egui::Ui) {
        
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
        let dimensions = self.evaluate_input(device, queue, graph, node_id, available_size, TextureResourceNodeInput::Dimensions.to_string(), outputs_cache)?.try_to_vec2()?;
        let mut texture = self.handle.lock().unwrap();
        texture.set_width(dimensions[0] as u32);
        texture.set_height(dimensions[1] as u32);
        texture.update_data(device, queue)?;
        self.populate_output(graph, node_id, TextureResourceNodeOutput::Texture.to_string(), ProtosValueType::Texture(Some(self.handle.clone())), outputs_cache);
        
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