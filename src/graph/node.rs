use std::{sync::Arc, borrow::Cow, collections::HashMap};

use egui::Vec2;
use egui_node_graph::{InputParamKind, NodeTemplateIter, NodeId, NodeTemplateTrait, Graph, UserResponseTrait, NodeDataTrait, NodeResponse, OutputId};

use crate::gfx;

use super::{ProtosDataType, ProtosValueType, core::ProtosGraph, ProtosNodeData, ProtosGraphState, ProtosResponse};

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

    pub fn build_node(&self, graph: &mut ProtosGraph, node_id: &NodeId) {
        // entirely optional.
        let input_texture = |graph: &mut ProtosGraph, name: &str| {
            graph.add_input_param(
                node_id.clone(),
                name.to_string(),
                ProtosDataType::Texture,
                ProtosValueType::Texture { value: None },
                InputParamKind::ConnectionOnly,
                true,
            );
        };
        let input_image = |graph: &mut ProtosGraph, name: &str| {
            graph.add_input_param(
                node_id.clone(),
                name.to_string(),
                ProtosDataType::Texture,
                ProtosValueType::Texture { value: None },
                InputParamKind::ConnectionOnly,
                true,
            );
        };

        let output_texture = |graph: &mut ProtosGraph, name: &str| {
            graph.add_output_param(node_id.clone(), name.to_string(), ProtosDataType::Texture);
        };

        // This need to match evaluate_node
        match self {
            ProtosNodeTemplate::BackbufferPass{ handle: _ } => {
                input_texture(graph, "input".into());
            }
            ProtosNodeTemplate::GraphicPass{ handle: _ } => {
                // TODO for loop
                input_texture(graph, "SRV0".into());
                // TODO for loop
                output_texture(graph, "RT0".into());
            }
            ProtosNodeTemplate::ComputePass{ handle: _ } => {
                // TODO for loop
                input_image(graph, "UAV0".into());
                output_texture(graph, "UAV0".into());
            }
            ProtosNodeTemplate::Buffer{ handle: _ } => {
                graph.add_input_param(
                    node_id.clone(),
                    String::from("Size"),
                    ProtosDataType::Scalar,
                    ProtosValueType::Scalar { value: 0.0 },
                    InputParamKind::ConstantOnly,
                    true,
                );
                graph.add_input_param(
                    node_id.clone(),
                    String::from("Format"),
                    ProtosDataType::Scalar,
                    ProtosValueType::Scalar { value: 0.0 },
                    InputParamKind::ConstantOnly,
                    true,
                );
                graph.add_output_param(
                    node_id.clone(), 
                    String::from("Buffer"),
                    ProtosDataType::Buffer
                );
            }
            ProtosNodeTemplate::ResourceTexture{ handle: _ } => {
                graph.add_input_param(
                    node_id.clone(),
                    String::from("Dimensions"),
                    ProtosDataType::Vec2,
                    ProtosValueType::Vec2 { value: [100.0, 100.0] },
                    InputParamKind::ConstantOnly,
                    true,
                );
                graph.add_output_param(
                    node_id.clone(), 
                    String::from("texture"),
                    ProtosDataType::Texture
                );
            }
            ProtosNodeTemplate::FileTexture{ handle: _ } => {
                graph.add_input_param(
                    node_id.clone(),
                    String::from("Path"),
                    ProtosDataType::String,
                    ProtosValueType::String { value: String::from("") },
                    InputParamKind::ConstantOnly,
                    true,
                );
                graph.add_output_param(
                    node_id.clone(), 
                    String::from("texture"),
                    ProtosDataType::Texture
                );
            }
            ProtosNodeTemplate::Camera{ handle: _ } => {
                input_image(graph, "v1");
                input_image(graph, "v2");
                output_texture(graph, "out");
            }
            ProtosNodeTemplate::Mesh{ handle: _ } => {
                input_texture(graph, "x");
                input_texture(graph, "y");
                output_texture(graph, "out");
            }
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
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        // We define some closures here to avoid boilerplate. Note that this is
        // entirely optional.
        let input_texture = |graph: &mut ProtosGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                ProtosDataType::Texture,
                ProtosValueType::Texture { value: None },
                InputParamKind::ConnectionOnly,
                true,
            );
        };
        let input_image = |graph: &mut ProtosGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                ProtosDataType::Texture,
                ProtosValueType::Texture { value: None },
                InputParamKind::ConnectionOnly,
                true,
            );
        };

        let output_texture = |graph: &mut ProtosGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), ProtosDataType::Texture);
        };

        // This need to match evaluate_node
        match self {
            ProtosNodeTemplate::BackbufferPass{ handle: _ } => {
                input_texture(graph, "input".into());
            }
            ProtosNodeTemplate::GraphicPass{ handle: _ } => {
                // TODO for loop
                input_texture(graph, "SRV0".into());
                // TODO for loop
                output_texture(graph, "RT0".into());
            }
            ProtosNodeTemplate::ComputePass{ handle: _ } => {
                // TODO for loop
                input_image(graph, "UAV0".into());
                output_texture(graph, "UAV0".into());
            }
            ProtosNodeTemplate::Buffer{ handle: _ } => {
                graph.add_input_param(
                    node_id,
                    String::from("Size"),
                    ProtosDataType::Scalar,
                    ProtosValueType::Scalar { value: 0.0 },
                    InputParamKind::ConstantOnly,
                    true,
                );
                graph.add_input_param(
                    node_id,
                    String::from("Format"),
                    ProtosDataType::Scalar,
                    ProtosValueType::Scalar { value: 0.0 },
                    InputParamKind::ConstantOnly,
                    true,
                );
                graph.add_output_param(
                    node_id, 
                    String::from("Buffer"),
                    ProtosDataType::Buffer
                );
            }
            ProtosNodeTemplate::ResourceTexture{ handle: _ } => {
                graph.add_input_param(
                    node_id,
                    String::from("Dimensions"),
                    ProtosDataType::Vec2,
                    ProtosValueType::Vec2 { value: [100.0, 100.0] },
                    InputParamKind::ConstantOnly,
                    true,
                );
                graph.add_output_param(
                    node_id, 
                    String::from("texture"),
                    ProtosDataType::Texture
                );
            }
            ProtosNodeTemplate::FileTexture{ handle: _ } => {
                graph.add_input_param(
                    node_id,
                    String::from("Path"),
                    ProtosDataType::String,
                    ProtosValueType::String { value: String::from("") },
                    InputParamKind::ConstantOnly,
                    true,
                );
                graph.add_output_param(
                    node_id, 
                    String::from("texture"),
                    ProtosDataType::Texture
                );
            }
            ProtosNodeTemplate::Camera{ handle: _ } => {
                input_image(graph, "v1");
                input_image(graph, "v2");
                output_texture(graph, "out");
            }
            ProtosNodeTemplate::Mesh{ handle: _ } => {
                input_texture(graph, "x");
                input_texture(graph, "y");
                output_texture(graph, "out");
            }
        }
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
        /*match &self.template {
            ProtosNodeTemplate::BackbufferPass { .. } => {
                // We only want bottom UI for backbuffer pass node.
            }
            _ => { 
                return vec![]; 
            }
        }*/
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

type OutputsCache = HashMap<OutputId, ProtosValueType>;

//impl ProtosNodeTemplate {
    /// Recursively evaluates all dependencies of this node, then evaluates the node itself.
    /// Should create all resources here & record command list.
    /// Here we return a single output. but node could have multiple outputs...
    pub fn evaluate_node(
        //&self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &ProtosGraph,
        node_id: NodeId,
        available_size: Vec2,
        outputs_cache: &mut OutputsCache,
    ) -> anyhow::Result<()> {
        // This will create data. 
        let node = &graph[node_id];
        match &node.user_data.template {
            ProtosNodeTemplate::BackbufferPass{ handle } => {
                let mut pass = handle.lock().unwrap();
                let input = evaluate_input(device, queue, graph, node_id, available_size, "input", outputs_cache)?;
                // Check input is valid type.
                if let ProtosValueType::Texture { value } = input {
                    pass.set_origin(value);
                } else {
                    pass.clear_origin();
                }
                pass.set_size(available_size.x as u32, available_size.y as u32);
                // Will call create if not created already.
                pass.update_data(device, queue);

                Ok(())
            },
            ProtosNodeTemplate::GraphicPass{ handle } => {
                // Here we should call all input_xxx, which will update the description of the graphic pass.
                // We can then update or not the pipeline with given data.
                // For recording command list, we will need something similar just running for graphic pass & compute pass.

                let mut pass = handle.lock().unwrap();
                // Input & co should be templated somewhere, trait to get these informations ?
                for i in 0..1 {
                    //let name = format!("SRV{}", i)
                    let srv = evaluate_input(device, queue, graph, node_id, available_size, "SRV0", outputs_cache)?;
                    // Check input is valid type.
                    if let ProtosValueType::Texture { value } = srv {
                        pass.set_shader_resource_view(i, value);
                    } else {
                        anyhow::bail!("Invalid type is not a texture")
                    }
                }
                let num_attachment = 1;
                for i in 0..num_attachment {
                    // Should gather these informations from a evaluate_output. -> reach output node, read its data & select informations.
                    let mut desc = gfx::AttachmentDescription::default();
                    desc.set_size(available_size.x as u32, available_size.y as u32);
                    pass.set_render_target(0, &desc);
                }
                
                // Will call create if not created already.
                pass.update_data(device, queue);
                
                for i in 0..num_attachment {
                    // Output graphic pass will populate output. need to ensure data is created already.
                    // TODO: custom name per output. (MRT support)
                    populate_output(graph, node_id, "RT0", ProtosValueType::Texture { value: pass.get_render_target(i) }, outputs_cache);
                }
                
                Ok(())
            }
            ProtosNodeTemplate::ComputePass{ handle: _ } => {
                let a = evaluate_input(device, queue, graph, node_id, available_size, "A", outputs_cache);
                populate_output(graph, node_id, "out", ProtosValueType::Texture { value: a.unwrap().try_to_texture().unwrap() }, outputs_cache);
                
                Ok(())
            }
            ProtosNodeTemplate::Buffer{ handle } => {
                // Inputs for buffer are mostly data... scalar & co
                // Could have buffer type aswell (camera, custom (script based ?), common data (time & res)...)
                // Should be custom nodes instead for cleaner UX (only buffer behind the scene).
                let size = evaluate_input(device, queue, graph, node_id, available_size, "Size", outputs_cache).unwrap().try_to_scalar();
                let format = evaluate_input(device, queue, graph, node_id, available_size, "Format", outputs_cache).unwrap().try_to_vec2();
                let mut buffer = handle.lock().unwrap();
                buffer.update_data(device);
                populate_output(graph, node_id, "buffer", ProtosValueType::Buffer { value: Some(handle.clone()) }, outputs_cache);
                
                Ok(())
            }
            ProtosNodeTemplate::FileTexture{ handle } => {
                let path = evaluate_input(device, queue, graph, node_id, available_size, "Path", outputs_cache)?.try_to_string()?;
                let mut texture = handle.lock().unwrap();
                //texture.set_path();
                texture.update_data(device, queue)?;
                populate_output(graph, node_id, "texture", ProtosValueType::Texture { value: Some(handle.clone()) }, outputs_cache);

                Ok(())
            }
            ProtosNodeTemplate::ResourceTexture{ handle } => {
                let dimensions = evaluate_input(device, queue, graph, node_id, available_size, "Dimensions", outputs_cache)?.try_to_vec2()?;
                let mut texture = handle.lock().unwrap();
                texture.set_width(dimensions[0] as u32);
                texture.set_height(dimensions[1] as u32);
                texture.update_data(device, queue)?;
                populate_output(graph, node_id, "texture", ProtosValueType::Texture { value: Some(handle.clone()) }, outputs_cache);
                
                Ok(())
            }
            /*ProtosNodeTemplate::Camera{ handle: _ } => {
                let scalar = evaluate_input(graph, node_id, "v1", outputs_cache);
                let vector = evaluate_input(graph, node_id, "v2", outputs_cache);
                populate_output(graph, node_id, "out", ProtosValueType::Texture { value: scalar.unwrap().try_to_texture().unwrap() }, outputs_cache);
            }
            ProtosNodeTemplate::Mesh{ handle: _ } => {
                let scalar = evaluate_input(graph, node_id, "x", outputs_cache);
                let vector = evaluate_input(graph, node_id, "y", outputs_cache);
                populate_output(grap, node_id, "out", ProtosValueType::Texture { value: scalar.unwrap().try_to_texture().unwrap() }, outputs_cache);
            }*/
            _ => unimplemented!("Missing template implementation")
        }
    }


    // Evaluates the input value of
    fn record_input(
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
        // This will create data. 
        let node = &graph[node_id];
        match &node.user_data.template {
            ProtosNodeTemplate::BackbufferPass{ handle } => {
                let pass = handle.lock().unwrap();
                if pass.has_data() {
                    record_input(device, cmd, graph, node_id, "input", outputs_cache);
                    pass.record_data(device, cmd);
                }
            },
            ProtosNodeTemplate::GraphicPass{ handle } => {
                let pass = handle.lock().unwrap();
                if pass.has_data() {
                    // TODO: for loop
                    record_input(device, cmd, graph, node_id, "SRV0", outputs_cache);
                    pass.record_data(device, cmd);
                }
            },
            ProtosNodeTemplate::ComputePass{ handle } => {
                let pass = handle.lock().unwrap();
                if pass.has_data() {
                    pass.record_data(device, cmd);
                }
            },
            _ => { assert!(!node.user_data.template.can_be_recorded()) }
        }
    }

    // Simply fill output with a value.
    fn populate_output(
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
    fn evaluate_input(
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
//}