use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppDrawerOptions {
    pub(crate) drawer: Direction,
    pub(crate) title: String,
    pub(crate) display_ref: DynHtml,
    pub(crate) hide_header: bool,
    pub(crate) hide_footer: bool,
    pub(crate) hide_close_x: bool,
    pub(crate) hide_cancel: bool,
    pub(crate) on_confirm: Option<usize>,
    pub(crate) confirm_display: String,
    pub(crate) content_class: String,
}
pub struct AppDrawerOptionsBuilder {
    drawer: Direction,
    title: String,
    display_ref: DynHtml,
    hide_header: bool,
    hide_footer: bool,
    hide_close_x: bool,
    hide_cancel: bool,
    on_confirm: Option<fn(Contexts) -> bool>,
    confirm_display: String,
    content_class: String,
}

impl AppDrawerOptionsBuilder {
    pub fn build(self) -> AppDrawerOptions {
        AppDrawerOptions {
            drawer: self.drawer,
            title: self.title,
            display_ref: self.display_ref,
            hide_header: self.hide_header,
            hide_footer: self.hide_footer,
            hide_close_x: self.hide_close_x,
            hide_cancel: self.hide_cancel,
            on_confirm: self.on_confirm.map(|method| method as usize),
            confirm_display: {
                let confirm_display = &self.confirm_display;
                confirm_display.to_owned()
            },
            content_class: self.content_class,
        }
    }
    pub fn set_drawer(&mut self, drawer: Direction) -> &mut Self {
        self.drawer = drawer;
        self
    }
    pub fn hide_close_x(&mut self) -> &mut Self {
        self.hide_close_x = true;
        self
    }
    pub fn hide_header(&mut self) -> &mut Self {
        self.hide_header = true;
        self
    }
    pub fn hide_footer(&mut self) -> &mut Self {
        self.hide_footer = true;
        self
    }
    pub(crate) fn hide_cancel(&mut self) -> &mut Self {
        self.hide_cancel = true;
        self
    }
    pub fn set_on_confirm(
        &mut self,
        display: String,
        on_confirm: fn(Contexts) -> bool,
    ) -> &mut Self {
        self.on_confirm = Some(on_confirm);
        self.confirm_display = display;
        self
    }
    pub fn set_content_class(&mut self, class: &str) -> &mut Self {
        self.content_class = String::from(class);
        self
    }
}

impl AppDrawerOptions {
    pub fn builder(title: String, display: DynHtml) -> AppDrawerOptionsBuilder {
        AppDrawerOptionsBuilder {
            drawer: Direction::Top,
            title,
            display_ref: display,
            hide_header: false,
            hide_footer: false,
            hide_close_x: false,
            hide_cancel: false,
            confirm_display: "Confirm".to_string(),
            on_confirm: None,
            content_class: String::default(),
        }
    }

    pub(crate) fn get_display(&self) -> DynHtml {
        self.display_ref.to_owned()
    }

    pub(crate) fn get_on_confirm(&self) -> fn(Contexts) -> bool {
        match self.on_confirm {
            Some(value) => {
                let content: fn(Contexts) -> bool = if value > 0 {
                    let fnptr = value as *const ();
                    unsafe { std::mem::transmute(fnptr) }
                } else {
                    |_| true
                };
                content
            }
            None => |_| true,
        }
    }
}

/// Properties for app drawer components
#[derive(Properties, PartialEq)]
pub(crate) struct AppDrawerProps {
    #[prop_or_default]
    pub class: Option<String>,
    pub drawer: Direction,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub(crate) struct AppDrawerState {
    pub is_open: bool,
    pub content: Option<AppDrawerOptions>,
}

const TRANSITION_DURATION: i32 = 300;

#[function_component(AppDrawer)]
pub(crate) fn app_drawer(props: &AppDrawerProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let is_open_handle = use_state(|| false);
    let is_transition_handle = use_state(|| false);
    let content_handle: UseStateHandle<Option<AppDrawerOptions>> = use_state(|| None);
    match contexts.drawer.deref().to_owned() {
        DrawerMessage::ToggleDrawer(option) => {
            if props.drawer == option.drawer {
                contexts.drawer.set(DrawerMessage::None);
                let is_open = is_open_handle.deref().to_owned();
                let is_transition = is_transition_handle.deref().to_owned();
                if !is_transition {
                    is_transition_handle.set(true);
                    let is_open = !is_open;
                    if is_open {
                        content_handle.set(Some(option));
                    }
                    is_open_handle.set(is_open);
                    let is_transition_handle = is_transition_handle.clone();
                    let content_handle = content_handle.clone();
                    set_timeout!(TRANSITION_DURATION, move || {
                        is_transition_handle.set(false);
                        if !is_open {
                            content_handle.set(None);
                        }
                    });
                }
            }
        }
        DrawerMessage::Close => {
            let is_open = is_open_handle.deref().to_owned();
            if is_open {
                is_open_handle.set(false);
                is_transition_handle.set(true);
                let is_transition_handle = is_transition_handle.clone();
                let content_handle = content_handle.clone();
                set_timeout!(TRANSITION_DURATION, move || {
                    is_transition_handle.set(false);
                    content_handle.set(None);
                });
            }
        }
        DrawerMessage::None => (),
    }
    match content_handle.deref().to_owned() {
        Some(content) => {
            let is_open = is_open_handle.deref().to_owned();
            let is_transition = is_transition_handle.deref().to_owned();
            if !is_open && !is_transition {
                return html! {
                    <></>
                };
            }
            let class = format!(
                "app-drawer {} {} {}",
                props.drawer,
                props.class.to_owned().unwrap_or_default(),
                if is_open { "open" } else { "closed" }
            );
            let content_class = format!("drawer-content elevation-20 {}", content.content_class);
            let drawer_body = content.get_display();
            let show_header = !content.hide_header;
            let show_footer = !content.hide_footer;
            let show_close_x = !content.hide_close_x;
            let show_close = !content.hide_cancel;
            let cancel_button_display = "Cancel";
            let show_confirm = content.on_confirm.is_some();
            let confirm_display = content.confirm_display.to_owned();
            let on_confirm_onclick = content.get_on_confirm();
            let drawer_context = contexts.drawer.clone();
            let content_close = content.clone();
            let handle_close = Callback::from(move |_| {
                let content_close = content_close.to_owned();
                drawer_context.set(DrawerMessage::ToggleDrawer(content_close));
            });

            let cover_click = handle_close.to_owned();
            let close_x_click = handle_close.to_owned();
            let close_click = handle_close.to_owned();
            let contexts_click = contexts.clone();
            let confirm_click = Callback::from(move |ev| {
                on_confirm_onclick(contexts_click.to_owned());
                handle_close.emit(ev);
            });

            let title = content.title;
            html! {
                <aside class={class}>
                    <div class="drawer-placement">
                        <div class="page-cover" onclick={cover_click}>
                        </div>
                        <div class={content_class}>
                            {if show_header {
                                html! {
                                    <header class="pl-2 pr-2">
                                        {title_standard!(
                                            html!{
                                                <>
                                                    <Paper>{title}</Paper>
                                                    <span class="flex-grow" />
                                                </>
                                            }
                                        )}
                                        <span class="flex-grow" />
                                        {if show_close_x {
                                            html! {
                                                <Button title="close" class="btn theme-danger mr-1 pt-1 bt-1 pl-3 pr-3" onclick={close_x_click}>
                                                    <i class="fa-solid fa-times" />
                                                </Button>
                                            }
                                        } else {html!{}}}
                                    </header>
                                }
                            }else{html!{}}}
                            <Paper class="flex-grow d-flex flex-column gap-1 overflow-auto pa-2">
                                {drawer_body.run()}
                            </Paper>
                            {if show_footer {
                                html! {
                                    <footer class="pa-2 d-flex flex-row">
                                        {if show_close {
                                            html! {
                                                <Button title="cancel" class="btn theme-warning" onclick={close_click}>
                                                    {cancel_button_display}
                                                </Button>
                                            }
                                        } else {empty_html(contexts.clone())}}
                                        {if show_confirm {
                                            html! {
                                                <>
                                                    <span class="flex-grow" />
                                                    <Button class="btn theme-success" onclick={confirm_click}>
                                                        {confirm_display}
                                                    </Button>
                                                </>
                                            }
                                        } else {empty_html(contexts.clone())}}
                                    </footer>
                                }
                            } else { html! {} }}
                        </div>
                    </div>
                </aside>
            }
        }
        None => {
            html!()
        }
    }
}
