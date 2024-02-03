use egui::DragValue;
use egui_node_graph::{WidgetValueTrait, NodeId};
// TODO: Should be renamed connection.rs & merged with data_type.rs
use crate::gfx::{self, ResourceHandle};

use super::{ProtosResponse, ProtosGraphState, ProtosNodeData};


#[derive(Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtosValueType {
    Unknown {},
    Texture { value: Option<ResourceHandle<gfx::Texture>> },
    Buffer { value: Option<ResourceHandle<gfx::Buffer>> },
    Mesh { value: Option<ResourceHandle<gfx::Mesh>> },
    Shader { value: Option<ResourceHandle<gfx::Shader>>},
    Scalar { value: f32 },
    Vec2 { value: [f32; 2] },
    Vec3 { value: [f32; 3] },
    String { value: String },
}

impl Default for ProtosValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Unknown {}
    }
}

impl ProtosValueType {
    pub fn try_to_texture(self) -> anyhow::Result<Option<ResourceHandle<gfx::Texture>>> {
        if let ProtosValueType::Texture { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast to texture")
        }
    }
    pub fn try_to_scalar(self) -> anyhow::Result<f32> {
        if let ProtosValueType::Scalar { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast to scalar")
        }
    }
    pub fn try_to_vec2(self) -> anyhow::Result<[f32;2]> {
        if let ProtosValueType::Vec2 { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast to scalar")
        }
    }
    pub fn try_to_string(self) -> anyhow::Result<String> {
        if let ProtosValueType::String { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast to scalar")
        }
    }
}


impl WidgetValueTrait for ProtosValueType {
    type Response = ProtosResponse;
    type UserState = ProtosGraphState;
    type NodeData = ProtosNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut ProtosGraphState,
        _node_data: &ProtosNodeData,
    ) -> Vec<ProtosResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            ProtosValueType::Texture { value } => {
                let _ = value;
                ui.label(param_name);
            }
            ProtosValueType::Buffer { value } => {
                let _ = value;
                ui.label(param_name);
            }
            ProtosValueType::Mesh { value } => {
                let _ = value;
                ui.label(param_name);
            }
            ProtosValueType::Shader { value }  => {
                // TODO: syntax
                let _ = value;
                ui.label(param_name);
                //ui.add(egui::TextEdit::multiline(value));
            }
            ProtosValueType::Scalar { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            }
            ProtosValueType::Vec2 { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(&mut value[0]));
                    ui.add(DragValue::new(&mut value[1]));
                });
            }
            ProtosValueType::Vec3 { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(&mut value[0]));
                    ui.add(DragValue::new(&mut value[1]));
                    ui.add(DragValue::new(&mut value[2]));
                });
            }
            ProtosValueType::String { value }  => {
                ui.add(egui::TextEdit::singleline(value));
            }
            _  => {
                ui.label("Unknown");
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}