use std::borrow::Cow;

use egui::DragValue;
use egui_node_graph::{DataTypeTrait, NodeId, WidgetValueTrait};
use crate::gfx::{self, ResourceHandle};

use super::{ProtosResponse, ProtosGraphState, ProtosNodeData};

// TODO: could we work out with no DataType ? By using ValueType in place ?
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtosDataType {
    // gpu node
    Unknown,
    Texture,
    Buffer,
    Mesh,
    Shader,
    // constant node
    Scalar, // float
    Vec2,   // float2
    Vec3,   // float3
    String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtosValueType {
    Unknown,
    Texture(Option<ResourceHandle<gfx::Texture>>),
    Buffer(Option<ResourceHandle<gfx::Buffer>>),
    Mesh(Option<ResourceHandle<gfx::Mesh>>),
    Shader(Option<ResourceHandle<gfx::Shader>>),
    Scalar(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    String(String),
}

impl Default for ProtosValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Unknown {}
    }
}

impl DataTypeTrait<ProtosGraphState> for ProtosDataType {
    fn data_type_color(&self, _user_state: &mut ProtosGraphState) -> egui::Color32 {
        match self {
            ProtosDataType::Unknown => egui::Color32::from_rgb(255, 255, 255),
            ProtosDataType::Texture => egui::Color32::from_rgb(255, 0, 0),
            ProtosDataType::Mesh => egui::Color32::from_rgb(0, 127, 0),
            ProtosDataType::Buffer => egui::Color32::from_rgb(0, 255, 0),
            ProtosDataType::Shader => egui::Color32::from_rgb(127, 0, 0),
            ProtosDataType::Scalar => egui::Color32::from_rgb(0, 0, 255),
            ProtosDataType::Vec2 => egui::Color32::from_rgb(255, 255, 0),
            ProtosDataType::Vec3 => egui::Color32::from_rgb(0, 255, 255),
            ProtosDataType::String => egui::Color32::from_rgb(0, 0, 0),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            ProtosDataType::Unknown => Cow::Borrowed("unknown"),
            ProtosDataType::Texture => Cow::Borrowed("texture"),
            ProtosDataType::Buffer => Cow::Borrowed("buffer"),
            ProtosDataType::Mesh => Cow::Borrowed("mesh"),
            ProtosDataType::Shader => Cow::Borrowed("shader"),
            ProtosDataType::Scalar => Cow::Borrowed("scalar"),
            ProtosDataType::Vec2 => Cow::Borrowed("vec2"),
            ProtosDataType::Vec3 => Cow::Borrowed("vec3"),
            ProtosDataType::String => Cow::Borrowed("string"),
        }
    }
}

impl ProtosValueType {
    pub fn try_to_texture(self) -> anyhow::Result<Option<ResourceHandle<gfx::Texture>>> {
        if let ProtosValueType::Texture(value) = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast to texture")
        }
    }
    pub fn try_to_geometry(self) -> anyhow::Result<Option<ResourceHandle<gfx::Mesh>>> {
        if let ProtosValueType::Mesh(value) = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast to mesh")
        }
    }
    pub fn try_to_scalar(self) -> anyhow::Result<f32> {
        if let ProtosValueType::Scalar(value) = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast to scalar")
        }
    }
    pub fn try_to_vec2(self) -> anyhow::Result<[f32;2]> {
        if let ProtosValueType::Vec2(value) = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast to scalar")
        }
    }
    pub fn try_to_string(self) -> anyhow::Result<String> {
        if let ProtosValueType::String(value) = self {
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
            ProtosValueType::Texture(value) => {
                let _ = value;
                ui.label(param_name);
            }
            ProtosValueType::Buffer(value) => {
                let _ = value;
                ui.label(param_name);
            }
            ProtosValueType::Mesh(value) => {
                let _ = value;
                ui.label(param_name);
            }
            ProtosValueType::Shader(value)  => {
                let _ = value;
                ui.label(param_name);
            }
            ProtosValueType::Scalar(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            }
            ProtosValueType::Vec2(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(&mut value[0]));
                    ui.add(DragValue::new(&mut value[1]));
                });
            }
            ProtosValueType::Vec3(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(&mut value[0]));
                    ui.add(DragValue::new(&mut value[1]));
                    ui.add(DragValue::new(&mut value[2]));
                });
            }
            ProtosValueType::String(value)  => {
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