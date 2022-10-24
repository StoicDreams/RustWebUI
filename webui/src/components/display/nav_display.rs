use yew::Callback;

use crate::*;

/// Properties for NavDislay component
#[derive(Properties, PartialEq)]
pub struct NavDisplayProps {
    #[prop_or_default]
    pub routes: Vec<NavRoute>,
    #[prop_or_default]
    pub class: String,
}

/// Component for display a navigation tree
#[function_component(NavDisplay)]
pub fn nav_display(props: &NavDisplayProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("nav-display");

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    html! {
        <section class={classes.clone()}>
            {nav_display_group(&props.routes)}
            <div class="grow" />
        </section>
    }
}

fn nav_display_group(routes: &Vec<NavRoute>) -> Html {
    html! {
        <>
            {
                routes.into_iter().map(|route| {
                    html!{display_nav_route(route)}
                }).collect::<Html>()
            }
        </>
    }
}

fn display_nav_route(route: &NavRoute) -> Html {
    match route {
        NavRoute::NavGroup(group_info) => {
            return html! {
                <Paper class="nav-group">
                    <DisplayNavGroupToggle group_info={group_info.to_owned()} />
                    {nav_display_group(&group_info.children)}
                </Paper>
            };
        }
        NavRoute::NavLink(link_info) => {
            return html! {
                <Paper class="nav-link">
                    <NavLink to={link_info.path.to_string()}>
                        <i class={&link_info.icon} />
                        <span>{link_info.name.to_string()}</span>
                    </NavLink>
                </Paper>
            };
        }
    }
}

/// Properties for NavDislay component
#[derive(Properties, PartialEq)]
pub struct DisplayNavGroupToggleProps {
    pub group_info: NavGroupInfo,
}

#[function_component(DisplayNavGroupToggle)]
fn display_nav_group_toggle(props: &DisplayNavGroupToggleProps) -> Html {
    let show_children = use_state(|| false);
    let onclick = {
        let show_children = show_children.clone();
        Callback::from(move |_| show_children.set(!*show_children))
    };
    let classes = &mut Classes::new();
    classes.push("navlink");
    if *show_children {
        classes.push("show");
    }
    return html! {
        <a {onclick} class={classes.to_string()}>
            <i class={&props.group_info.icon} />
            <span>{props.group_info.name.to_string()}</span>
            {display_caret(*show_children)}
        </a>
    };
}

fn display_caret(is_showing: bool) -> Html {
    let status_classes = &mut Classes::new();
    status_classes.push("fa-solid");
    if is_showing {
        status_classes.push("fa-caret-up");
    } else {
        status_classes.push("fa-caret-down");
    }
    jslog!("Is Showing: {} {}", is_showing, status_classes.to_string());
    let path = if is_showing {
        "M182.6 137.4c-12.5-12.5-32.8-12.5-45.3 0l-128 128c-9.2 9.2-11.9 22.9-6.9 34.9s16.6 19.8 29.6 19.8H288c12.9 0 24.6-7.8 29.6-19.8s2.2-25.7-6.9-34.9l-128-128z"
    } else {
        "M137.4 374.6c12.5 12.5 32.8 12.5 45.3 0l128-128c9.2-9.2 11.9-22.9 6.9-34.9s-16.6-19.8-29.6-19.8L32 192c-12.9 0-24.6 7.8-29.6 19.8s-2.2 25.7 6.9 34.9l128 128z"
    };
    let class = if is_showing {
        "svg-inline--fa fa-caret-up"
    } else {
        "svg-inline--fa fa-caret-down"
    };
    html! {
        <svg {class} aria-hidden="true" focusable="false" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path fill="currentColor" d={path}></path></svg>
    }
}
