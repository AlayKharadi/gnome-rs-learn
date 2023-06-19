use std::{time::Duration, thread};
use gtk::{
    glib::{self},
    prelude::*,
    Application, ApplicationWindow, Button
};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";
const APP_TITLE: &str = "Title";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button: Button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|_| {
        thread::spawn(move || {
            let five_seconds = Duration::from_secs(5);
            std::thread::sleep(five_seconds);
        });
    });

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&button)
        .build();

    window.present()
}
