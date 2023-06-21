use std::cell::Cell;

use crate::custom_button::CustomButton;

use gtk::{
    glib::{self, object_subclass, subclass::InitializingObject},
    subclass::{
        prelude::{ApplicationWindowImpl, ObjectImpl, ObjectSubclass},
        widget::{
            CompositeTemplateCallbacksClass, CompositeTemplateClass,
            CompositeTemplateInitializingExt, WidgetImpl,
        },
        window::WindowImpl,
    },
    template_callbacks,
    traits::ButtonExt,
    ApplicationWindow, CompositeTemplate, prelude::StaticTypeExt,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/resources/window.ui")]
pub struct CustomWindow {
    pub number: Cell<i32>,
}

#[object_subclass]
impl ObjectSubclass for CustomWindow {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::CustomWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        CustomButton::ensure_type();

        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[template_callbacks]
impl CustomWindow {
    #[template_callback]
    fn handle_button_clicked(&self, button: &CustomButton) {
        let number_increased: i32 = self.number.get() + 1;
        self.number.set(number_increased);
        button.set_label(&number_increased.to_string());
    }
}

impl ObjectImpl for CustomWindow {}

impl WidgetImpl for CustomWindow {}

impl ApplicationWindowImpl for CustomWindow {}

impl WindowImpl for CustomWindow {}
