mod window;

use gtk::gdk::Display;
use gtk::{prelude::*, CssProvider};
use gtk::{glib, Application};

use crate::window::Window;

const APP_ID: &str = "org.gtk_rs.Css";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    let provider: CssProvider = CssProvider::new();
    provider.load_from_string(include_str!("css/style.css"));

    gtk::style_context_add_provider_for_display(&Display::default().expect("Could not connect to a display."), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn build_ui(app: &Application) {
    // Create a new window and present it
    let window = Window::new(app);
    window.present();
}