mod core;
mod data_type;
mod node;
mod value;
mod graph;
mod response;

pub use self::data_type::ProtosDataType;

pub use self::node::ProtosNodeTemplate;
pub use self::node::AllProtosNodeTemplates;
pub use self::node::record_node;
pub use self::node::evaluate_node;

pub use self::core::ProtosNodeData;
pub use self::core::ProtosEditorState;

pub use self::value::ProtosValueType;

pub use self::graph::ProtosGraphState;

pub use self::response::ProtosResponse;

// we have :
// - data type which described connections
// - value type which hold content
// - template type -> Node