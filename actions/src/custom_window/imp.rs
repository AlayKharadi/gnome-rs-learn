use gtk::{
    glib::{self, subclass::InitializingObject},
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
    ApplicationWindow, CompositeTemplate, Label, TemplateChild,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/resources/window.ui")]
pub struct CustomWindow {
    #[template_child]
    pub label: TemplateChild<Label>,
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
        self.obj().setup_actions();
    }
}

impl WidgetImpl for CustomWindow {}

impl WindowImpl for CustomWindow {}

impl ApplicationWindowImpl for CustomWindow {}
