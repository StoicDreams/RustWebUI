use crate::{function_component, html, Children, Classes, Properties};

/// Properties for Paper component
#[derive(Properties, PartialEq)]
pub struct PaperProps {
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
}

/// Common container component
///
/// Basic example
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<Paper>{"Your child content here"}</Paper>
/// 	}
/// }
/// ```
///
/// Add classes
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<Paper class="d-flex flex-column">{"Your child content here"}</Paper>
/// 	}
/// }
/// ```
///
/// Apply elevetation
///
/// Elevation applies a box shadow to the Paper component.
/// Valid ranges range from 0 ro 25.
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<Paper elevation={10}>{"Your child content here"}</Paper>
/// 	}
/// }
/// ```
#[function_component(Paper)]
pub fn paper(props: &PaperProps) -> Html {
    let classes = &mut Classes::new();
    classes.push("paper");

    if props.elevation > 0 {
        classes.push(format!("elevation-{}", props.elevation));
    }

    if !props.class.is_empty() {
        classes.push(&props.class);
    }

    let class = classes.clone();
    let style = props.style.to_owned();

    html! {
        <section {class} {style}>
            { for props.children.iter() }
        </section>
    }
}
