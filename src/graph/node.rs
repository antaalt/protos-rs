use std::{sync::{Arc, Mutex}, borrow::Cow, collections::HashMap};

use egui::Vec2;
use egui_node_graph::{NodeTemplateIter, NodeId, NodeTemplateTrait, Graph, UserResponseTrait, NodeDataTrait, NodeResponse, OutputId};

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, ProtosNodeData, ProtosGraphState, ProtosResponse, node_backbuffer_pass::BackbufferPassNode, node_texture_file::TextureFileNode, node_texture_resource::TextureResourceNode, node_graphic_pass::GraphicPassNode, node_buffer::BufferNode, node_compute_pass::ComputePassNode, node_camera::CameraNode, node_mesh::MeshNode};

pub type OutputsCache = HashMap<OutputId, ProtosValueType>;

pub trait ProtosNode {
    // Get node name
    fn get_name(&self) -> &str;
    // Describe the node
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId);
    // Evaluate its input / output
    fn evaluate(&self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &ProtosGraph,
        node_id: NodeId,
        available_size: Vec2,
        outputs_cache: &mut OutputsCache) -> anyhow::Result<()>;
    // Record the node to command buffer
    fn record(&self,
        device: &wgpu::Device,
        cmd: &mut wgpu::CommandEncoder,
        graph: &ProtosGraph,
        node_id: NodeId,
        outputs_cache: &mut OutputsCache) -> anyhow::Result<()>;

    // Evaluates the input value of
    fn record_input(
        &self,
        device: &wgpu::Device,
        cmd: &mut wgpu::CommandEncoder,
        graph: &ProtosGraph,
        node_id: NodeId,
        param_name: &str,
        outputs_cache: &mut OutputsCache,
    ) -> anyhow::Result<()> {
        let input_id = graph[node_id].get_input(param_name).unwrap();

        // The output of another node is connected.
        if let Some(other_output_id) = graph.connection(input_id) {
            // The value was already computed due to the evaluation of some other
            // node. We simply return value from the cache.
            if let Some(other_value) = outputs_cache.get(&other_output_id) {
                let _ = other_value;
            } else {
                // First time in this node, recurse it.
                let input_node = graph[graph[other_output_id].node].user_data.template.get_node();
                return input_node.record(device, cmd, graph, graph[other_output_id].node, outputs_cache);
            }
        }
        Ok(())
    }

    // Simply fill output with a value.
    fn populate_output(
        &self,
        graph: &ProtosGraph,
        node_id: NodeId,
        param_name: &str,
        value: ProtosValueType,
        outputs_cache: &mut OutputsCache,
    ) {
        let output_id = graph[node_id].get_output(param_name).unwrap();
        outputs_cache.insert(output_id, value.clone());
    }

    // Evaluates the input value of
    fn evaluate_input(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &ProtosGraph,
        node_id: NodeId,
        available_size: Vec2,
        param_name: &str,
        outputs_cache: &mut OutputsCache,
    ) -> anyhow::Result<ProtosValueType> {
        let input_id = graph[node_id].get_input(param_name)?;

        // The output of another node is connected.
        if let Some(other_output_id) = graph.connection(input_id) {
            // The value was already computed due to the evaluation of some other
            // node. We simply return value from the cache.
            if let Some(other_value) = outputs_cache.get(&other_output_id) {
                Ok(other_value.clone())
            }
            // This is the first time encountering this node, so we need to
            // recursively evaluate it.
            else {
                let input_node = graph[graph[other_output_id].node].user_data.template.get_node();
                match input_node.evaluate(device, queue, graph, graph[other_output_id].node, available_size, outputs_cache) {
                    Ok(()) => {
                        Ok(outputs_cache
                        .get(&other_output_id)
                        .expect("Cache should be populated").clone())
                    }
                    Err(err) => anyhow::bail!("Node failed to compile : {}.", err.to_string())
                }
            }
        }
        // No existing connection, take the inline value instead.
        else {
            Ok(graph[input_id].value.clone())
        }
    }
}


// TODO:ProtosNode should be a node handle instead for simplification...
// with a trait. but every node need a custom impl...
//pub type NodeHandle<Type> = Arc<Mutex<Type>>;

#[derive(Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtosNodeTemplate {
    BackbufferPass (BackbufferPassNode),
    GraphicPass (GraphicPassNode), 
    ComputePass (ComputePassNode), 
    Buffer (BufferNode), 
    FileTexture (TextureFileNode),
    ResourceTexture (TextureResourceNode),
    Camera (CameraNode), 
    Mesh (MeshNode), 
}

impl ProtosNodeTemplate {
    pub fn get_node(&self) -> Box<dyn ProtosNode> {
        match self {
            ProtosNodeTemplate::BackbufferPass(handle) => { Box::new(handle.clone()) }
            ProtosNodeTemplate::GraphicPass(handle) => { Box::new(handle.clone()) }
            ProtosNodeTemplate::ComputePass(handle) => { Box::new(handle.clone()) }
            ProtosNodeTemplate::FileTexture(handle) => { Box::new(handle.clone()) }
            ProtosNodeTemplate::ResourceTexture(handle) => { Box::new(handle.clone()) }
            ProtosNodeTemplate::Buffer(handle) => { Box::new(handle.clone()) }
            ProtosNodeTemplate::Camera(handle) => { Box::new(handle.clone()) }
            ProtosNodeTemplate::Mesh(handle) => { Box::new(handle.clone()) }
            _ => { unimplemented!("Missing node implementation"); }
        }
    }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for ProtosNodeTemplate {
    type NodeData = ProtosNodeData;
    type DataType = ProtosDataType;
    type ValueType = ProtosValueType;
    type UserState = ProtosGraphState;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        let node = self.get_node();
        Cow::Owned(node.get_name().to_owned())
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        ProtosNodeData { template: self.clone() }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        println!("Building node : {}", self.get_node().get_name());
        let node = self.get_node();
        node.build(graph, node_id);
    }
}

pub struct AllProtosNodeTemplates;

impl NodeTemplateIter for AllProtosNodeTemplates {
    type Item = ProtosNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![
            ProtosNodeTemplate::BackbufferPass (BackbufferPassNode::default()),
            ProtosNodeTemplate::GraphicPass(GraphicPassNode::default()),
            ProtosNodeTemplate::ComputePass(ComputePassNode::default()),
            ProtosNodeTemplate::Buffer(BufferNode::default()),
            ProtosNodeTemplate::FileTexture(TextureFileNode::default()),
            ProtosNodeTemplate::ResourceTexture(TextureResourceNode::default()),
            ProtosNodeTemplate::Camera(CameraNode::default()),
            ProtosNodeTemplate::Mesh(MeshNode::default()),
        ]
    }
}


impl NodeDataTrait for ProtosNodeData {
    type Response = ProtosResponse;
    type UserState = ProtosGraphState;
    type DataType = ProtosDataType;
    type ValueType = ProtosValueType;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &ProtosGraph,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<ProtosResponse, ProtosNodeData>>
    where
        ProtosResponse: UserResponseTrait,
    {
        match &self.template {
            ProtosNodeTemplate::BackbufferPass { .. } => {
                // We only want bottom UI for backbuffer pass node.
            }
            _ => { 
                return vec![]; 
            }
        }
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        let is_active = user_state
            .backbuffer_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        if !is_active {
            if ui.button("👁 Set active").clicked() {
                responses.push(NodeResponse::User(ProtosResponse::SetCurrentBackbuffer(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(ProtosResponse::ClearCurrentBackbuffer));
            }
        }

        responses
    }
}