use gtk::{
    glib::{self, Object},
    Accessible, Actionable, Buildable, Button, ConstraintTarget, Widget,
};

mod imp;

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends Button, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget;
}

impl CustomButton {
    pub fn new() -> Self {
        Object::builder().build()
    }
}