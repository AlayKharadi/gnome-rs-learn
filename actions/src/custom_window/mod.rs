mod imp;

use gio::SimpleAction;
use glib::{clone, Object};
use gtk::{
    gio::{self, ActionGroup, ActionMap},
    glib::{self, Variant},
    prelude::{ActionExt, ActionMapExt, StaticVariantType, ToVariant},
    subclass::prelude::ObjectSubclassIsExt,
    Accessible, Application, ApplicationWindow, Buildable, ConstraintTarget, Label, Native, Root,
    ShortcutManager, Widget, Window,
};

glib::wrapper! {
    pub struct CustomWindow(ObjectSubclass<imp::CustomWindow>)
        @extends ApplicationWindow, Window, Widget,
        @implements ActionGroup, ActionMap, Accessible, Buildable,
                    ConstraintTarget, Native, Root, ShortcutManager;
}

impl CustomWindow {
    pub fn new(app: &Application) -> Self {
        // Create new CustomWindow
        Object::builder().property("application", app).build()
    }

    fn setup_actions(&self) {
        let label: Label = self.imp().label.get();

        // Add stateful action "count" to `CustomWindow` taking an integer as parameter
        let original_state: i32 = 0;
        let action_count: SimpleAction = SimpleAction::new_stateful(
            "count",
            Some(&i32::static_variant_type()),
            original_state.to_variant(),
        );

        action_count.connect_activate(
            clone!(@weak label => move |action: &SimpleAction, parameter: Option<&Variant>| {
                // Get state
                let mut state = action
                    .state()
                    .expect("Could not get state.")
                    .get::<i32>()
                    .expect("The value needs to be of type `i32`.");

                // Get parameter
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<i32>()
                    .expect("The value needs to be of type `i32`.");

                // Increase state by parameter and save state
                state += parameter;
                action.set_state(state.to_variant());

                // Update label with new state
                label.set_label(&format!("Counter: {state}"));
            }),
        );
        self.add_action(&action_count);
    }
}
