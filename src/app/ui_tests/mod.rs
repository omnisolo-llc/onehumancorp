pub mod agent_config;
pub mod agents;
pub mod ai_config;
pub mod builder;
pub mod channels;
pub mod chat;
pub mod chat_help;
pub mod checklist;
pub mod cost_dashboard;
pub mod dashboard;
pub mod diagnostics;
pub mod docs;
pub mod grow;
pub mod handoffs;
pub mod help;
pub mod hire;
pub mod indicators;
pub mod integrations;
pub mod landing;
pub mod login;
pub mod logs;
pub mod meetings;
pub mod memory;
pub mod my_plan;
pub mod notes;
pub mod ongoing;
pub mod pipelines;
pub mod pricing;
pub mod prompt_tuning;
pub mod referrals;
pub mod scaling;
pub mod secure_agent_config;
pub mod security;
pub mod settings;
pub mod share;
pub mod skills;
pub mod tasklist;
pub mod tutorials;
pub mod users;
pub mod walkthrough;
pub mod wizard;

pub fn init() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        struct HeadlessPlatform;
        impl slint::platform::Platform for HeadlessPlatform {
            fn create_window_adapter(
                &self,
            ) -> Result<std::rc::Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError>
            {
                thread_local! {
                    static WINDOW: std::rc::Rc<slint::platform::software_renderer::MinimalSoftwareWindow> = {
                        slint::platform::software_renderer::MinimalSoftwareWindow::new(
                            slint::platform::software_renderer::RepaintBufferType::NewBuffer,
                        )
                    };
                }
                let window = WINDOW.with(|w| w.clone());
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
            fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
                Ok(())
            }
        }
        let _ = slint::platform::set_platform(Box::new(HeadlessPlatform));
    }
}
pub mod login_settings;
pub mod social_media;
