use std::borrow::Cow;

use egui_node_graph::DataTypeTrait;

use super::ProtosGraphState;


/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtosDataType {
    // gpu node
    Unknown,
    Texture,
    Buffer,
    // constant node
    Scalar, // float
    Vec2,   // float2
    Vec3,   // float3
    String,
}

// =========== Then, you need to implement some traits ============

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<ProtosGraphState> for ProtosDataType {
    fn data_type_color(&self, _user_state: &mut ProtosGraphState) -> egui::Color32 {
        match self {
            ProtosDataType::Unknown => egui::Color32::from_rgb(255, 255, 255),
            ProtosDataType::Texture => egui::Color32::from_rgb(255, 0, 0),
            ProtosDataType::Buffer => egui::Color32::from_rgb(0, 255, 0),
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
            ProtosDataType::Scalar => Cow::Borrowed("scalar"),
            ProtosDataType::Vec2 => Cow::Borrowed("vec2"),
            ProtosDataType::Vec3 => Cow::Borrowed("vec3"),
            ProtosDataType::String => Cow::Borrowed("string"),
        }
    }
}