pub mod app {
    include!(concat!(env!("OUT_DIR"), "/app.rs"));
}

use slint::ComponentHandle;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        let _ = console_log::init_with_level(log::Level::Debug);

        let login_ui = app::Login::new().unwrap();

        if let Some(web_window) = web_sys::window() {
            let w = web_window.inner_width().ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1280.0) as u32;
            let h = web_window.inner_height().ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(720.0) as u32;
            login_ui.window().set_size(slint::WindowSize::Logical(
                slint::LogicalSize { width: w as f32, height: h as f32 }
            ));
        }

        login_ui.run().unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let login_ui = app::Login::new().unwrap();
        login_ui.run().unwrap();
    }
}
