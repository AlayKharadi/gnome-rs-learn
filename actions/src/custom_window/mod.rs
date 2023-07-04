mod imp;

use gio::SimpleAction;
use glib::{clone, Object};
use gtk::{
    gio::{self, ActionGroup, ActionMap, PropertyAction},
    glib::{self, Variant},
    prelude::{ActionExt, ActionMapExt, StaticVariantType, ToVariant},
    subclass::prelude::ObjectSubclassIsExt,
    Accessible, Application, ApplicationWindow, Box, Buildable, ConstraintTarget, Label, Native, Root,
    ShortcutManager, Widget, Window, Orientation, Button, traits::OrientableExt,
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

        // Add property action "button-frame" to `window`
        let button: Button = self.imp().button.get();
        let action_button_frame: PropertyAction =
            PropertyAction::new("button-frame", &button, "has-frame");
        self.add_action(&action_button_frame);

        // Add stateful action "orientation" to `window` taking a string as parameter
        let gtk_box: Box = self.imp().gtk_box.get();
        let action_orientation = SimpleAction::new_stateful(
            "orientation",
            Some(&String::static_variant_type()),
            "Vertical".to_variant(),
        );

        action_orientation.connect_activate(clone!(@weak gtk_box =>
            move |action, parameter| {
                // Get parameter
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<String>()
                    .expect("The value needs to be of type `String`.");

                let orientation = match parameter.as_str() {
                    "Horizontal" => Orientation::Horizontal,
                    "Vertical" => Orientation::Vertical,
                    _ => unreachable!()
                };

                // Set orientation and save state
                gtk_box.set_orientation(orientation);
                action.set_state(parameter.to_variant());
        }));
        self.add_action(&action_orientation);
    }
}
