use gtk::{
    glib::{self, subclass::InitializingObject, once_cell::sync::OnceCell},
    subclass::{
        prelude::{
            ApplicationWindowImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
        },
        widget::{
            CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassSubclassExt,
            WidgetImpl,
        },
        window::WindowImpl,
    },
    ApplicationWindow, Box, CompositeTemplate, Label, TemplateChild, Button, gio::Settings,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/resources/window.ui")]
pub struct CustomWindow {
    #[template_child]
    pub gtk_box: TemplateChild<Box>,
    #[template_child]
    pub button: TemplateChild<Button>,
    #[template_child]
    pub label: TemplateChild<Label>,
    pub settings: OnceCell<Settings>,
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
        let obj = self.obj();
        obj.setup_settings();
        obj.setup_actions();
        obj.bind_settings();
    }
}

impl WidgetImpl for CustomWindow {}

impl WindowImpl for CustomWindow {}

impl ApplicationWindowImpl for CustomWindow {}
