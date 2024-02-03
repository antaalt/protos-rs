use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, node::{ProtosNode, OutputsCache}};

use crate::gfx;

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BufferNode {
    handle: gfx::ResourceHandle<gfx::Buffer>
}

impl ProtosNode for BufferNode {
    fn get_name(&self) -> &str {
        "Buffer"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_input_param(
            node_id,
            String::from("Size"),
            ProtosDataType::Scalar,
            ProtosValueType::Scalar { value: 0.0 },
            InputParamKind::ConstantOnly,
            true,
        );
        graph.add_input_param(
            node_id,
            String::from("Format"),
            ProtosDataType::Scalar,
            ProtosValueType::Scalar { value: 0.0 },
            InputParamKind::ConstantOnly,
            true,
        );
        graph.add_output_param(
            node_id, 
            String::from("Buffer"),
            ProtosDataType::Buffer
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
        let size = self.evaluate_input(device, queue, graph, node_id, available_size, "Size", outputs_cache)?.try_to_scalar()?;
        let format = self.evaluate_input(device, queue, graph, node_id, available_size, "Format", outputs_cache)?.try_to_scalar()?;
        let mut buffer = self.handle.lock().unwrap();
        buffer.set_size(size as u32);
        buffer.set_format(format as u32);
        buffer.update_data(device, queue)?;
        self.populate_output(graph, node_id, "buffer", ProtosValueType::Buffer { value: Some(self.handle.clone()) }, outputs_cache);
        
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