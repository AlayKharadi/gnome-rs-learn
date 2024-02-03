mod custom_window;
mod task_object;
mod task_row;
mod utils;

use custom_window::CustomWindow;

use gtk::{
    gio,
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::{GtkApplicationExt, GtkWindowExt},
    Application,
};

const APP_ID: &str = "org.gtk_rs.Todo";

fn main() -> ExitCode {
    gio::resources_register_include!("todo.gresource").expect("Failed to register resources.");

    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_startup(setup_shortcuts);

    app.connect_activate(build_ui);

    app.run()
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &Application) {
    let window: CustomWindow = CustomWindow::new(app);
    window.present();
}
