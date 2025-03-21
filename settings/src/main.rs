use gio::Settings;
use gtk::prelude::*;
use gtk::{gio, glib, Align, Application, ApplicationWindow, Switch};

const APP_ID: &str = "org.gtk_rs.Settings";

fn main() -> glib::ExitCode {
    // Create a new application
    let app: Application = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Initialize settings
    let settings = Settings::new(APP_ID);

    // Get the last switch state from the settings
    let is_switch_enabled: bool = settings.boolean("is-switch-enabled");

    // Create a switch
    let switch: Switch = Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .state(is_switch_enabled)
        .build();

    settings.bind("is-switch-enabled", &switch, "active").build();

    // Create a window
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&switch)
        .build();

    // Present window
    window.present();
}