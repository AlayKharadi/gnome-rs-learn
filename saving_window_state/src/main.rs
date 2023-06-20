mod custom_window;

use custom_window::CustomWindow;
use gtk::{glib, prelude::*, Application};

const APP_ID: &str = "org.gtk_rs.SavingWindowState";
const APP_TITLE: &str = "Persistent Window width";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window: CustomWindow = CustomWindow::with_title(app, APP_TITLE);
    window.connect_maximized_notify(|window: &CustomWindow| {
        println!("{}", window.is_maximized())
    });
    window.present()
}
