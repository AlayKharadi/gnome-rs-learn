use std::{cell::RefCell, fs::File};

use gtk::{
    gio::{ListStore, Settings},
    glib::{self, once_cell::sync::OnceCell, subclass::InitializingObject},
    prelude::{Cast, ListModelExtManual},
    subclass::{
        prelude::{
            ApplicationWindowImpl, ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt,
        },
        widget::{
            CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassSubclassExt,
            WidgetImpl,
        },
        window::{WindowImpl, WindowImplExt},
    },
    ApplicationWindow, CompositeTemplate, Entry, Inhibit, ListView, TemplateChild,
};

use crate::{
    task_object::{TaskData, TaskObject},
    utils::data_path,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/Todo/window.ui")]
pub struct CustomWindow {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListView>,
    pub tasks: RefCell<Option<ListStore>>,
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomWindow {
    const NAME: &'static str = "TodoWindow";
    type Type = super::CustomWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        // Create action to remove done tasks and add to action group "win"
        klass.install_action("win.remove-done-tasks", None, |window: &super::CustomWindow, _, _| {
            window.remove_done_tasks();
        });
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
        obj.setup_settings();
        obj.setup_tasks();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_factory();
        obj.setup_actions();
    }
}

impl WidgetImpl for CustomWindow {}

impl WindowImpl for CustomWindow {
    fn close_request(&self) -> Inhibit {
        // Store task data in vector
        let backup_data: Vec<TaskData> = self
            .obj()
            .tasks()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<TaskObject>)
            .map(TaskObject::task_data)
            .collect();

        // Save state to file
        let file: File = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data).expect("Could not write data to json file");

        // Pass close request on to the paglibrent
        self.parent_close_request()
    }
}

impl ApplicationWindowImpl for CustomWindow {}
