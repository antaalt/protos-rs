
use std::{borrow::Cow, collections::HashMap, sync::{Arc, Mutex}};

use egui::{self, DragValue, Vec2, TextStyle};
use egui_node_graph::*;

use crate::gfx;


// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct ProtosNodeData {
    template: ProtosNodeTemplate,
}

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


/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Scalar and a ValueType of Vec2.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtosValueType {
    Unknown {},
    Texture { value: Option<Arc<Mutex<gfx::Texture>>> },
    Buffer { value: Option<Arc<Mutex<gfx::Buffer>>> },
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
    pub fn try_to_texture(self) -> anyhow::Result<Option<Arc<Mutex<gfx::Texture>>>> {
        if let ProtosValueType::Texture { value } = self {
            Ok(value.clone())
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

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtosNodeTemplate {
    BackbufferPass { handle: Arc<Mutex<gfx::BackbufferPass>> },
    GraphicPass { handle: Arc<Mutex<gfx::GraphicPass>> }, 
    ComputePass { handle: Arc<Mutex<gfx::ComputePass>> }, 
    Buffer { handle: Arc<Mutex<gfx::Buffer>> }, 
    FileTexture { handle: Arc<Mutex<gfx::Texture>> },
    ResourceTexture { handle: Arc<Mutex<gfx::Texture>> },
    Camera { handle: Arc<Mutex<gfx::Camera>> }, 
    Mesh { handle: Arc<Mutex<gfx::Mesh>> }, 
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
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtosResponse {
    SetCurrentBackbuffer(NodeId),
    ClearCurrentBackbuffer,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct ProtosGraphState {
    backbuffer_node: Option<NodeId>,
}

#[derive(Default)]
pub struct ProtosRuntimeState {
    available_size: egui::Vec2,
    egui_image_filter: wgpu::FilterMode,
    egui_texture_id: egui::TextureId,
    dirty_egui_texture: bool,
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

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for ProtosNodeTemplate {
    type NodeData = ProtosNodeData;
    type DataType = ProtosDataType;
    type ValueType = ProtosValueType;
    type UserState = ProtosGraphState;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            ProtosNodeTemplate::BackbufferPass{ .. } => "Backbuffer",
            ProtosNodeTemplate::GraphicPass{ .. } => "Graphic pass",
            ProtosNodeTemplate::ComputePass{ .. } => "Compute pass",
            ProtosNodeTemplate::Buffer{ .. } => "Buffer",
            ProtosNodeTemplate::FileTexture{ .. } => "FileTexture",
            ProtosNodeTemplate::ResourceTexture{ .. } => "ResourceTexture",
            ProtosNodeTemplate::Mesh{ .. } => "Mesh",
            ProtosNodeTemplate::Camera{ .. } => "Camera",
        })
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
                ui.label(param_name);
                //ui.horizontal(|ui| {
                   // ui.label("x");
                   // ui.add(DragValue::new(&mut value.x));
                    //ui.label("y");
                    //ui.add(DragValue::new(&mut value.y));
                //});
            }
            ProtosValueType::Buffer { value } => {
                ui.label(param_name);
                // TODO retrieve value here 
                //ui.horizontal(|ui| {
                    //ui.label(param_name);
                    //ui.add(DragValue::new(v));
                //});
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
                ui.horizontal(|ui| {
                });
            }
            _  => {
                ui.horizontal(|ui| {
                    ui.label("Unknown");
                });
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}

impl UserResponseTrait for ProtosResponse {}
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
        _graph: &Graph<ProtosNodeData, ProtosDataType, ProtosValueType>,
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

type ProtosGraph = Graph<ProtosNodeData, ProtosDataType, ProtosValueType>;
type ProtosEditorState = GraphEditorState<ProtosNodeData, ProtosDataType, ProtosValueType, ProtosNodeTemplate, ProtosGraphState>;

#[derive(Default)]
pub struct ProtosApp {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: ProtosEditorState,

    user_state: ProtosGraphState,
    runtime_state: ProtosRuntimeState,
}

type OutputsCache = HashMap<OutputId, ProtosValueType>;

impl ProtosApp {

    pub fn new() -> Self 
    {
        let runtime_state = ProtosRuntimeState {
            available_size: Vec2::new(500.0, 500.0),
            egui_image_filter: wgpu::FilterMode::Nearest,
            egui_texture_id: egui::TextureId::default(),
            dirty_egui_texture: false,
        };
        #[cfg(feature = "persistence")]
        {
            use std::fs;

            let state_json = fs::read_to_string("state.json");
            let user_state_json = fs::read_to_string("user_state.json");
            if state_json.is_ok() && user_state_json.is_ok() {
                let state = serde_json::from_str(state_json.unwrap().as_str()).unwrap_or_default();
                let user_state = serde_json::from_str(user_state_json.unwrap().as_str()).unwrap_or_default();
                Self {
                    state,
                    user_state,
                    runtime_state
                }
            } else {
                Self {
                    state: ProtosEditorState::default(),
                    user_state: ProtosGraphState::default(),
                    runtime_state
                }
            }
        }
        #[cfg(not(feature = "persistence"))]
        Self {
            state: ProtosEditorState::default(),
            user_state: ProtosGraphState::default(),
            runtime_state
        }
    }

    
    #[cfg(feature = "persistence")]
    /// If the persistence function is enabled,
    /// Called by the frame work to save state before shutdown.
    pub fn save(&mut self) {
        use std::fs;
        let json_state = serde_json::to_string(&self.state).unwrap();
        let json_user_state = serde_json::to_string(&self.user_state).unwrap();
        fs::write("state.json", json_state).expect("Unable to write state file");
        fs::write("user_state.json", json_user_state).expect("Unable to write user state file");
    }
    
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    pub fn ui(&mut self, ctx: &egui::Context, device : &wgpu::Device, cmd : &mut wgpu::CommandEncoder, egui_rpass : &mut egui_wgpu_backend::RenderPass) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Project", |ui| {
                    if ui.button("Compile").clicked() {
                        
                    }
                    if ui.button("Save").clicked() {
                        self.save();
                    }
                });
            });
        });
        // Render zone
        egui::SidePanel::right("RenderPanel")
            .default_width(ctx.used_size().x / 2.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    // Do we really need those ? Texture size exactly match the UI...
                    ui.menu_button("Sampling", |ui| {
                        if ui.button("Nearest").clicked() {
                            self.runtime_state.egui_image_filter = wgpu::FilterMode::Nearest;
                            self.runtime_state.dirty_egui_texture = true;
                        }
                        if ui.button("Linear").clicked() {
                            self.runtime_state.egui_image_filter = wgpu::FilterMode::Linear;
                            self.runtime_state.dirty_egui_texture = true;
                        }
                    });
                });
                self.runtime_state.available_size = ui.available_size();
                if self.user_state.backbuffer_node.is_some() {
                    let node = &self.state.graph[self.user_state.backbuffer_node.unwrap()];

                    match &node.user_data.template {
                        ProtosNodeTemplate::BackbufferPass{ handle } => {
                            let mut pass = handle.lock().unwrap();
                            let render_target_size = egui::Vec2::new(pass.get_width() as f32, pass.get_height() as f32);
                            if self.runtime_state.available_size != render_target_size  {
                                // resize, egui texture id
                                pass.set_size(self.runtime_state.available_size.x as u32, self.runtime_state.available_size.y as u32);
                            }
                            let view_result = pass.get_view_handle();
                            match view_result {
                                Ok(..) => {
                                    let view = view_result.unwrap();
                                    // Create resource if not created.
                                    if self.runtime_state.egui_texture_id == egui::TextureId::default() {
                                        self.runtime_state.egui_texture_id = egui_rpass.egui_texture_from_wgpu_texture(device, view, self.runtime_state.egui_image_filter);
                                    }
                                    if self.runtime_state.dirty_egui_texture {
                                        let update_result = egui_rpass.update_egui_texture_from_wgpu_texture(
                                            device, 
                                            view,
                                            self.runtime_state.egui_image_filter, 
                                            self.runtime_state.egui_texture_id
                                        );
                                        assert!(update_result.is_ok());
                                    }
                                    ui.image(self.runtime_state.egui_texture_id, ui.available_size());
                                },
                                Err(e) => {
                                    let message = format!("{}", e);
                                    ui.add_sized(ui.available_size(), egui::Label::new(message));
                                }
                            }
                        }
                        _ => panic!("Backbuffer node is node a backbuffer pass...")
                    }
                    
                } else {
                    ui.add_sized(ui.available_size(), egui::Label::new("No backbuffer active."));
                }
            });
        
        // Node graph
        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state
                    .draw_graph_editor(ui, AllProtosNodeTemplates, &mut self.user_state)
            })
            .inner;

        // This might not be necessary for protos...
        for node_response in graph_response.node_responses {
            // Here, we ignore all other graph events. But you may find
            // some use for them. For example, by playing a sound when a new
            // connection is created
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    ProtosResponse::SetCurrentBackbuffer(node) => {
                        self.user_state.backbuffer_node = Some(node);
                    }
                    ProtosResponse::ClearCurrentBackbuffer => self.user_state.backbuffer_node = None,
                }
            }
        }
        // Here we must create all resources & cache it & create command buffers...
        // Should have a RUN button.
        if let Some(node) = self.user_state.backbuffer_node {
            if self.state.graph.nodes.contains_key(node) {
                // Evaluate & create nodes
                match self.evaluate_node(device, &self.state.graph, node, &mut HashMap::new()) {
                    Ok(()) => {
                        // Record node.
                        self.record_node(device, cmd, &self.state.graph, node, &mut HashMap::new());
                    }
                    Err(err) => {
                        // TODO: somehow display error to user.
                        //self.user_state.backbuffer_node = None;
                        ctx.debug_painter().text(
                            egui::pos2(10.0, 35.0),
                            egui::Align2::LEFT_TOP,
                            err.to_string(),
                            TextStyle::Button.resolve(&ctx.style()),
                            egui::Color32::WHITE,
                        );
                    }
                }
            } else {
                self.user_state.backbuffer_node = None;
            }
        }
        // TODO: some control window for ui
        egui::Window::new("Window").show(ctx, |ui| {
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally choose either panels OR windows.");
        });
    }

    /// Recursively evaluates all dependencies of this node, then evaluates the node itself.
    /// Should create all resources here & record command list.
    /// Here we return a single output. but node could have multiple outputs...
    pub fn evaluate_node(
        &self,
        device: &wgpu::Device,
        graph: &ProtosGraph,
        node_id: NodeId,
        outputs_cache: &mut OutputsCache,
    ) -> anyhow::Result<()> {
        // This will create data. 
        let node = &graph[node_id];
        match &node.user_data.template {
            ProtosNodeTemplate::BackbufferPass{ handle } => {
                let mut pass = handle.lock().unwrap();
                let input = self.evaluate_input(device, graph, node_id, "input", outputs_cache)?;
                // Check input is valid type.
                if let ProtosValueType::Texture { value } = input {
                    pass.set_origin(value);
                } else {
                    pass.clear_origin();
                }
                pass.set_size(self.runtime_state.available_size.x as u32, self.runtime_state.available_size.y as u32);
                // Will call create if not created already.
                pass.update_data(device);

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
                    let srv = self.evaluate_input(device, graph, node_id, "SRV0", outputs_cache)?;
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
                    desc.set_size(self.runtime_state.available_size.x as u32, self.runtime_state.available_size.y as u32);
                    pass.set_render_target(0, &desc);
                }
                
                // Will call create if not created already.
                pass.update_data(device);
                
                for i in 0..num_attachment {
                    // Output graphic pass will populate output. need to ensure data is created already.
                    // TODO: custom name per output. (MRT support)
                    self.populate_output(graph, node_id, "RT0", ProtosValueType::Texture { value: pass.get_render_target(i) }, outputs_cache);
                }
                
                Ok(())
            }
            ProtosNodeTemplate::ComputePass{ handle: _ } => {
                let a = self.evaluate_input(device, graph, node_id, "A", outputs_cache);
                self.populate_output(graph, node_id, "out", ProtosValueType::Texture { value: a.unwrap().try_to_texture().unwrap() }, outputs_cache);
                
                Ok(())
            }
            ProtosNodeTemplate::Buffer{ handle } => {
                // Inputs for buffer are mostly data... scalar & co
                // Could have buffer type aswell (camera, custom (script based ?), common data (time & res)...)
                // Should be custom nodes instead for cleaner UX (only buffer behind the scene).
                let size = self.evaluate_input(device, graph, node_id, "Size", outputs_cache).unwrap().try_to_scalar();
                let format = self.evaluate_input(device, graph, node_id, "Format", outputs_cache).unwrap().try_to_vec2();
                let mut buffer = handle.lock().unwrap();
                buffer.update_data(device);
                self.populate_output(graph, node_id, "buffer", ProtosValueType::Buffer { value: Some(handle.clone()) }, outputs_cache);
                
                Ok(())
            }
            ProtosNodeTemplate::FileTexture{ handle } => {
                let path = self.evaluate_input(device, graph, node_id, "Path", outputs_cache)?.try_to_string()?;
                let mut texture = handle.lock().unwrap();
                texture.load(path.as_str())?;
                texture.update_data(device);
                self.populate_output(graph, node_id, "texture", ProtosValueType::Texture { value: Some(handle.clone()) }, outputs_cache);

                Ok(())
            }
            ProtosNodeTemplate::ResourceTexture{ handle } => {
                let dimensions = self.evaluate_input(device, graph, node_id, "Dimensions", outputs_cache)?.try_to_vec2()?;
                let mut texture = handle.lock().unwrap();
                texture.set_width(dimensions[0] as u32);
                texture.set_height(dimensions[1] as u32);
                texture.update_data(device);
                self.populate_output(graph, node_id, "texture", ProtosValueType::Texture { value: Some(handle.clone()) }, outputs_cache);
                
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
        &self,
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
                self.record_node(device, cmd, graph, graph[other_output_id].node, outputs_cache);
            }
        }
    }

    pub fn record_node(
        &self,
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
                    self.record_input(device, cmd, graph, node_id, "input", outputs_cache);
                    pass.record_data(device, cmd);
                }
            },
            ProtosNodeTemplate::GraphicPass{ handle } => {
                let pass = handle.lock().unwrap();
                if pass.has_data() {
                    // TODO: for loop
                    self.record_input(device, cmd, graph, node_id, "SRV0", outputs_cache);
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
        graph: &ProtosGraph,
        node_id: NodeId,
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
                match self.evaluate_node(device, graph, graph[other_output_id].node, outputs_cache) {
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