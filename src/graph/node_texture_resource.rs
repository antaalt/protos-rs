use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, node::{ProtosNode, evaluate_input, populate_output, OutputsCache}};

use crate::gfx;

#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct TextureResourceNode {
    handle: gfx::Texture
}

impl TextureResourceNode {
    pub fn new(handle: gfx::Texture) -> Self {
        Self {
            handle
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
            String::from("Dimensions"),
            ProtosDataType::Vec2,
            ProtosValueType::Vec2 { value: [100.0, 100.0] },
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
        let dimensions = evaluate_input(device, queue, graph, node_id, available_size, "Dimensions", outputs_cache)?.try_to_vec2()?;
        let mut texture = self.handle;
        texture.set_width(dimensions[0] as u32);
        texture.set_height(dimensions[1] as u32);
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