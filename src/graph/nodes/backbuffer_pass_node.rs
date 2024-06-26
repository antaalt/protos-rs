use core::fmt;

use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use crate::{gfx, graph::{core::ProtosGraph, node::OutputsCache, ProtosDataType, ProtosNode, ProtosValueType}};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BackbufferPassNode {
    pub handle: gfx::ResourceHandle<gfx::BackbufferPass>
}

pub enum BackbufferPassNodeInput {
    Input,
}
impl fmt::Display for BackbufferPassNodeInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackbufferPassNodeInput::Input => write!(f, "input"),
        }
    }
}

impl ProtosNode for BackbufferPassNode {
    fn get_name(&self) -> &str {
        "Backbuffer pass"
    }
    fn ui(&self, _graph: &ProtosGraph, _node_id: NodeId, _ui: &mut egui::Ui) {
        
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_input_param(
            node_id,
            "input".to_string(),
            ProtosDataType::Texture,
            ProtosValueType::Texture(None),
            InputParamKind::ConnectionOnly,
            true,
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
        let input = self.evaluate_input(device, queue, graph, node_id, available_size, BackbufferPassNodeInput::Input.to_string(), outputs_cache)?.try_to_texture()?;
        // Check input is valid type.
        let mut pass = self.handle.lock().unwrap();
        if let Some(value) = input {
            pass.set_origin(value);
        } else {
            anyhow::bail!("No input set.")
        }
        pass.set_size(available_size.x as u32, available_size.y as u32);
        // Will call create if not created already.
        pass.update_data(device, queue)?;

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
        // TODO should store inputs & set them automatically (we only need to define it at startup.)
        self.record_input(device, cmd, graph, node_id, BackbufferPassNodeInput::Input.to_string(), outputs_cache)?;
        let pass = self.handle.lock().unwrap();
        pass.record_data(device, cmd)
    }
}