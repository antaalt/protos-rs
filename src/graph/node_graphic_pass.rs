use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, node::{ProtosNode, OutputsCache}};

use crate::gfx;

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct GraphicPassNode {
    handle: gfx::ResourceHandle<gfx::GraphicPass>
}

impl ProtosNode for GraphicPassNode {
    fn get_name(&self) -> &str {
        "Graphic pass"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        
        // TODO for loop
        graph.add_input_param(
            node_id,
            "SRV0".to_string(),
            ProtosDataType::Texture,
            ProtosValueType::Texture { value: None },
            InputParamKind::ConnectionOnly,
            true,
        );
        // TODO for loop
        graph.add_output_param(node_id, "RT0".to_string(), ProtosDataType::Texture);
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
        // Here we should call all input_xxx, which will update the description of the graphic pass.
        // We can then update or not the pipeline with given data.
        // For recording command list, we will need something similar just running for graphic pass & compute pass.

        let mut pass = self.handle.lock().unwrap();
        // Input & co should be templated somewhere, trait to get these informations ?
        for i in 0..1 {
            //let name = format!("SRV{}", i)
            let srv = self.evaluate_input(device, queue, graph, node_id, available_size, "SRV0", outputs_cache)?;
            // Check input is valid type.
            if let ProtosValueType::Texture { value } = srv {
                pass.set_shader_resource_view(i, value);
            } else {
                anyhow::bail!("Invalid type is not a texture")
            }
        }
        let num_attachment = 1;
        for i in 0..num_attachment {
            // Should gather these informations from a evaluate_output. -> reach output node, read its data & select informations.
            let mut desc = gfx::AttachmentDescription::default();
            desc.set_size(available_size.x as u32, available_size.y as u32);
            pass.set_render_target(0, &desc);
        }
        
        // Will call create if not created already.
        pass.update_data(device, queue)?;
        
        for i in 0..num_attachment {
            // Output graphic pass will populate output. need to ensure data is created already.
            // TODO: custom name per output. (MRT support)
            self.populate_output(graph, node_id, "RT0", ProtosValueType::Texture { value: pass.get_render_target(i) }, outputs_cache);
        }
        
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
            // TODO: for loop
            self.record_input(device, cmd, graph, node_id, "SRV0", outputs_cache)?;
            pass.record_data(device, cmd);
            Ok(()) // TODO: propagate
        } else {
            Ok(())
        }
    }
}