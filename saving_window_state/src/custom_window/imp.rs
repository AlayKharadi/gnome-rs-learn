use gtk::{
    gio::Settings,
    glib,
    subclass::prelude::{
        ApplicationWindowImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
        WidgetImpl, WindowImpl,
    },
    ApplicationWindow, Inhibit,
};

use once_cell::sync::OnceCell;

#[derive(Default)]
pub struct CustomWindow {
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomWindow {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::CustomWindow;
    type ParentType = ApplicationWindow;
}

impl ObjectImpl for CustomWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

impl WidgetImpl for CustomWindow {}

impl WindowImpl for CustomWindow {
    fn close_request(&self) -> Inhibit {
        self.obj()
            .save_window_size()
            .expect("Failed to save window state.");
        Inhibit(false)
    }
}

impl ApplicationWindowImpl for CustomWindow {}
