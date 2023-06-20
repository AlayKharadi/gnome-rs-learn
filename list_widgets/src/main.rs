mod integer_object;

use gtk::{
    glib,
    prelude::{ApplicationExt, ApplicationExtManual, Cast, GObjectPropertyExpressionExt},
    traits::GtkWindowExt,
    Application, ApplicationWindow, Label, ListItem, ListView, NoSelection, PolicyType,
    ScrolledWindow, SignalListItemFactory, StringList, Widget, StringObject,
};

const APP_ID: &str = "org.gtk_rs.ListWidgets";
const APP_TITLE: &str = "My GTK App";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let model: StringList = (0..=100_000)
        .into_iter()
        .map(|number: i32| number.to_string())
        .collect();

    let factory: SignalListItemFactory = SignalListItemFactory::new();

    factory.connect_setup(|_, list_item: &ListItem| {
        let label: Label = Label::new(None);
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));

        list_item
            .property_expression("item")
            .chain_property::<StringObject>("string")
            .bind(&label, "label", Widget::NONE);
    });

    let selection_model: NoSelection = NoSelection::new(Some(model));
    let list_view: ListView = ListView::new(Some(selection_model), Some(factory));

    let scrolled_view: ScrolledWindow = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_height(360)
        .child(&list_view)
        .build();

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .default_width(600)
        .default_height(360)
        .child(&scrolled_view)
        .build();

    window.present();
}
