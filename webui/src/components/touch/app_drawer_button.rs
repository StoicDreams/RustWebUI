use crate::prelude::*;
use yew::{use_state, UseStateHandle};

/// Properties for NavLink component
#[derive(Debug, Properties, PartialEq)]
pub struct AppDrawerButtonProps {
    pub info: Option<DrawerToggleInfo>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub logosrc: Option<String>,
    #[prop_or_default]
    pub logotitle: Option<String>,
    #[prop_or_default]
    pub always_show_logo: bool,
}

/// Button that is used to trigger opening one of the four app drawers.
///
/// Left and right app drawers are side panels that pop out with a width dependent on their content.
/// Top and Bottom app drawers act more like dialogs|modals, sliding out and displaying in the center of the page.
#[function_component(AppDrawerButton)]
pub(crate) fn app_drawer_button(props: &AppDrawerButtonProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let title_context = contexts.clone();
    let logo_src_handle: UseStateHandle<Option<String>> = use_state(|| None);
    let logo_title_handle: UseStateHandle<String> = use_state(String::default);
    let drawer_info = &props.info;
    let logo_src = logo_src_handle.deref().to_owned();
    let logo_title = logo_title_handle.deref().to_owned();
    let drawer_info_click = drawer_info.clone();
    let contexts_onclick = contexts.clone();
    let setup_onclick = Callback::from(move |_| {
        let drawer_info_click = drawer_info_click.clone();
        if let Some(info) = drawer_info_click {
            let options = info.get_options();
            contexts_onclick
                .drawer
                .set(DrawerMessage::ToggleDrawer(options));
        };
    });
    let children = &props.children;

    html! {
        <>
            {match drawer_info.clone() {
                Some(drawer_info) => {
                    let title = (drawer_info.title)(title_context);
                    let btn_class = if drawer_info.class.is_empty() {"btn toggle elevation-1".to_string()} else {drawer_info.class.to_string()};
                    html! {
                        <button type="button" title={title.to_owned()} class={props.class.to_string()}
                            aria-label={title.to_owned()}
                            onclick={setup_onclick}>
                            <span class={btn_class}>{(drawer_info.display)(contexts.clone())}</span>
                            {match &logo_src {
                                Some(logo) => {
                                    html! {
                                        <img class="pl-1" src={logo.to_string()} title={logo_title.to_owned()} />
                                    }
                                },
                                None => html! {}
                            }}
                            {for children.iter()}
                        </button>
                    }
                },
                None => html! {
                    if props.always_show_logo {
                        {match &logo_src {
                            Some(logo) => {
                                html! {
                                    <Paper>
                                        <img src={logo.to_string()} title={logo_title.to_owned()} />
                                    </Paper>
                                }
                            },
                            None => html! {}
                        }}
                    }
                }
            }}
        </>
    }
}
