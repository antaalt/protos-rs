use std::{path::PathBuf, str::FromStr};

use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, node::{OutputsCache, ProtosNode}};

use crate::gfx;

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TextureFileNode {
    handle: gfx::ResourceHandle<gfx::Texture>
}

impl ProtosNode for TextureFileNode {
    fn get_name(&self) -> &str {
        "FileTexture"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_input_param(
            node_id.clone(),
            String::from("Path"),
            ProtosDataType::String,
            ProtosValueType::String { value: String::from("") },
            InputParamKind::ConstantOnly,
            true,
        );
        graph.add_output_param(
            node_id.clone(), 
            String::from("texture"),
            ProtosDataType::Texture
        );
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
        let path = self.evaluate_input(device, queue, graph, node_id, available_size, "Path", outputs_cache)?.try_to_string()?;
        let mut texture = self.handle.lock().unwrap();
        texture.set_path(PathBuf::from_str(path.as_str())?);
        texture.update_data(device, queue)?;
        self.populate_output(graph, node_id, "texture", ProtosValueType::Texture { value: Some(self.handle.clone()) }, outputs_cache);

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