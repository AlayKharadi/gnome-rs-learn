mod imp;

use gtk::{
    gio::{ActionGroup, ActionMap, Settings},
    glib::{self, Object},
    prelude::{GtkWindowExt, SettingsExt},
    subclass::prelude::ObjectSubclassIsExt,
    Accessible, Application, ApplicationWindow, Buildable, ConstraintTarget, Native, Root,
    ShortcutManager, Widget, Window,
};

use crate::APP_ID;

glib::wrapper! {
    pub struct CustomWindow(ObjectSubclass<imp::CustomWindow>)
        @extends ApplicationWindow, Window, Widget,
        @implements ActionGroup, ActionMap, Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;
}

impl CustomWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn with_title(app: &Application, title: &str) -> Self {
        Object::builder()
            .property("application", app)
            .property("title", title)
            .build()
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_Settings`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let size: (i32, i32) = self.default_size();
        self.settings().set_int("window-width", size.0)?;
        self.settings().set_int("window-height", size.1)?;
        self.settings()
            .set_boolean("is-maximized", self.is_maximized())?;
        Ok(())
    }

    fn load_window_size(&self) {
        let width: i32 = self.settings().int("window-width");
        let height: i32 = self.settings().int("window-height");
        let is_maximized: bool = self.settings().boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}
