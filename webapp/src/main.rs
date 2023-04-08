use crate::prelude::*;
use webui::prelude::*;

mod components;
mod feedback;
mod nav_menu;
mod pages;
mod prelude;

fn main() {
    webui::start_app(setup_app_config());
}

fn setup_app_config() -> AppConfig {
    AppConfig::builder(
        "Web UI Demo & Documentation".to_owned(),
        "Stoic Dreams".to_owned(),
        "https://www.stoicdreams.com".to_owned(),
        "StoicDreams.com".to_owned(),
    )
    .set_header_logo_src("Logo.svg".to_owned())
    .set_nav_routing(nav_menu::get_nav_routing())
    .set_drawer_toggle_header_left(nav_menu::nav_menu_info())
    .set_drawer_toggle_header_middle(feedback_button_info())
    .set_header_strip_bar(header_strip_bar::header_strip_bar)
    .set_user_info_panel(info_panel::info_panel)
    .set_copyright_start(2022)
    .build()
}
