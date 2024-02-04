use core::fmt;
use std::path::PathBuf;

use egui::Vec2;
use egui_node_graph::NodeId;

use crate::{gfx::{self, MeshShape, MeshSource}, graph::{core::ProtosGraph, node::OutputsCache, ProtosDataType, ProtosNode, ProtosValueType}};

#[derive(Default, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct MeshNode {
    mesh: gfx::ResourceHandle<gfx::Mesh>
}
pub enum MeshNodeOutput {
    Geometry,
}

impl fmt::Display for MeshNodeOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeshNodeOutput::Geometry => write!(f, "Geometry"),
        }
    }
}

impl ProtosNode for MeshNode {
    fn get_name(&self) -> &str {
        "Mesh"
    }
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId) {
        graph.add_output_param(node_id, MeshNodeOutput::Geometry.to_string(), ProtosDataType::Mesh);
    }
    fn ui(&self, _graph: &ProtosGraph, _node_id: NodeId, ui: &mut egui::Ui) {
        gfx::visit_resource_mut(&self.mesh, |mesh| {
            mesh.visit_desc_mut(|desc| {
                // Select shape
                let mut changed = false;
                egui::ComboBox::from_label("Source")
                    .selected_text(format!("{:?}", desc.source.get_source_name()))
                    .show_ui(ui, |ui| {
                        changed |= ui.selectable_value(&mut desc.source, MeshSource::Path(PathBuf::from("")), MeshSource::Path(PathBuf::from("")).get_source_name()).changed();
                        changed |= ui.selectable_value(&mut desc.source, MeshSource::Shape(MeshShape::Cube{ size:0.0 }), MeshSource::Shape(MeshShape::Cube{ size:0.0 }).get_source_name()).changed();
                    });
                match &mut desc.source {
                    MeshSource::Path(path) => {
                        let mut str = path.to_str().unwrap();
                        changed |= ui.text_edit_singleline(&mut str).changed();
                    },
                    MeshSource::Shape(shape) => {
                        let default_sphere = MeshShape::default_sphere();
                        let default_cube = MeshShape::default_cube();
                        egui::ComboBox::from_label("Shape")
                            .selected_text(format!("{:?}", shape.get_shape_name()))
                            .show_ui(ui, |ui| {
                                changed |= ui.selectable_value(shape, default_sphere, default_sphere.get_shape_name()).changed();
                                changed |= ui.selectable_value(shape, default_cube, default_cube.get_shape_name()).changed();
                            });
                        match shape {
                            MeshShape::Sphere{ ring_count, segment_count, radius } => {
                                changed |= ui.add(egui::Slider::new(ring_count, 4..=32)).changed();
                                changed |= ui.add(egui::Slider::new(segment_count, 2..=32)).changed();
                                changed |= ui.add(egui::Slider::new(radius, 0.0..=10.0)).changed();
                            },
                            MeshShape::Cube{ size } => {
                                changed |= ui.add(egui::Slider::new(size, 0.1..=10.0)).changed();
                            },
                        }
                    }
                    _ => {}
                }
                changed
            });
        });
    }
    fn evaluate(
        &self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &ProtosGraph,
        node_id: NodeId,
        _available_size: Vec2,
        outputs_cache: &mut OutputsCache
    ) -> anyhow::Result<()> {
        
        let mut mesh = self.mesh.lock().unwrap();
        mesh.update_data(device, queue)?;
        self.populate_output(graph, node_id, MeshNodeOutput::Geometry.to_string(), ProtosValueType::Mesh(Some(self.mesh.clone())), outputs_cache);
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