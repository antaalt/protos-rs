use egui_node_graph::{Graph, GraphEditorState};

use super::{ProtosNodeTemplate, ProtosDataType, ProtosValueType, ProtosGraphState};


pub type ProtosGraph = Graph<ProtosNodeData, ProtosDataType, ProtosValueType>;
pub type ProtosEditorState = GraphEditorState<ProtosNodeData, ProtosDataType, ProtosValueType, ProtosNodeTemplate, ProtosGraphState>;

#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct ProtosNodeData {
    pub(crate) template: ProtosNodeTemplate,
}
