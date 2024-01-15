use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, node::{ProtosNode, evaluate_input, populate_output, OutputsCache}};

use crate::gfx;

pub struct TextureFileNode {
    handle: gfx::ResourceHandle<gfx::Texture>
}

impl TextureFileNode {
    pub fn new(handle: gfx::ResourceHandle<gfx::Texture>) -> Self {
        Self {
            handle
        }
    }
}

impl ProtosNode for TextureFileNode {
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
    fn evaluate(
        &self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &ProtosGraph,
        node_id: NodeId,
        available_size: Vec2,
        outputs_cache: &mut OutputsCache
    ) -> anyhow::Result<()> {
        let path = evaluate_input(device, queue, graph, node_id, available_size, "Path", outputs_cache)?.try_to_string()?;
        let mut texture = self.handle.lock().unwrap();
        //texture.set_path();
        texture.update_data(device, queue)?;
        populate_output(graph, node_id, "texture", ProtosValueType::Texture { value: Some(self.handle.clone()) }, outputs_cache);

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