
use std::{borrow::Cow, collections::HashMap, sync::{Arc, Mutex}, fmt::{self, Formatter}};

use egui::{self, DragValue, TextStyle};
use egui_node_graph::*;

use crate::gfx::{self};


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
    Image,
    RawBuffer,
    ConstantBuffer,
    // constant node
    Scalar, // float
    Vec2,   // float2
    Vec3,   // float3
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
    Image { value: Option<Arc<Mutex<gfx::Texture>>> },
    RawBuffer { value: Option<Arc<Mutex<gfx::Buffer>>> },
    ConstantBuffer { value: Option<Arc<Mutex<gfx::Buffer>>> },
    Scalar { value: f32 },
    Vec2 { value: [f32; 2] },
    Vec3 { value: [f32; 3] },
}

impl Default for ProtosValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Unknown {}
    }
}

impl ProtosValueType {
    /// Tries to downcast this value type to a vector
    pub fn try_to_texture(self) -> anyhow::Result<Option<Arc<Mutex<gfx::Texture>>>> {
        if let ProtosValueType::Texture { value } = self {
            Ok(value.clone())
        } else {
            anyhow::bail!("Invalid cast to texture")
        }
    }

    /// Tries to downcast this value type to a scalar
    pub fn try_to_image(self) -> anyhow::Result<Option<Arc<Mutex<gfx::Texture>>>> {
        if let ProtosValueType::Image { value } = self {
            Ok(value.clone())
        } else {
            anyhow::bail!("Invalid cast to image")
        }
    }
}

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ProtosNodeTemplate {
    //Backbuffer { handle: Arc<Mutex<gfx::Texture>> }, // Just a texture.
    GraphicPass { handle: Arc<Mutex<gfx::GraphicPass>> }, 
    ComputePass { handle: Arc<Mutex<gfx::ComputePass>> }, 
    Buffer { handle: Arc<Mutex<gfx::Buffer>> }, 
    Texture { handle: Arc<Mutex<gfx::Texture>> }, 
    Camera { handle: Arc<Mutex<gfx::Camera>> }, 
    Mesh { handle: Arc<Mutex<gfx::Mesh>> }, 
}

impl ProtosNodeTemplate {
    pub fn can_be_recorded(&self) -> bool {
        match self {
            // Can
            ProtosNodeTemplate::GraphicPass{ handle } => true,
            ProtosNodeTemplate::ComputePass{ handle } => true,
            // Cannot
            ProtosNodeTemplate::Buffer{ handle } => false,
            ProtosNodeTemplate::Texture{ handle } => false,
            ProtosNodeTemplate::Camera{ handle } => false,
            ProtosNodeTemplate::Mesh{ handle } => false,
        }
    }
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtosResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct ProtosGraphState {
    pub backbuffer_node: Option<NodeId>,
}

// =========== Then, you need to implement some traits ============

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<ProtosGraphState> for ProtosDataType {
    fn data_type_color(&self, _user_state: &mut ProtosGraphState) -> egui::Color32 {
        match self {
            ProtosDataType::Unknown => egui::Color32::from_rgb(238, 207, 109),
            ProtosDataType::Texture => egui::Color32::from_rgb(38, 109, 211),
            ProtosDataType::Image => egui::Color32::from_rgb(238, 207, 109),
            ProtosDataType::RawBuffer => egui::Color32::from_rgb(238, 207, 109),
            ProtosDataType::ConstantBuffer => egui::Color32::from_rgb(238, 207, 109),
            ProtosDataType::Scalar => egui::Color32::from_rgb(238, 207, 109),
            ProtosDataType::Vec2 => egui::Color32::from_rgb(238, 207, 109),
            ProtosDataType::Vec3 => egui::Color32::from_rgb(238, 207, 109),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            ProtosDataType::Unknown => Cow::Borrowed("unknown"),
            ProtosDataType::Texture => Cow::Borrowed("texture"),
            ProtosDataType::Image => Cow::Borrowed("image"),
            ProtosDataType::RawBuffer => Cow::Borrowed("rawbuffer"),
            ProtosDataType::ConstantBuffer => Cow::Borrowed("constantbuffer"),
            ProtosDataType::Scalar => Cow::Borrowed("scalar"),
            ProtosDataType::Vec2 => Cow::Borrowed("vec2"),
            ProtosDataType::Vec3 => Cow::Borrowed("vec3"),
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
            ProtosNodeTemplate::GraphicPass{ .. } => "New graphic pass",
            ProtosNodeTemplate::ComputePass{ .. } => "New compute pass",
            ProtosNodeTemplate::Buffer{ .. } => "New buffer",
            ProtosNodeTemplate::Texture{ .. } => "New texture",
            ProtosNodeTemplate::Mesh{ .. } => "New mesh",
            ProtosNodeTemplate::Camera{ .. } => "New Camera",
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
                ProtosDataType::Image,
                ProtosValueType::Image { value: None },
                InputParamKind::ConnectionOnly,
                true,
            );
        };

        let output_texture = |graph: &mut ProtosGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), ProtosDataType::Texture);
        };
        let output_image = |graph: &mut ProtosGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), ProtosDataType::Image);
        };

        // This need to match evaluate_node
        match self {
            ProtosNodeTemplate::GraphicPass{ handle } => {
                // TODO for loop
                input_texture(graph, "SRV0".into());
                // TODO for loop
                output_image(graph, "RT0".into());
            }
            ProtosNodeTemplate::ComputePass{ handle } => {
                // TODO for loop
                input_image(graph, "UAV0".into());
                output_image(graph, "UAV0".into());
            }
            ProtosNodeTemplate::Buffer{ handle } => {
                graph.add_input_param(
                    node_id,
                    String::from("Size"),
                    ProtosDataType::Scalar,
                    ProtosValueType::Scalar { value: 0.0 },
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
                graph.add_input_param(
                    node_id,
                    String::from("Format"),
                    ProtosDataType::Scalar,
                    ProtosValueType::Scalar { value: 0.0 },
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
                graph.add_output_param(
                    node_id, 
                    String::from("Buffer"),
                    ProtosDataType::RawBuffer
                );
            }
            ProtosNodeTemplate::Texture{ handle } => {
                input_image(graph, "v1");
                input_image(graph, "v2");
                output_image(graph, "out");
            }
            ProtosNodeTemplate::Camera{ handle } => {
                input_image(graph, "v1");
                input_image(graph, "v2");
                output_image(graph, "out");
            }
            ProtosNodeTemplate::Mesh{ handle } => {
                input_texture(graph, "x");
                input_texture(graph, "y");
                output_image(graph, "out");
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
            ProtosNodeTemplate::GraphicPass{ handle: Arc::default() },
            ProtosNodeTemplate::ComputePass{ handle: Arc::default() },
            ProtosNodeTemplate::Buffer{ handle: Arc::default() },
            ProtosNodeTemplate::Texture{ handle: Arc::default() },
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
                ui.horizontal(|ui| {
                   // ui.label("x");
                   // ui.add(DragValue::new(&mut value.x));
                    //ui.label("y");
                    //ui.add(DragValue::new(&mut value.y));
                });
            }
            ProtosValueType::Image { value } => {
                // TODO retrieve value here 
                ui.horizontal(|ui| {
                    //ui.label(param_name);
                    //ui.add(DragValue::new(v));
                });
            }
            ProtosValueType::RawBuffer { value } => {
                // TODO retrieve value here 
                ui.horizontal(|ui| {
                    //ui.label(param_name);
                    //ui.add(DragValue::new(v));
                });
            }
            ProtosValueType::ConstantBuffer { value } => {
                // TODO retrieve value here 
                ui.horizontal(|ui| {
                    //ui.label(param_name);
                    //ui.add(DragValue::new(v));
                });
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
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        /*let is_active = user_state
            .backbuffer_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        if !is_active {
            if ui.button("👁 Set active").clicked() {
                responses.push(NodeResponse::User(ProtosResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(ProtosResponse::ClearActiveNode));
            }
        }*/

        responses
    }
}

type ProtosGraph = Graph<ProtosNodeData, ProtosDataType, ProtosValueType>;
type ProtosEditorState = GraphEditorState<ProtosNodeData, ProtosDataType, ProtosValueType, ProtosNodeTemplate, ProtosGraphState>;

struct ProtosRenderTarget {
    texture: wgpu::Texture,
    view: wgpu::TextureView,

    egui: egui::TextureId,
    size: egui::Vec2,
}

#[derive(Default)]
pub struct ProtosApp {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: ProtosEditorState,

    user_state: ProtosGraphState,

    //render_graph: gfx::RenderGraph,
    
    render_target : Option<ProtosRenderTarget>,
}

#[cfg(feature = "persistence")]
const PERSISTENCE_KEY: &str = "protos_rs";

impl ProtosApp {

    fn create_render_target(width: u32, height: u32, device : &wgpu::Device, egui_rpass : &mut egui_wgpu_backend::RenderPass) -> ProtosRenderTarget
    {
        let label = Some("RenderTarget");
        let size = wgpu::Extent3d {
            width: 500,
            height: 500,
            depth_or_array_layers: 1,
        };
        let texture_format = wgpu::TextureFormat::Rgba8Unorm;
        let texture_view_format = texture_format;
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: texture_format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                //view_formats: &[texture_view_format],
            }
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let egui_texid = egui_rpass.egui_texture_from_wgpu_texture(&device, &view, wgpu::FilterMode::Linear); // TODO: add toggle for linear / nearest.

        ProtosRenderTarget {
            texture, 
            view,
            egui: egui_texid,
            size: egui::Vec2::new(width as f32, height as f32),
        }
    }

    pub fn new(cc: &egui::Context, device : &wgpu::Device, egui_rpass : &mut egui_wgpu_backend::RenderPass) -> Self 
    {
        let texture = Self::create_render_target(500, 500, device, egui_rpass);
        /// If the persistence feature is enabled, Called once before the first frame.
        /// Load previous app state (if any).
        #[cfg(feature = "persistence")]
        {
            /*let state = cc
                .storage
                .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
                .unwrap_or_default();
            Self {
                state,
                user_state: ProtosGraphState::default(),
                render_target: texture,
            }*/
        }
        Self {
            state: ProtosEditorState::default(),
            user_state: ProtosGraphState::default(),
            //render_graph: RenderGraph::default(),
            render_target: Some(texture),
        }
    }

    
    #[cfg(feature = "persistence")]
    /// If the persistence function is enabled,
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.state);
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    pub fn ui(&mut self, ctx: &egui::Context, device : &wgpu::Device, egui_rpass : &mut egui_wgpu_backend::RenderPass) {
        let mut compile = false;
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.menu_button("Project", |ui| {
                    if ui.button("Compile").clicked() {
                        compile = true;
                    }
                });
            });
        });
        // Render zone
        egui::SidePanel::right("RenderPanel")
            .resizable(true)
            .default_width(ctx.used_size().x / 2.0)
            .show(ctx, |ui| {
                let render_target_size = self.render_target.as_ref().unwrap().size;
                let available_size = ui.available_size();
                if available_size != render_target_size  {
                    self.render_target = Some(Self::create_render_target(available_size.x as u32, available_size.y as u32, device, egui_rpass));
                }
                let render_target = self.render_target.as_ref().unwrap().egui;
                ui.image(render_target, ui.available_size());
            });
        
        // Node graph
        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state
                    .draw_graph_editor(ui, AllProtosNodeTemplates, &mut self.user_state)
            })
            .inner;

        // This might not be necessary for protos...
        /*for node_response in graph_response.node_responses {
            // Here, we ignore all other graph events. But you may find
            // some use for them. For example, by playing a sound when a new
            // connection is created
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    ProtosResponse::SetActiveNode(node) => self.user_state.active_node = Some(node),
                    ProtosResponse::ClearActiveNode => self.user_state.active_node = None,
                }
            }
        }*/
        // Here we must create all resources & cache it & create command buffers...
        // Should have a RUN button.
        if compile {
            if let Some(node) = self.user_state.backbuffer_node {
                if self.state.graph.nodes.contains_key(node) {
                    evaluate_node(&self.state.graph, node, &mut HashMap::new());
                } else {
                    self.user_state.backbuffer_node = None;
                    println!("Backbuffer node was deleted OMGGGGGGG");
                }
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
}

type OutputsCache = HashMap<OutputId, ProtosValueType>;

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
/// Should create all resources here & record command list.
/// Here we return a single output. but node could have multiple outputs...
pub fn evaluate_node(
    graph: &ProtosGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) {
    // To solve a similar problem as creating node types above, we define an
    // Evaluator as a convenience. It may be overkill for this small example,
    // but something like this makes the code much more readable when the
    // number of nodes starts growing.

    /*struct Evaluator<'a> {
        graph: &'a ProtosGraph,
        outputs_cache: &'a mut OutputsCache,
        node_id: NodeId,
    }
    impl<'a> Evaluator<'a> {
        fn new(graph: &'a ProtosGraph, outputs_cache: &'a mut OutputsCache, node_id: NodeId) -> Self {
            Self {
                graph,
                outputs_cache,
                node_id,
            }
        }
        fn evaluate_input(&mut self, name: &str) -> anyhow::Result<ProtosValueType> {
            // Calling `evaluate_input` recursively evaluates other nodes in the
            // graph until the input value for a paramater has been computed.
            evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
        }
        fn populate_output(
            &mut self,
            name: &str,
            value: ProtosValueType,
        ) -> anyhow::Result<ProtosValueType> {
            // After computing an output, we don't just return it, but we also
            // populate the outputs cache with it. This ensures the evaluation
            // only ever computes an output once.
            //
            // The return value of the function is the "final" output of the
            // node, the thing we want to get from the evaluation. The example
            // would be slightly more contrived when we had multiple output
            // values, as we would need to choose which of the outputs is the
            // one we want to return. Other outputs could be used as
            // intermediate values.
            //
            // Note that this is just one possible semantic interpretation of
            // the graphs, you can come up with your own evaluation semantics!
            populate_output(self.graph, self.outputs_cache, self.node_id, name, value)
        }
        fn input_image(&mut self, name: &str) -> anyhow::Result<Option<Arc<Mutex<gfx::Texture>>>> {
            self.evaluate_input(name)?.try_to_image()
        }
        fn input_texture(&mut self, name: &str) -> anyhow::Result<Option<Arc<Mutex<gfx::Texture>>>> {
            self.evaluate_input(name)?.try_to_texture()
        }
        fn output_image(&mut self, name: &str, value: Option<Arc<Mutex<gfx::Texture>>>) -> anyhow::Result<ProtosValueType> {
            self.populate_output(name, ProtosValueType::Image { value })
        }
        fn output_texture(&mut self, name: &str, value: Option<Arc<Mutex<gfx::Texture>>>) -> anyhow::Result<ProtosValueType> {
            self.populate_output(name, ProtosValueType::Texture { value })
        }
        fn output_graphic_pass(&mut self, name: &str, rt0: Option<Arc<Mutex<gfx::Texture>>>) -> anyhow::Result<ProtosValueType> {
            self.populate_output(name, ProtosValueType::Texture { value: rt0 })
        }
    }*/
    // This will create data. 
    let node = &graph[node_id];
    //let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
    match &node.user_data.template {
        ProtosNodeTemplate::GraphicPass{ handle } => {
            // Here we should call all input_xxx, which will update the description of the graphic pass.
            // We can then update or not the pipeline with given data.
            // For recording command list, we will need something similar just running for graphic pass & compute pass.

            let mut pass = handle.lock().unwrap();
            // Input & co should be templated somewhere, trait to get these informations ?
            for i in 0..1 {
                //let name = format!("SRV{}", i)
                let srv = evaluate_input(graph, node_id, "SRV0", outputs_cache);
                // Check input is used
                if srv.is_ok() {
                    let s = srv.unwrap();
                    // Check input is valid type.
                    if let ProtosValueType::Image { value } = s {
                        // Check its data is filled.
                        if value.is_some() {
                            pass.set_shader_resource_view(i, value.unwrap());
                        } else {
                            pass.clear_shader_resource_view(i);
                        }
                    } else {
                        pass.clear_shader_resource_view(i);
                    }
                } else {
                    // input not filled.
                    pass.clear_shader_resource_view(i);
                }
            }
            // If we pass UAV, they are outputs aswell...
            // Can pass a rt as input in order to render it.
            /*let rt0 = evaluate_input(graph, node_id, "RT0", outputs_cache);
            if rt0.is_ok() {
                pass.set_render_target(0, rt0.unwrap());
            } else {
                // set dummy data...
            }*/
            // Will call create if not created already.
            //pass.update_data(&device);
            
            // Output graphic pass will populate output. need to ensure data is created already.
            populate_output(graph, outputs_cache, node_id, "RT0", ProtosValueType::Texture { value: pass.get_render_target(0) });
        }
        ProtosNodeTemplate::ComputePass{ handle: _ } => {
            let a = evaluate_input(graph, node_id, "A", outputs_cache);
            populate_output(graph, outputs_cache, node_id, "out", ProtosValueType::Texture { value: a.unwrap().try_to_texture().unwrap() });
        }
        ProtosNodeTemplate::Buffer{ handle: _ } => {
            let scalar = evaluate_input(graph, node_id, "scalar", outputs_cache).unwrap().try_to_image();
            let vector = evaluate_input(graph, node_id, "vector", outputs_cache).unwrap().try_to_image();
            populate_output(graph, outputs_cache, node_id, "out", ProtosValueType::Texture { value: scalar.unwrap() });
        }
        ProtosNodeTemplate::Texture{ handle: _ } => {
            let scalar = evaluate_input(graph, node_id, "v1", outputs_cache);
            let vector = evaluate_input(graph, node_id, "v2", outputs_cache);
            populate_output(graph, outputs_cache, node_id, "out", ProtosValueType::Texture { value: scalar.unwrap().try_to_texture().unwrap() });
        }
        ProtosNodeTemplate::Camera{ handle: _ } => {
            let scalar = evaluate_input(graph, node_id, "v1", outputs_cache);
            let vector = evaluate_input(graph, node_id, "v2", outputs_cache);
            populate_output(graph, outputs_cache, node_id, "out", ProtosValueType::Texture { value: scalar.unwrap().try_to_texture().unwrap() });
        }
        ProtosNodeTemplate::Mesh{ handle: _ } => {
            let scalar = evaluate_input(graph, node_id, "x", outputs_cache);
            let vector = evaluate_input(graph, node_id, "y", outputs_cache);
            populate_output(graph, outputs_cache, node_id, "out", ProtosValueType::Texture { value: scalar.unwrap().try_to_texture().unwrap() });
        }
        _ => unimplemented!("Missing template implementation")
    }
}

fn populate_output(
    graph: &ProtosGraph,
    outputs_cache: &mut OutputsCache,
    node_id: NodeId,
    param_name: &str,
    value: ProtosValueType,
) {
    let output_id = graph[node_id].get_output(param_name).unwrap();
    outputs_cache.insert(output_id, value.clone());
}

// Evaluates the input value of
fn evaluate_input(
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
            // Calling this will populate the cache
            evaluate_node(graph, graph[other_output_id].node, outputs_cache);

            // Now that we know the value is cached, return it
            Ok(outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated").clone())
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value.clone())
    }
}
