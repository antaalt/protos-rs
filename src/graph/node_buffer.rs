use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, node::{ProtosNode, evaluate_input, OutputsCache, populate_output}};

use crate::gfx;

// TODO: template node ? at least content ?
pub struct BufferNode {
    handle: gfx::ResourceHandle<gfx::Buffer>
}

impl BufferNode {
    pub fn new(handle: gfx::ResourceHandle<gfx::Buffer>) -> Self {
        Self {
            handle
        }
    }
}

impl ProtosNode for BufferNode {
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
    fn evaluate(
        &self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &ProtosGraph,
        node_id: NodeId,
        available_size: Vec2,
        outputs_cache: &mut OutputsCache
    ) -> anyhow::Result<()> {
        let size = evaluate_input(device, queue, graph, node_id, available_size, "Size", outputs_cache).unwrap().try_to_scalar();
        let format = evaluate_input(device, queue, graph, node_id, available_size, "Format", outputs_cache).unwrap().try_to_vec2();
        let mut buffer = self.handle.lock().unwrap();
        buffer.update_data(device);
        populate_output(graph, node_id, "buffer", ProtosValueType::Buffer { value: Some(self.handle.clone()) }, outputs_cache);
        
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