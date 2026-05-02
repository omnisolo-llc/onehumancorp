pub mod login;
pub mod wizard;
pub mod dashboard;
pub mod referrals;
pub mod builder;
pub mod pricing;
pub mod agents;
pub mod settings;
pub mod chat;
pub mod ai_config;
pub mod skills;
pub mod grow;
pub mod meetings;
pub mod ongoing;
pub mod tasklist;
pub mod users;
pub mod security;
pub mod pipelines;
pub mod integrations;
pub mod diagnostics;
pub mod handoffs;
pub mod scaling;
pub mod memory;
pub mod hire;
pub mod landing;
pub mod share;
pub mod checklist;
pub mod indicators;
pub mod velocity;
pub mod observability;
pub mod help;
pub mod docs;
pub mod tutorials;
pub mod notes;
pub mod chat_help;
pub mod walkthrough;
pub mod my_plan;
pub mod channels;
pub mod cost_dashboard;
pub mod logs;
pub mod prompt_tuning;
pub mod secure_agent_config;
pub mod agent_config;

use std::sync::Once;
static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        #[cfg(not(target_arch = "wasm32"))]
        {
            struct HeadlessPlatform;
            impl slint::platform::Platform for HeadlessPlatform {
                fn create_window_adapter(
                    &self,
                ) -> Result<std::rc::Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
                    let window = slint::platform::software_renderer::MinimalSoftwareWindow::new(
                        slint::platform::software_renderer::RepaintBufferType::NewBuffer,
                    );
                    struct AdapterWrapper(
                        std::rc::Rc<slint::platform::software_renderer::MinimalSoftwareWindow>,
                    );
                    impl slint::platform::WindowAdapter for AdapterWrapper {
                        fn window(&self) -> &slint::Window {
                            self.0.window()
                        }
                        fn size(&self) -> slint::PhysicalSize {
                            self.0.size()
                        }
                        fn renderer(&self) -> &dyn slint::platform::Renderer {
                            self.0.renderer()
                        }
                    }
                    Ok(std::rc::Rc::new(AdapterWrapper(window)))
                }
            }
            let _ = slint::platform::set_platform(Box::new(HeadlessPlatform));
        }
    });
}
