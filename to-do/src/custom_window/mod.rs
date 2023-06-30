use gtk::{
    gio::{ActionGroup, ActionMap, ListStore},
    glib::{self, clone, Object},
    prelude::{Cast, CastNone, EntryBufferExtManual, StaticType},
    subclass::prelude::ObjectSubclassIsExt,
    traits::EntryExt,
    Accessible, Application, ApplicationWindow, Buildable, ConstraintTarget, EntryBuffer, ListItem,
    Native, NoSelection, Root, ShortcutManager, SignalListItemFactory, Widget, Window,
};

use crate::{task_object::TaskObject, task_row::TaskRow};

mod imp;

glib::wrapper! {
    pub struct CustomWindow(ObjectSubclass<imp::CustomWindow>)
    @extends ApplicationWindow, Window, Widget,
    @implements ActionGroup, ActionMap, Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;
}

impl CustomWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn tasks(&self) -> ListStore {
        // Get state
        self.imp()
            .tasks
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn setup_tasks(&self) {
        // Create new model
        let model: ListStore = ListStore::new(TaskObject::static_type());

        // Get state and set model
        self.imp().tasks.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.tasks()));
        self.imp().tasks_list.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        // Setup callback for activation of the entry
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_task();
            }));

        // Setup callback for clicking (and the releasing) the icon of the entry
        self.imp()
            .entry
            .connect_icon_release(clone!(@weak self as window => move |_,_| {
                window.new_task();
            }));
    }

    fn new_task(&self) {
        // Get content from entry and clear it
        let buffer: EntryBuffer = self.imp().entry.buffer();
        let content: String = buffer.text().to_string();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");

        // Add new task to model
        let task = TaskObject::new(false, content);
        self.tasks().append(&task);
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory: SignalListItemFactory = SignalListItemFactory::new();

        // Create an empty `TaskRow` during setup
        factory.connect_setup(move |_, list_item: &ListItem| {
            // Create `TaskRow`
            let task_row: TaskRow = TaskRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&task_row));
        });

        // Tell factory how to bind `TaskRow` to a `TaskObject`
        factory.connect_bind(move |_, list_item: &ListItem| {
            // Get `TaskObject` from `ListItem`
            let task_object: TaskObject = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<TaskObject>()
                .expect("The item has to be an `TaskObject`.");

            // Get `TaskRow` from `ListItem`
            let task_row: TaskRow = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.bind(&task_object);
        });

        // Tell factory how to unbind `TaskRow` from `TaskObject`
        factory.connect_unbind(move |_, list_item: &ListItem| {
            // Get `TaskRow` from `ListItem`
            let task_row: TaskRow = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.unbind();
        });

        // Set the factory of the list view
        self.imp().tasks_list.set_factory(Some(&factory));
    }
}
