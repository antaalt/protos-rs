use std::{sync::Arc, borrow::Cow, collections::HashMap};

use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeTemplateIter, NodeId, NodeTemplateTrait, Graph, UserResponseTrait, NodeDataTrait, NodeResponse, OutputId};

use crate::gfx;

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, ProtosNodeData, ProtosGraphState, ProtosResponse, node_buffer::BufferNode, node_texture_file::TextureFileNode, node_texture_resource::TextureResourceNode, node_graphic_pass::GraphicPassNode, node_backbuffer_pass::BackbufferPassNode};

// Trait could be applied directly to protosNodeTemplate ResourceHandle ?
pub trait ProtosNode {
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
}

#[derive(Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtosNodeTemplate {
    BackbufferPass { handle: gfx::ResourceHandle<gfx::BackbufferPass> },
    GraphicPass { handle: gfx::ResourceHandle<gfx::GraphicPass> }, 
    ComputePass { handle: gfx::ResourceHandle<gfx::ComputePass> }, 
    Buffer { handle: gfx::ResourceHandle<gfx::Buffer> }, 
    FileTexture { handle: gfx::ResourceHandle<gfx::Texture> },
    ResourceTexture { handle: gfx::ResourceHandle<gfx::Texture> },
    Camera { handle: gfx::ResourceHandle<gfx::Camera> }, 
    Mesh { handle: gfx::ResourceHandle<gfx::Mesh> }, 
}

impl ProtosNodeTemplate {
    pub fn can_be_recorded(&self) -> bool {
        match self {
            // Can
            ProtosNodeTemplate::BackbufferPass{ .. } => true,
            ProtosNodeTemplate::GraphicPass{ .. } => true,
            ProtosNodeTemplate::ComputePass{ .. } => true,
            // Cannot
            ProtosNodeTemplate::Buffer{ .. } => false,
            ProtosNodeTemplate::FileTexture{ .. } => false,
            ProtosNodeTemplate::ResourceTexture{ .. } => false,
            ProtosNodeTemplate::Camera{ .. } => false,
            ProtosNodeTemplate::Mesh{ .. } => false,
        }
    }
    pub fn get_name(&self) -> &str {
        match self {
            ProtosNodeTemplate::BackbufferPass{ .. } => "BackbufferPass",
            ProtosNodeTemplate::GraphicPass{ .. } => "GraphicPass",
            ProtosNodeTemplate::ComputePass{ .. } => "ComputePass",
            ProtosNodeTemplate::Buffer{ .. } => "Buffer",
            ProtosNodeTemplate::FileTexture{ .. } => "FileTexture",
            ProtosNodeTemplate::ResourceTexture{ .. } => "ResourceTexture",
            ProtosNodeTemplate::Camera{ .. } => "Camera",
            ProtosNodeTemplate::Mesh{ .. } => "Mesh",
         }
    }
    // might need to return a box instead...
    fn into_node(&self) -> Box<dyn ProtosNode> {
        match self {
            ProtosNodeTemplate::Buffer { handle } => { Box::new(BufferNode::new(handle.clone())) }
            ProtosNodeTemplate::FileTexture { handle } => { Box::new(TextureFileNode::new(handle.clone())) }
            ProtosNodeTemplate::ResourceTexture { handle } => { Box::new(TextureResourceNode::new(handle.clone())) }
            ProtosNodeTemplate::GraphicPass { handle } => { Box::new(GraphicPassNode::new(handle.clone())) }
            ProtosNodeTemplate::BackbufferPass { handle } => { Box::new(BackbufferPassNode::new(handle.clone())) }
            _ => unimplemented!("You need to register node here") // TODO: display
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
        Cow::Borrowed(self.get_name())
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
        let node = self.into_node();
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
            ProtosNodeTemplate::BackbufferPass { handle: Arc::default() },
            ProtosNodeTemplate::GraphicPass{ handle: Arc::default() },
            ProtosNodeTemplate::ComputePass{ handle: Arc::default() },
            ProtosNodeTemplate::Buffer{ handle: Arc::default() },
            ProtosNodeTemplate::FileTexture{ handle: Arc::default() },
            ProtosNodeTemplate::ResourceTexture{ handle: Arc::default() },
            ProtosNodeTemplate::Camera{ handle: Arc::default() },
            ProtosNodeTemplate::Mesh{ handle: Arc::default() },
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

pub type OutputsCache = HashMap<OutputId, ProtosValueType>;
// TODO: could be default trait
pub fn evaluate_node(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    graph: &ProtosGraph,
    node_id: NodeId,
    available_size: Vec2,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<()> {
    let node = graph[node_id].user_data.template.into_node();
    node.evaluate(device, queue, graph, node_id, available_size, outputs_cache)
}


// Evaluates the input value of
pub fn record_input(
    //&self,
    device: &wgpu::Device,
    cmd: &mut wgpu::CommandEncoder,
    graph: &ProtosGraph,
    node_id: NodeId,
    param_name: &str,
    outputs_cache: &mut OutputsCache,
) {
    let input_id = graph[node_id].get_input(param_name).unwrap();

    // The output of another node is connected.
    if let Some(other_output_id) = graph.connection(input_id) {
        // The value was already computed due to the evaluation of some other
        // node. We simply return value from the cache.
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            
        }
        // This is the first time encountering this node, so we need to
        // recursively evaluate it.
        else {
            // Calling this will populate the cache
            record_node(device, cmd, graph, graph[other_output_id].node, outputs_cache);
        }
    }
}

pub fn record_node(
    device: &wgpu::Device,
    cmd: &mut wgpu::CommandEncoder,
    graph: &ProtosGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) {
    let node = graph[node_id].user_data.template.into_node();
    // TODO: can be recorded ?
    node.record(device, cmd, graph, node_id, outputs_cache);
}

// Simply fill output with a value.
pub fn populate_output(
    //&self,
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
pub fn evaluate_input(
    //&self,
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
            match evaluate_node(device, queue, graph, graph[other_output_id].node, available_size, outputs_cache) {
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