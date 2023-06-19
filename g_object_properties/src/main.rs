use gtk::{glib, prelude::*, Align, Application, ApplicationWindow, Box, Orientation, Switch};

const APP_ID: &str = "org.gtk_rs.GObjectProperties1";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let switch_1: Switch = Switch::new();
    let switch_2: Switch = Switch::new();

    switch_1
        .bind_property("active", &switch_2, "active")
        .bidirectional()
        .build();

    let gtk_box: Box = Box::builder()
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&switch_1);
    gtk_box.append(&switch_2);

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(400)
        .default_height(400)
        .child(&gtk_box)
        .build();

    window.present()
}
