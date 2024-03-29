use crate::prelude::*;
use yew::Html;

/// Properties for Paper component
#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub width: u16,
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub avatar: Option<String>,
    #[prop_or_default]
    pub link: Option<String>,
    #[prop_or_default]
    pub link_title: String,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub header: Option<fn() -> Html>,
    #[prop_or_default]
    pub footer: Option<fn() -> Html>,
    #[prop_or(Theme::Title)]
    pub theme: Theme,
}

/// Card component
///
/// Display a group of content with a header, body, and optional footer content.
/// Header content generally consists of an Avatar (icon or image), a title, and a link or button for some context related action.
///
/// Note: avatar string is expected to start with "fa-" if using an icon, otherwise string is treated as an image source value.
///
/// Basic example
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: &Contexts) -> Html {
///     html! {
///         <Card title="Hello World" avatar="fa-solid fa-acorn">{"Your card body content here"}</Card>
///     }
/// }
/// ```
///
/// Apply theme to card header
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: &Contexts) -> Html {
///     html! {
///         <Card title="Hello World" theme={Theme::Primary}>{"Your card body content here"}</Card>
///     }
/// }
/// ```
///
/// Add classes - applied to outer Paper component
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: &Contexts) -> Html {
///     html! {
///         <Card class="d-flex flex-column">{"Your card body content here"}</Card>
///     }
/// }
/// ```
///
/// Apply elevetation
///
/// Elevation applies a box shadow to the Card component.
/// Valid ranges range from 0 ro 25.
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: &Contexts) -> Html {
///     html! {
///         <Card elevation={10}>{"Your card body content here"}</Card>
///     }
/// }
/// ```
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("card");

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    let header_classes = &mut Classes::new();
    header_classes.push("card-header d-flex flex-row flex-gap align-center");
    header_classes.push(props.theme.to_string());

    let mut styles: Vec<String> = Vec::new();
    if props.width > 0 {
        styles.push(format!(
            "max-width:{}px;min-width:{}px;",
            props.width,
            props.width / 2
        ));
    }
    if !props.style.is_empty() {
        styles.push(props.style.to_string());
    }

    html! {
        <Paper class={classes.to_string()} style={styles.join(" ")}>
            <Paper class={header_classes.to_string()}>
                {match props.avatar.to_owned() {
                    Some(avatar) => {
                        if avatar.is_empty() {
                            html!()
                        } else if avatar.starts_with("fa-") {
                            html! {
                                <Avatar class="f3 ml-1 pa-1" icon={avatar} />
                            }
                        } else {
                            html! {
                                <Avatar class="ml-1 pa-1" image={avatar} />
                            }
                        }
                    },
                    None => html! {}
                }}
                <Paper class="card-title d-flex flex-column flex-grow flex-gap">
                    {match props.title.to_owned() {
                        Some(title) => {
                            html! {
                                <h2 class="f3 pa-1 d-flex flex-wrap flex-row elevation-0">{title}</h2>
                            }
                        },
                        None => html! {}
                    }}
                    {match props.header.to_owned() {
                        Some(header) => {
                            html! {
                                {header()}
                            }
                        },
                        None => html! {}
                    }}
                </Paper>
                {match props.link.to_owned() {
                    Some(link) => {
                        if !link.is_empty() {
                            html!(
                                <Link href={link} class="f3 pr-3" title={props.link_title.to_owned()}>
                                    <i class="fa-solid fa-external-link" />
                                </Link>
                            )
                        } else {
                            html!()
                        }
                    },
                    None => html!{}
                }}
            </Paper>
            <Paper class="card-body d-flex flex-column flex-gap pa-1">
                { for props.children.iter() }
            </Paper>
            {match props.footer.to_owned() {
                Some(footer) => {
                    html! {
                        <Paper class="card-footer">
                            {footer()}
                        </Paper>
                    }
                },
                None => html! {}
            }}
        </Paper>
    }
}

/// Properties for Paper component
#[derive(Properties, PartialEq)]
pub struct CardsProps {
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
}

#[function_component(Cards)]
pub fn cards(props: &CardsProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("cards page-segment-cards");

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }
    let style = props.style.to_owned();

    html! {
        <Paper class={classes.to_string()} {style}>
            { for props.children.iter() }
        </Paper>
    }
}
