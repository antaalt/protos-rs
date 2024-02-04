use core::fmt;

use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use crate::{gfx, graph::{core::ProtosGraph, node::OutputsCache, ProtosDataType, ProtosNode, ProtosValueType}};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BufferNode {
    handle: gfx::ResourceHandle<gfx::Buffer>
}
pub enum BufferNodeInput {
    Size,
    Format,
}
pub enum BufferNodeOutput {
    Buffer,
}
impl fmt::Display for BufferNodeInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufferNodeInput::Size => write!(f, "Size"),
            BufferNodeInput::Format => write!(f, "Format"),
        }
    }
}
impl fmt::Display for BufferNodeOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufferNodeOutput::Buffer => write!(f, "Buffer"),
        }
    }
}

impl ProtosNode for BufferNode {
    fn get_name(&self) -> &str {
        "Buffer"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_input_param(
            node_id,
            BufferNodeInput::Size.to_string(),
            ProtosDataType::Scalar,
            ProtosValueType::Scalar(0.0),
            InputParamKind::ConstantOnly,
            true,
        );
        graph.add_input_param(
            node_id,
            BufferNodeInput::Format.to_string(),
            ProtosDataType::Scalar,
            ProtosValueType::Scalar(0.0),
            InputParamKind::ConstantOnly,
            true,
        );
        graph.add_output_param(
            node_id, 
            BufferNodeOutput::Buffer.to_string(),
            ProtosDataType::Buffer
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
        let size = self.evaluate_input(device, queue, graph, node_id, available_size, BufferNodeInput::Size.to_string(), outputs_cache)?.try_to_scalar()?;
        let format = self.evaluate_input(device, queue, graph, node_id, available_size, BufferNodeInput::Format.to_string(), outputs_cache)?.try_to_scalar()?;
        let mut buffer = self.handle.lock().unwrap();
        buffer.set_size(size as u32);
        buffer.set_format(format as u32);
        buffer.update_data(device, queue)?;
        self.populate_output(graph, node_id, BufferNodeOutput::Buffer.to_string(), ProtosValueType::Buffer(Some(self.handle.clone())), outputs_cache);
        
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