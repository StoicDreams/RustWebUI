use crate::{
    app_state_agent::{AppStateAgent, AppStateReceiverMessage, AppStateRequest},
    data_types::nav_route::{NavLinkInfo, NavRoute},
    interop, AppConfig,
};
use yew::{html, Callback, Component, Html};
use yew_agent::{Bridge, Bridged};

pub(crate) struct AppBody {
    app_state_agent: Box<dyn Bridge<AppStateAgent>>,
    current_path: String,
}

impl Component for AppBody {
    type Message = AppStateReceiverMessage;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            app_state_agent: AppStateAgent::bridge(
                ctx.link()
                    .callback(AppStateReceiverMessage::AppStateMessage),
            ),
            current_path: interop::get_path().to_lowercase(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppStateReceiverMessage::AppStateMessage(message) => match message {
                AppStateRequest::PathUpdate(path) => {
                    if path.to_lowercase() == self.current_path {
                        return false;
                    }
                    self.current_path = path.to_lowercase();
                    return true;
                }
            },
            AppStateReceiverMessage::None => false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let (app_config, _) = ctx
            .link()
            .context::<AppConfig>(Callback::noop())
            .expect("no app config found");
        let path = self.current_path.to_owned();
        html! {
            <main>
                {(get_page_content(app_config.nav_routing, &path))()}
            </main>
        }
    }
}

fn page_not_found() -> Html {
    html! {
        {"Page Not Found"}
    }
}

fn get_page_content(routes: Vec<NavRoute>, page: &str) -> fn() -> Html {
    let page = get_page(routes, page);
    match page {
        Option::Some(link_info) => link_info.page,
        Option::None => page_not_found,
    }
}

fn get_page(routes: Vec<NavRoute>, page: &str) -> Option<NavLinkInfo> {
    for route in routes {
        match route {
            NavRoute::NavLink(link_info) => {
                if link_info.path.to_lowercase() == page.to_lowercase() {
                    return Option::Some(link_info);
                }
            }
            NavRoute::NavGroup(group_info) => {
                if group_info.children.len() == 0 {
                    continue;
                }
                if let Option::Some(link_info) = get_page(group_info.children, page) {
                    return Option::Some(link_info);
                }
            }
        }
    }
    Option::None
}
