use std::cell::Cell;

use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::ObjectExt,
    subclass::{
        prelude::{
            ButtonImpl, DerivedObjectProperties, ObjectImpl, ObjectImplExt, ObjectSubclass,
            ObjectSubclassExt,
        },
        widget::WidgetImpl,
    },
};

// Object holding the state
#[derive(Default, Properties)]
#[properties(wrapper_type=super::CustomButton)]
pub struct CustomButton {
    #[property(get, set)]
    number: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj
            .bind_property("number", obj.as_ref(), "label")
            .sync_create()
            .build();
    }

    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }
}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        self.obj().set_number(self.obj().number() + 1);
    }
}
