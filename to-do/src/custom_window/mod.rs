use std::fs::File;

use gtk::{
    gio::{ Action, ActionGroup, ActionMap, ListStore, Settings, SimpleAction },
    glib::{self, clone, Object},
    prelude::{
        ActionMapExt, Cast, CastNone, EntryBufferExtManual, ListModelExt, SettingsExt, StaticType, SettingsExtManual,
    },
    subclass::prelude::ObjectSubclassIsExt,
    traits::EntryExt,
    Accessible, Application, ApplicationWindow, Buildable, ConstraintTarget, EntryBuffer, ListItem,
    Native, NoSelection, Root, ShortcutManager, SignalListItemFactory, Widget, Window, CustomFilter, FilterListModel,
};

use crate::{
    task_object::{TaskData, TaskObject},
    task_row::TaskRow,
    utils::data_path,
    APP_ID,
};

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

    fn setup_settings(&self) {
        let settings: Settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    fn setup_actions(&self) {
        // Create action from key "filter" and add to action group "win"
        let action_filter: Action = self.settings().create_action("filter");
        self.add_action(&action_filter);

        // Create action to remove done tasks and add to action group "win"
        let action_remove_done_tasks: SimpleAction = SimpleAction::new("remove-done-tasks", None);
        action_remove_done_tasks.connect_activate(clone!(@weak self as window => move |_, _| {
            let tasks = window.tasks();
            let mut position = 0;
            while let Some(item) = tasks.item(position) {
                // Get `TaskObject` from `glib::Object`
                let task_object = item
                    .downcast_ref::<TaskObject>()
                    .expect("The object needs to be of type `TaskObject`.");

                if task_object.is_completed() {
                    tasks.remove(position);
                } else {
                    position += 1;
                }
            }
        }));
        self.add_action(&action_remove_done_tasks);
    }

    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            // Deserialize data from file to vector
            let backup_data: Vec<TaskData> = serde_json::from_reader(file)
                .expect("It should be possible to read `backup_data` from the json file.");

            // Convert `Vec<TaskData>` to `Vec<TaskObject>`
            let task_objects: Vec<TaskObject> = backup_data
                .into_iter()
                .map(TaskObject::from_task_data)
                .collect();

            // Insert restored objects into model
            self.tasks().extend_from_slice(&task_objects);
        }
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
        let filter_model: FilterListModel = FilterListModel::new(Some(self.tasks()), self.filter());
        let selection_model: NoSelection = NoSelection::new(Some(filter_model.clone()));
        self.imp().tasks_list.set_model(Some(&selection_model));

        // Filter model whenever the value of the key "filter" changes
        self.settings().connect_changed(
            Some("filter"),
            clone!(@weak self as window, @weak filter_model => move |_, _| {
                filter_model.set_filter(window.filter().as_ref());
            }),
        );
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

    fn remove_done_tasks(&self) {
        let tasks = self.tasks();
        let mut position = 0;
        while let Some(item) = tasks.item(position) {
            // Get `TaskObject` from `glib::Object`
            let task_object = item
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            if task_object.is_completed() {
                tasks.remove(position);
            } else {
                position += 1;
            }
        }
    }

    fn filter(&self) -> Option<CustomFilter> {
        // Get state

        // Get filter_state from settings
        let filter_state: String = self.settings().get("filter");

        // Create custom filters
        let filter_open: CustomFilter = CustomFilter::new(|obj: &Object| {
            // Get `TaskObject` from `glib::Object`
            let task_object: &TaskObject = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow completed tasks
            !task_object.is_completed()
        });
        let filter_done: CustomFilter = CustomFilter::new(|obj: &Object| {
            // Get `TaskObject` from `glib::Object`
            let task_object: &TaskObject = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow done tasks
            task_object.is_completed()
        });

        // Return the correct filter
        match filter_state.as_str() {
            "All" => None,
            "Open" => Some(filter_open),
            "Done" => Some(filter_done),
            _ => unreachable!(),
        }
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
