use std::{borrow::Cow, collections::HashMap};

use egui::Vec2;
use egui_node_graph::{NodeTemplateIter, NodeId, NodeTemplateTrait, Graph, UserResponseTrait, NodeDataTrait, NodeResponse, OutputId};

use super::{core::{ProtosCategoryType, ProtosGraph}, nodes::{BackbufferPassNode, BufferNode, CameraNode, ComputePassNode, GraphicPassNode, MeshNode, ShaderNode, TextureFileNode, TextureResourceNode}, ProtosDataType, ProtosGraphState, ProtosNodeData, ProtosResponse, ProtosValueType};

pub type OutputsCache = HashMap<OutputId, ProtosValueType>;

pub trait ProtosNode {
    // Get node name
    fn get_name(&self) -> &str;
    // Describe the node
    fn build(&self, graph: &mut ProtosGraph, node_id: NodeId);
    // Describe the UI
    fn ui(&self, graph: &ProtosGraph, node_id: NodeId, ui: &mut egui::Ui);
    // Evaluate its input / output
    fn evaluate(&self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &ProtosGraph,
        node_id: NodeId, // TODO: store in data & remove.
        available_size: Vec2, // TODO: remove somehow
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
        input_name: String,
        outputs_cache: &mut OutputsCache,
    ) -> anyhow::Result<()> {
        let input_id = graph[node_id].get_input(input_name.to_string().as_str()).unwrap();

        // The output of another node is connected.
        if let Some(other_output_id) = graph.connection(input_id) {
            // The value was already computed due to the evaluation of some other
            // node. We simply return value from the cache.
            if let Some(other_value) = outputs_cache.get(&other_output_id) {
                let _ = other_value;
                Ok(())
            } else {
                // First time in this node, recurse it.
                graph[graph[other_output_id].node].user_data.template.visit_node(|input_node| {
                    input_node.record(device, cmd, graph, graph[other_output_id].node, outputs_cache)
                })
            }
        } else {
            Ok(())
        }
    }

    // Simply fill output with a value.
    fn populate_output(
        &self,
        graph: &ProtosGraph,
        node_id: NodeId,
        param_name: String,
        value: ProtosValueType,
        outputs_cache: &mut OutputsCache,
    ) {
        let output_id = graph[node_id].get_output(param_name.as_str()).unwrap();
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
        param_name: String,
        outputs_cache: &mut OutputsCache,
    ) -> anyhow::Result<ProtosValueType> {
        let input_id = graph[node_id].get_input(param_name.as_str())?;

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
                graph[graph[other_output_id].node].user_data.template.visit_node(|input_node| {
                    match input_node.evaluate(device, queue, graph, graph[other_output_id].node, available_size, outputs_cache) {
                        Ok(()) => {
                            Ok(outputs_cache
                            .get(&other_output_id)
                            .expect("Cache should be populated").clone())
                        }
                        Err(err) => anyhow::bail!("Node failed to compile : {}.", err.to_string())
                    }
                })
            }
        }
        // No existing connection, take the inline value instead.
        else {
            Ok(graph[input_id].value.clone())
        }
    }
}

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
    Shader(ShaderNode),
}

impl ProtosNodeTemplate {
    pub fn visit_node<T>(&self, f : impl FnOnce(&dyn ProtosNode) -> anyhow::Result<T>) -> anyhow::Result<T> {
        match self {
            ProtosNodeTemplate::BackbufferPass(handle) => { f(handle) }
            ProtosNodeTemplate::GraphicPass(handle) => { f(handle) }
            ProtosNodeTemplate::ComputePass(handle) => { f(handle) }
            ProtosNodeTemplate::FileTexture(handle) => { f(handle) }
            ProtosNodeTemplate::ResourceTexture(handle) => { f(handle) }
            ProtosNodeTemplate::Buffer(handle) => { f(handle) }
            ProtosNodeTemplate::Camera(handle) => { f(handle) }
            ProtosNodeTemplate::Mesh(handle) => { f(handle) }
            ProtosNodeTemplate::Shader(handle) => { f(handle) }
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
    type CategoryType = ProtosCategoryType;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        match self.visit_node(|node| {
            Ok(node.get_name().to_owned())
        }) {
            Ok(name) => Cow::Owned(name),
            Err(err) => unreachable!("{}", err.to_string())
        }
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
        self.visit_node(|node| {
            println!("Building node : {}", node.get_name());
            node.build(graph, node_id);
            Ok(())
        }).expect("Should not fail.");
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
            ProtosNodeTemplate::BackbufferPass(BackbufferPassNode::default()),
            ProtosNodeTemplate::GraphicPass(GraphicPassNode::default()),
            ProtosNodeTemplate::ComputePass(ComputePassNode::default()),
            ProtosNodeTemplate::Buffer(BufferNode::default()),
            ProtosNodeTemplate::FileTexture(TextureFileNode::default()),
            ProtosNodeTemplate::ResourceTexture(TextureResourceNode::default()),
            ProtosNodeTemplate::Camera(CameraNode::default()),
            ProtosNodeTemplate::Mesh(MeshNode::default()),
            ProtosNodeTemplate::Shader(ShaderNode::default()),
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
        graph: &ProtosGraph,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<ProtosResponse, ProtosNodeData>>
    where
        ProtosResponse: UserResponseTrait,
    {
        match &self.template {
            ProtosNodeTemplate::BackbufferPass(node) => {
                // We only want bottom UI for backbuffer pass node.
                node.ui(graph, node_id, ui);

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
            _ => { 
                self.template.visit_node(|node| {
                    node.ui(graph, node_id, ui);
                    Ok(())
                }).expect("Should not fail.");
                return vec![]; 
            }
        }
    }
}