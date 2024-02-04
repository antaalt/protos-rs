use core::fmt;

use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeId};

use crate::{gfx, graph::{core::ProtosGraph, node::OutputsCache, ProtosDataType, ProtosNode, ProtosValueType}};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct GraphicPassNode {
    handle: gfx::ResourceHandle<gfx::GraphicPass>,
}

pub enum GraphicPassNodeInput {
    ShaderResourceView(u32),
    VertexShader,
    FragmentShader,
    Geometry,
}
pub enum GraphicPassNodeOutput {
    RenderTarget(u32),
}
impl fmt::Display for GraphicPassNodeInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicPassNodeInput::ShaderResourceView(index) => write!(f, "SRV{}", index),
            GraphicPassNodeInput::VertexShader => write!(f, "VertexShader"),
            GraphicPassNodeInput::FragmentShader => write!(f, "FragmentShader"),
            GraphicPassNodeInput::Geometry => write!(f, "Geometry"),
        }
    }
}
impl fmt::Display for GraphicPassNodeOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicPassNodeOutput::RenderTarget(index) => write!(f, "RT{}", index),
        }
    }
}

impl ProtosNode for GraphicPassNode {
    fn get_name(&self) -> &str {
        "Graphic pass"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        // TODO: +/- button
        for i in 0..1 {
            graph.add_input_param(
                node_id,
                GraphicPassNodeInput::ShaderResourceView(i).to_string(),
                ProtosDataType::Texture,
                ProtosValueType::Texture(None),
                InputParamKind::ConnectionOnly,
                true,
            );
        }
        graph.add_input_param(
            node_id,
            GraphicPassNodeInput::VertexShader.to_string(),
            ProtosDataType::Shader,
            ProtosValueType::Shader(None),
            InputParamKind::ConnectionOnly,
            true,
        );
        graph.add_input_param(
            node_id,
            GraphicPassNodeInput::FragmentShader.to_string(),
            ProtosDataType::Shader,
            ProtosValueType::Shader(None),
            InputParamKind::ConnectionOnly,
            true,
        );
        graph.add_input_param(
            node_id,
            GraphicPassNodeInput::Geometry.to_string(),
            ProtosDataType::Mesh,
            ProtosValueType::Mesh(None),
            InputParamKind::ConnectionOnly,
            true,
        );
        for i in 0..1 {
            graph.add_output_param(node_id, GraphicPassNodeOutput::RenderTarget(i).to_string(), ProtosDataType::Texture);
        }
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
        // Here we should call all input_xxx, which will update the description of the graphic pass.
        // We can then update or not the pipeline with given data.
        // For recording command list, we will need something similar just running for graphic pass & compute pass.

        let mut pass = self.handle.lock().unwrap();

        let geometry = self.evaluate_input(device, queue, graph, node_id, available_size, GraphicPassNodeInput::Geometry.to_string(), outputs_cache)?.try_to_geometry()?;
        if let Some(geo) = geometry {
            pass.set_geometry(geo);
        } else {
            anyhow::bail!("Invalid geometry input")
        }

        for i in 0..1 {
            let srv = self.evaluate_input(device, queue, graph, node_id, available_size, GraphicPassNodeInput::ShaderResourceView(i).to_string(), outputs_cache)?.try_to_texture()?;
            pass.set_shader_resource_view(i, srv);
        }
        let num_attachment = 1;
        for i in 0..num_attachment {
            // Should gather these informations from a evaluate_output. -> reach output node, read its data & select informations.
            let mut desc = gfx::AttachmentDescription::default();
            desc.set_size(available_size.x as u32, available_size.y as u32);
            pass.set_render_target(i, &desc);
        }
        
        // Will call create if not created already.
        pass.update_data(device, queue)?;
        
        for i in 0..num_attachment {
            // Output graphic pass will populate output. need to ensure data is created already.
            self.populate_output(graph, node_id, GraphicPassNodeOutput::RenderTarget(i).to_string(), ProtosValueType::Texture(pass.get_render_target(i)), outputs_cache);
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
        for i in 0..1 {
            self.record_input(device, cmd, graph, node_id, GraphicPassNodeInput::ShaderResourceView(i).to_string(), outputs_cache)?;
        }
        let pass = self.handle.lock().unwrap();
        pass.record_data(device, cmd)
    }
}