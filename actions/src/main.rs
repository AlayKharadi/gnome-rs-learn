mod custom_window;

use custom_window::CustomWindow;
use gtk::{
    gio::resources_register_include,
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::{GtkWindowExt, WidgetExt},
    Application,
};

const APP_ID: &str = "org.gtk_rs.Actions";
const APP_TITLE: &str = "My GTK App";

fn main() -> ExitCode {
    resources_register_include!("gtk_actions.gresource").expect("Failed to register resources.");

    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window: CustomWindow = CustomWindow::new(app);
    window.set_title(Some(APP_TITLE));
    window.set_width_request(360);

    // Present window
    window.present();
}
