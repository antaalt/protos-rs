use core::fmt;
use std::{path::PathBuf, str::FromStr};

use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use crate::{gfx, graph::{core::ProtosGraph, node::OutputsCache, ProtosDataType, ProtosNode, ProtosValueType}};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TextureFileNode {
    handle: gfx::ResourceHandle<gfx::Texture>
}

pub enum TextureFileNodeInput {
    Path,
}
pub enum TextureFileNodeOutput {
    Texture,
}
impl fmt::Display for TextureFileNodeInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextureFileNodeInput::Path => write!(f, "Path"),
        }
    }
}
impl fmt::Display for TextureFileNodeOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextureFileNodeOutput::Texture => write!(f, "texture"),
        }
    }
}

impl ProtosNode for TextureFileNode {
    fn get_name(&self) -> &str {
        "FileTexture"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_input_param(
            node_id.clone(),
            TextureFileNodeInput::Path.to_string(),
            ProtosDataType::String,
            ProtosValueType::String(String::from("")),
            InputParamKind::ConstantOnly,
            true,
        );
        graph.add_output_param(
            node_id.clone(), 
            TextureFileNodeOutput::Texture.to_string(),
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
        let path = self.evaluate_input(device, queue, graph, node_id, available_size, TextureFileNodeInput::Path.to_string(), outputs_cache)?.try_to_string()?;
        let mut texture = self.handle.lock().unwrap();
        texture.set_path(PathBuf::from_str(path.as_str())?);
        texture.update_data(device, queue)?;
        self.populate_output(graph, node_id, TextureFileNodeOutput::Texture.to_string(), ProtosValueType::Texture(Some(self.handle.clone())), outputs_cache);

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