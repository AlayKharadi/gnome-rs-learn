mod custom_window;
mod custom_button;

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
    window.set_default_width(400);
    window.set_default_height(50);
    window.present();
}
