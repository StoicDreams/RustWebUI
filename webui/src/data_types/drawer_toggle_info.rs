use crate::prelude::*;

/// Struct used for defining details for displaying buttons that toggle drawer content.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawerToggleInfo {
    pub(crate) display: fn() -> Html,
    pub(crate) title: String,
    pub(crate) class: String,
    pub(crate) content_class: String,
    pub(crate) drawer: Direction,
    pub(crate) drawer_content: DynHtml,
    pub(crate) hide_header: bool,
    pub(crate) hide_footer: bool,
    pub(crate) hide_close_x: bool,
    pub(crate) hide_cancel: bool,
    pub(crate) on_confirm: Option<fn(Contexts) -> bool>,
    pub(crate) confirm_display: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawerToggleInfoBuilder {
    display: fn() -> Html,
    title: String,
    class: String,
    content_class: String,
    drawer: Direction,
    drawer_content: DynHtml,
    hide_header: bool,
    hide_footer: bool,
    hide_close_x: bool,
    hide_cancel: bool,
    on_confirm: Option<fn(Contexts) -> bool>,
    confirm_display: String,
}

impl DrawerToggleInfo {
    pub fn new(
        title: &str,
        button_display: fn() -> Html,
        drawer_content: DynHtml,
    ) -> DrawerToggleInfoBuilder {
        DrawerToggleInfoBuilder {
            title: String::from(title),
            display: button_display,
            drawer_content,
            class: String::default(),
            content_class: String::default(),
            drawer: Direction::Bottom,
            hide_header: false,
            hide_footer: false,
            hide_close_x: false,
            hide_cancel: false,
            on_confirm: None,
            confirm_display: String::default(),
        }
    }
    pub(crate) fn get_options(&self) -> AppDrawerOptions {
        let mut builder: AppDrawerOptionsBuilder =
            AppDrawerOptions::new(self.title.to_owned(), self.drawer_content.to_owned());
        builder.set_content_class(&self.content_class);
        builder.set_drawer(self.drawer);
        if self.hide_header {
            builder.hide_header();
        } else if self.hide_close_x {
            builder.hide_close_x();
        }
        if self.hide_footer {
            builder.hide_footer();
        } else {
            if self.hide_cancel {
                builder.hide_cancel();
            }
            match self.on_confirm {
                Some(on_confirm) => {
                    builder.set_on_confirm(self.confirm_display.to_string(), on_confirm);
                }
                None => (),
            }
        }

        builder.build()
    }
}

impl DrawerToggleInfoBuilder {
    pub fn build(&mut self) -> DrawerToggleInfo {
        DrawerToggleInfo {
            display: self.display,
            title: self.title.to_string(),
            class: self.class.to_string(),
            content_class: self.content_class.to_string(),
            drawer: self.drawer.to_owned(),
            drawer_content: self.drawer_content.to_owned(),
            hide_header: self.hide_header,
            hide_footer: self.hide_footer,
            hide_close_x: self.hide_close_x,
            hide_cancel: self.hide_cancel,
            on_confirm: self.on_confirm,
            confirm_display: self.confirm_display.to_string(),
        }
    }
    pub fn set_content_class(&mut self, class: &str) -> &Self {
        self.content_class = String::from(class);
        self
    }
    pub fn set_button_class(&mut self, class: &str) -> &mut Self {
        self.class = String::from(class);
        self
    }
    pub fn set_drawer(&mut self, drawer: Direction) -> &mut Self {
        self.drawer = drawer;
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

    pub fn hide_cancel_button(&mut self) -> &mut Self {
        self.hide_cancel = true;
        self
    }

    pub fn hide_close_x_button(&mut self) -> &mut Self {
        self.hide_close_x = true;
        self
    }

    /// Set the confirmation display text and handler handler
    ///
    /// This button will display on the right side of the drawer footer
    pub fn set_on_confirm(&mut self, display: &str, on_confirm: fn(Contexts) -> bool) -> &mut Self {
        self.on_confirm = Some(on_confirm);
        self.confirm_display = String::from(display);
        self
    }
}
