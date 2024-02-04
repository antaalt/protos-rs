#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::iter;
use chrono::Timelike;
use egui_wgpu::{Renderer, renderer::ScreenDescriptor};
use egui_winit::State;
use winit::event::{Event, KeyEvent, WindowEvent};

const INITIAL_WIDTH: u32 = 1280;
const INITIAL_HEIGHT: u32 = 720;

/// A custom event type for the winit app.
/*enum Event {
    RequestRedraw,
}

/// This is the repaint signal type that egui needs for requesting a repaint from another thread.
/// It sends the custom RequestRedraw event to the winit event loop.
struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}*/

/// A simple egui + wgpu + winit based example.
#[cfg(not(target_arch = "wasm32"))] // TODO support wasm
fn main() {
    use winit::keyboard::{Key, NamedKey};

    std::env::set_var("RUST_BACKTRACE", "1");
    /*std::panic::set_hook(Box::new(|_panic_info| {
        let backtrace = backtrace::Backtrace::new();
        //  Do something with backtrace and panic_info.
        println!("{:?}", backtrace);
    }));*/


    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("Failed to create event loop");
    let window = winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("Protos RS")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        flags: wgpu::InstanceFlags::default(),
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });
    let surface = unsafe { instance.create_surface(&window).expect("Failed to create surface") };

    // WGPU 0.11+ support force fallback (if HW implementation not supported), set it to true or false (optional).
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::VERTEX_WRITABLE_STORAGE | wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ))
    .unwrap();

    let size = window.inner_size();
    let capabilities = surface.get_capabilities(&adapter);
    let surface_format = capabilities.formats[0];
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
    };
    surface.configure(&device, &surface_config);

    let egui_context = egui::Context::default();
    // We use the egui_winit_platform crate as the platform.
    let mut state = State::new(egui_context.clone(), egui_context.viewport_id(), &window, None, None);

    // We use the egui_wgpu_backend crate as the render backend.
    let mut egui_renderer = Renderer::new(&device, surface_format, None, 1);

    // Display the demo application that ships with egui.
    //let mut demo_app = egui_demo_lib::DemoWindows::default();

    // Create Protos app
    let mut protos_app = protos_rs::ProtosApp::new();

    //let start_time = Instant::now();
    let _ = event_loop.run(move |event, elwt| {
        // Pass the winit events to the platform integration.
        if let Event::WindowEvent { event, .. } = event {
            let _ = state.on_window_event(&window, &event);

            match event {
                WindowEvent::RedrawRequested => {
                    //platform.update_time(start_time.elapsed().as_secs_f64());

                    let output_frame = match surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(wgpu::SurfaceError::Outdated) => {
                            // This error occurs when the app is minimized on Windows.
                            // Silently return here to prevent spamming the console with:
                            // "The underlying surface has changed, and therefore the swap chain must be updated"
                            return;
                        }
                        Err(e) => {
                            eprintln!("Dropped frame with error: {}", e);
                            return;
                        }
                    };
                    let output_view = output_frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());


                    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("encoder"),
                    });


                    // End the UI frame. We could now handle the output and draw the UI with the backend.
                    let raw_input = state.take_egui_input(&window);
                    let full_output = egui_context.run(raw_input, |ui| {
                        // Draw the demo application.
                        let _ = ui;
                        protos_app.ui(&egui_context, &device, &queue, &mut encoder, &mut egui_renderer);
                    });
                    state.handle_platform_output(&window, full_output.platform_output);
                    
                    let tris = egui_context
                        .tessellate(full_output.shapes, full_output.pixels_per_point);
                    for (id, image_delta) in &full_output.textures_delta.set {
                        egui_renderer.update_texture(&device, &queue, *id, &image_delta);
                    }
                    // Upload all resources for the GPU.
                    let screen_descriptor = ScreenDescriptor {
                        size_in_pixels: [surface_config.width, surface_config.height],
                        pixels_per_point: window.scale_factor() as f32,
                    };
                    egui_renderer.update_buffers(&device, &queue, &mut encoder, &tris, &screen_descriptor);
                    
                    // Record all render passes.
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &output_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        label: Some("egui main render pass"),
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    egui_renderer.render(&mut rpass, &tris, &screen_descriptor);
                    drop(rpass);
                    for x in &full_output.textures_delta.free {
                        egui_renderer.free_texture(x)
                    }
                    // Submit the commands.
                    queue.submit(iter::once(encoder.finish()));

                    // Redraw egui
                    output_frame.present();

                    // Support reactive on windows only, but not on linux.
                    // if _output.needs_repaint {
                    //     *control_flow = ControlFlow::Poll;
                    // } else {
                    //     *control_flow = ControlFlow::Wait;
                    // }
                }
                /*MainEventsCleared | UserEvent(Event::RequestRedraw) => {
                    window.request_redraw();
                }*/
                WindowEvent::Resized(physical_size) => {
                    // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                    // See: https://github.com/rust-windowing/winit/issues/208
                    // This solves an issue where the app would panic when minimizing on Windows.
                    if physical_size.width > 0 && physical_size.height > 0 {
                        surface_config.width = physical_size.width;
                        surface_config.height = physical_size.height;
                        surface.configure(&device, &surface_config);
                    }
                }
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    event: KeyEvent  {
                        state: winit::event::ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                    ..
                } => elwt.exit(),
                WindowEvent::Destroyed => {
                    #[cfg(feature = "persistence")]
                    protos_app.save();
                }
                _ => (),
            }
        }
    });
}

/// Time of day as seconds since midnight. Used for clock in demo app.
pub fn seconds_since_midnight() -> f64 {
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}
