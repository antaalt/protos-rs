use egui_node_graph::{NodeId, UserResponseTrait};

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtosResponse {
    SetCurrentBackbuffer(NodeId),
    ClearCurrentBackbuffer,
}

impl UserResponseTrait for ProtosResponse {}