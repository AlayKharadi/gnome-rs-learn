use std::cell::Cell;

use gtk::{
    glib::{self, Properties, ParamSpec, Value},
    prelude::ObjectExt,
    subclass::prelude::{ObjectImpl, ObjectSubclass, DerivedObjectProperties},
};

#[derive(Default, Properties)]
#[properties(wrapper_type=super::IntegerObject)]
pub struct IntegerObject {
    #[property(get, set)]
    number: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for IntegerObject {
    const NAME: &'static str = "MyGtkAppIntegerObject";
    type Type = super::IntegerObject;
}

impl ObjectImpl for IntegerObject {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec);
    }
}
