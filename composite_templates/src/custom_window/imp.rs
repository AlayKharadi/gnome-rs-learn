use crate::custom_button::CustomButton;

use gtk::{
    glib::{self, subclass::InitializingObject},
    subclass::{
        prelude::{ApplicationWindowImpl, ObjectImpl, ObjectSubclass, ObjectImplExt},
        widget::{WidgetClassSubclassExt, WidgetImpl, CompositeTemplateClass, CompositeTemplateInitializingExt},
        window::WindowImpl,
    },
    ApplicationWindow, CompositeTemplate, TemplateChild, traits::ButtonExt,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/resources/window.ui")]
pub struct CustomWindow {
    #[template_child]
    pub button: TemplateChild<CustomButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomWindow {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::CustomWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CustomWindow {
    fn constructed(&self) {
        self.parent_constructed();
        self.button.connect_clicked(move |button: &CustomButton| {
            button.set_label("Hello world!");
        });
    }
}

impl WidgetImpl for CustomWindow {}

impl ApplicationWindowImpl for CustomWindow {}

impl WindowImpl for CustomWindow {}
