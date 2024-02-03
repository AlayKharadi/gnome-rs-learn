mod custom_window;
mod task_object;
mod task_row;
mod utils;

use gdk::Display;

use custom_window::CustomWindow;

use gtk::{
    gdk, gio, glib::ExitCode, prelude::{ApplicationExt, ApplicationExtManual}, traits::{GtkApplicationExt, GtkWindowExt}, Application, CssProvider
};

const APP_ID: &str = "org.gtk_rs.Todo";

fn main() -> ExitCode {
    gio::resources_register_include!("todo.gresource").expect("Failed to register resources.");

    let app: Application = Application::builder().application_id(APP_ID).build();

    
    app.connect_startup(|app| {
        setup_shortcuts(app);
        load_css()
    });

    app.connect_activate(build_ui);

    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("/org/gtk_rs/Todo/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
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
