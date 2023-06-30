mod custom_window;
mod task_object;
mod task_row;

use custom_window::CustomWindow;

use gtk::{
    gio,
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::GtkWindowExt,
    Application,
};


const APP_ID: &str = "org.gtk_rs.ToDo";

fn main() -> ExitCode {
    gio::resources_register_include!("todo.gresource").expect("Failed to register resources.");

    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window: CustomWindow = CustomWindow::new(app);
    window.present();
}
