mod custom_window;

use gtk::{
    gio,
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::GtkWindowExt,
    Application,
};

use custom_window::CustomWindow;

const APP_ID: &str = "org.gtk_rs.CompositeTemplates";

fn main() -> ExitCode {
    gio::resources_register_include!("composite_templates.gresource")
        .expect("Failed to register resources.");

    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window: CustomWindow = CustomWindow::new(app);
    window.present();
}
