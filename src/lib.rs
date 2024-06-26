#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod protos;
mod app;
mod gfx;
mod graph;

pub use app::run;

// ----------------------------------------------------------------------------
// When compiling for web:
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    use std::panic;
    console_log::init_with_level(log::Level::Debug).expect("could not initialize logger");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    app::run().await;

    Ok(())
}