use crate::prelude::*;

/// App home page
pub(crate) fn page_components_touch(_contexts: Contexts) -> Html {
    set_title("Touch / User Input Components");
    html! {
        <>
            <PageContent />
            <NextPageButton url="/about" />
        </>
    }
}

#[function_component(PageContent)]
fn page_content() -> Html {
    let dropdown_value = use_state(|| String::from(""));
    let mut options = vec![
        DropdownOption::new("Sample 1", || html!(<span>{"Sample 1"}</span>)),
        DropdownOption::new(
            "Sample 2",
            || html!(<Icon icon="fa-duotone fa-fire-flame-simple" color={Theme::Danger} />),
        ),
    ];
    if (*dropdown_value).is_empty() {
        options.insert(0, DropdownOption::new("", || html!("Select an option!")));
    }
    html! {
        <>
            <div class={CLASSES_SIDE_BY_SIDE}>
                <Paper class="d-flex align-center justify-center flex-column gap-3">
                    <p>{format!("The currently selected dropdown value is {}", dropdown_value.deref())}</p>
                    <Dropdown selected={dropdown_value} {options} />
                </Paper>
                <Paper>
                    <MarkdownContent markdown={r#"
                        ## Dropdown
                        Component for displaying a dropdown menu that supports advanced styling of options (full HTML instead of text).
                        ```rust
                        #[function_component(Example)]
                        pub fn example() -> Html {
                            let dropdown_value = use_state(|| String::from(""));
                            let mut options = vec![
                                DropdownOption::new("Sample 1", || html!(<span>{"Sample 1"}</span>)),
                                DropdownOption::new("Sample 2", || html!(<Icon icon="fa-duotone fa-fire-flame-simple" color={Theme::Danger} />)),
                                ];
                            if (*dropdown_value).is_empty() {
                                options.insert(0, DropdownOption::new("", || html!("Select an option!")));
                            }
                            html! {
                                <Paper class="d-flex align-center justify-center flex-column gap-3">
                                    <p>{format!("The currently selected dropdown value is {}", dropdown_value.deref())}</p>
                                    <Dropdown selected={dropdown_value} {options} />
                                </Paper>
                            }
                        }
                        ```
                        "#} />
                </Paper>
            </div>
        </>
    }
}
