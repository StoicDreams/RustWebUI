//! # Web UI
//!
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.

pub use crate::components::container::paper::Paper;
pub use crate::data_types::app_config::AppConfig;

use components::layout::app::start_webui_app;

pub mod agents;
pub mod components;

/// Data Types
pub mod data_types;

/// Initializer to run in app main() to start website
///
/// example
/// ```rust,ignore
/// use webui::AppConfig;
///
/// fn main() {
///     let app_config: AppConfig = AppConfig {
///         app_name: "Web UI".to_owned(),
///         company_name: "Sample Company".to_owned(),
///         company_home_url: "https://www.stoicdreams.com".to_owned(),
///         domain: "StoicDreams.com".to_owned(),
///         hide_powered_by: false,
///         body_html: page_routes::body_html,
///         header_left_drawer_toggle: Option::Some(nav_menu::nav_menu_info()),
///     };
///     webui::start_app(app_config);
/// }
/// ```
pub fn start_app(app_config: AppConfig) {
    console_log::init_with_level(log::Level::Debug);
    start_webui_app(app_config);
}
