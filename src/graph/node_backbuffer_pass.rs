use std::rc::Rc;

use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, node::{ProtosNode, OutputsCache}};

use crate::gfx;

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BackbufferPassNode {
    pub handle: gfx::ResourceHandle<gfx::BackbufferPass>
}

impl ProtosNode for BackbufferPassNode {
    fn get_name(&self) -> &str {
        "Backbuffer pass"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_input_param(
            node_id,
            "input".to_string(),
            ProtosDataType::Texture,
            ProtosValueType::Texture { value: None },
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
        let input = self.evaluate_input(device, queue, graph, node_id, available_size, "input", outputs_cache)?;
        // Check input is valid type.
        let mut pass = self.handle.lock().unwrap();
        if let ProtosValueType::Texture { value } = input {
            pass.set_origin(value);
        } else {
            pass.clear_origin();
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
        let pass = self.handle.lock().unwrap();
        if pass.has_data() {
            self.record_input(device, cmd, graph, node_id, "input", outputs_cache);
            pass.record_data(device, cmd);
        }
        Ok(())
    }
}