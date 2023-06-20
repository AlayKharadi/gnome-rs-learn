mod integer_object;

use gtk::{
    gio::ListStore,
    glib::{self, Object},
    prelude::{
        ApplicationExt, ApplicationExtManual, Cast, CastNone, GObjectPropertyExpressionExt,
        ListModelExt, StaticType,
    },
    traits::{GtkWindowExt, FilterExt, SorterExt},
    Application, ApplicationWindow, Label, ListItem, ListView, PolicyType, ScrolledWindow,
    SelectionModel, SignalListItemFactory, SingleSelection, Widget, CustomFilter, FilterListModel, CustomSorter, SortListModel, FilterChange, SorterChange,
};
use integer_object::IntegerObject;

const APP_ID: &str = "org.gtk_rs.ListWidgets";
const APP_TITLE: &str = "My GTK App";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let vector: Vec<IntegerObject> = (0..=100_000).map(IntegerObject::new).collect();
    let model: ListStore = ListStore::new(IntegerObject::static_type());
    model.extend_from_slice(&vector);

    let factory: SignalListItemFactory = SignalListItemFactory::new();

    factory.connect_setup(|_, list_item: &ListItem| {
        let label: Label = Label::new(None);
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));

        list_item
            .property_expression("item")
            .chain_property::<IntegerObject>("number")
            .bind(&label, "label", Widget::NONE);
    });

    let filter: CustomFilter = CustomFilter::new(|obj: &Object| {
        let integer_object: &IntegerObject = obj.downcast_ref::<IntegerObject>().expect("The object needs to be of type `IntegerObject`.");

        integer_object.number() % 2 == 0
    });

    let filter_model: FilterListModel = FilterListModel::new(Some(model.clone()), Some(filter.clone()));

    let sorter: CustomSorter = CustomSorter::new(move |obj1: &Object, obj2: &Object| {
        let integer_object_1: &IntegerObject = obj1.downcast_ref::<IntegerObject>().expect("The object needs to be of type `IntegerObject`.");
        let integer_object_2: &IntegerObject = obj2.downcast_ref::<IntegerObject>().expect("The object needs to be of type `IntegerObject`.");

        let number_1: i32 = integer_object_1.number();
        let number_2: i32 = integer_object_2.number();

        number_2.cmp(&number_1).into()
    });
    let sort_model: SortListModel = SortListModel::new(Some(filter_model), Some(sorter.clone()));

    let selection_model: SingleSelection = SingleSelection::new(Some(sort_model));
    let list_view: ListView = ListView::new(Some(selection_model), Some(factory));

    list_view.connect_activate(move |list_view: &ListView, position: u32| {
        let model: SelectionModel = list_view.model().expect("The model has to exist.");
        let integer_object: IntegerObject = model
            .item(position)
            .and_downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`.");

        integer_object.increase_number();

        filter.changed(FilterChange::Different);
        sorter.changed(SorterChange::Different);
    });

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
