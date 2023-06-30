use std::cell::RefCell;

use gtk::{
    glib::{self, subclass::InitializingObject},
    subclass::{
        prelude::{ApplicationWindowImpl, ObjectImpl, ObjectSubclass, ObjectImplExt, ObjectSubclassExt},
        widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl, WidgetClassSubclassExt},
        window::WindowImpl,
    },
    ApplicationWindow, CompositeTemplate, TemplateChild, Entry, ListView, gio::ListStore,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/Todo/window.ui")]
pub struct CustomWindow{
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListView>,
    pub tasks: RefCell<Option<ListStore>>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomWindow {
    const NAME: &'static str = "TodoWindow";
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
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let obj = self.obj();
        obj.setup_tasks();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}

impl WidgetImpl for CustomWindow {}

impl WindowImpl for CustomWindow {}

impl ApplicationWindowImpl for CustomWindow {}
