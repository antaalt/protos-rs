
use std::collections::HashMap;

use egui::{self, Vec2, TextStyle};
use egui_node_graph::*;

use crate::{gfx, graph::*};


#[derive(Default)]
pub struct ProtosRuntimeState {
    available_size: egui::Vec2,
    egui_image_filter: wgpu::FilterMode,
    egui_texture_id: egui::TextureId,
    dirty_egui_texture: bool,
}

#[derive(Default)]
pub struct ProtosApp {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: ProtosEditorState,

    user_state: ProtosGraphState,
    runtime_state: ProtosRuntimeState,
}


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
    pub fn ui(&mut self, ctx: &egui::Context, device : &wgpu::Device, queue : &wgpu::Queue, cmd : &mut wgpu::CommandEncoder, egui_rpass : &mut egui_wgpu_backend::RenderPass) {
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
                        ProtosNodeTemplate::BackbufferPass(pass_node) => {
                            gfx::visit_resource_mut(&pass_node.handle, |pass| {
                                // Resize render_target if required.
                                let render_target_size = egui::Vec2::new(pass.get_width() as f32, pass.get_height() as f32);
                                if self.runtime_state.available_size != render_target_size  {
                                    // resize, egui texture id
                                    pass.set_size(self.runtime_state.available_size.x as u32, self.runtime_state.available_size.y as u32);
                                }
                                let view_result = pass.get_view_handle();
                                match view_result {
                                    Ok(view) => {
                                        // Create resource if not created.
                                        if self.runtime_state.egui_texture_id == egui::TextureId::default() {
                                            self.runtime_state.egui_texture_id = egui_rpass.egui_texture_from_wgpu_texture(device, view, self.runtime_state.egui_image_filter);
                                        }
                                        // Should do this update only if backbuffer changed or dirty
                                        //if self.runtime_state.dirty_egui_texture {
                                            let update_result = egui_rpass.update_egui_texture_from_wgpu_texture(
                                                device, 
                                                view,
                                                self.runtime_state.egui_image_filter, 
                                                self.runtime_state.egui_texture_id
                                            );
                                            assert!(update_result.is_ok());
                                        //}
                                        ui.image(self.runtime_state.egui_texture_id, ui.available_size());
                                    },
                                    Err(e) => {
                                        let message = format!("{}", e);
                                        ui.add_sized(ui.available_size(), egui::Label::new(message));
                                    }
                                }
                            });
                        }
                        _ => unreachable!("Backbuffer node is not a backbuffer pass...")
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
        if let Some(node_id) = self.user_state.backbuffer_node {
            if self.state.graph.nodes.contains_key(node_id) {
                // Evaluate & create nodes
                let backbuffer_node = match &self.state.graph.nodes[node_id].user_data.template {
                    ProtosNodeTemplate::BackbufferPass(node_handle) => node_handle,
                    _ => unreachable!("to backbuffer or not to backbuffer ?")
                };
                match backbuffer_node.evaluate(device, queue, &self.state.graph, node_id, self.runtime_state.available_size, &mut HashMap::new()) {
                    Ok(()) => {
                        // Record node.
                        match backbuffer_node.record(device, cmd, &self.state.graph, node_id, &mut HashMap::new()) {
                            Ok(()) => {}
                            Err(err) => {
                                ctx.debug_painter().text(
                                    egui::pos2(10.0, 55.0),
                                    egui::Align2::LEFT_TOP,
                                    err.to_string(),
                                    TextStyle::Button.resolve(&ctx.style()),
                                    egui::Color32::WHITE,
                                );
                            }
                        }
                    }
                    Err(err) => {
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

    
}