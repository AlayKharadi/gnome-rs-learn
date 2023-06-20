use gtk::{
    glib,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::GtkWindowExt,
    Application, ApplicationWindow, Label, ListBox, PolicyType, ScrolledWindow,
};

const APP_ID: &str = "org.gtk_rs.ListWidgets";
const APP_TITLE: &str = "My GTK App";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let list_box: ListBox = ListBox::new();

    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }

    let scrolled_view: ScrolledWindow = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_height(360)
        .child(&list_box)
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
