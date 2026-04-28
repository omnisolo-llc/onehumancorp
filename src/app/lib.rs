#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod app {
    slint::include_modules!();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    // This is the entry point for the WASM app
    let ui = app::AppWindow::new().unwrap();
    ui.run().unwrap();
}

// Dummy lib for non-wasm targets if needed
#[cfg(not(target_arch = "wasm32"))]
pub fn dummy() {}
