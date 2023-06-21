use gtk::{
    glib,
    subclass::{
        prelude::{ButtonImpl, ObjectImpl, ObjectSubclass},
        widget::WidgetImpl,
    },
    Button,
};

#[derive(Default)]
pub struct CustomButton;

#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";

    type Type = super::CustomButton;

    type ParentType = Button;
}

impl ObjectImpl for CustomButton {}

impl WidgetImpl for CustomButton {}

impl ButtonImpl for CustomButton {}
