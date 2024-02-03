use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, glib, Application, ApplicationWindow, Button, CssProvider};

// ANCHOR: main
const APP_ID: &str = "org.gtk_rs.Css1";

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
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("css/style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create button
    let button1 = Button::with_label("Delete");
    let button2 = Button::with_label("Next");

    button1.add_css_class("destructive-action");
    button2.add_css_class("suggested-action");

    let gtk_box = gtk::Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .build();
    gtk_box.append(&button1);
    gtk_box.append(&button2);

    // Create a new window and present it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();
    window.present();
}