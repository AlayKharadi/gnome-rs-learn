mod imp;

use gtk::{glib::{self, Object}, ApplicationWindow, Widget, gio::{ActionGroup, ActionMap}, Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager, Application, Window};

glib::wrapper!{
    pub struct CustomWindow(ObjectSubclass<imp::CustomWindow>)
        @extends ApplicationWindow, Window, Widget,
        @implements ActionGroup, ActionMap, Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;
}

impl CustomWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }
}