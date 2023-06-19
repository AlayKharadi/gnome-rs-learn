mod custom_button;

use custom_button::CustomButton;
use gtk::{glib, prelude::*, Align, Application, ApplicationWindow, Box, Orientation};

const APP_ID: &str = "org.gtk_rs.GObjectProperties1";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button_1: CustomButton = CustomButton::new();
    let button_2: CustomButton = CustomButton::new();

    button_1
        .bind_property("number", &button_2, "number")
        .transform_to(|_, number: i32| {
            let incremented_number: i32 = number + 1;
            Some(incremented_number.to_value())
        })
        .transform_from(|_, number: i32| {
            let decremented_number: i32 = number - 1;
            Some(decremented_number.to_value())
        })
        .bidirectional()
        .sync_create()
        .build();

    button_1.connect_number_notify(|button| {
        println!("Then current number of `button_1` is {}.", button.number());
    });

    let gtk_box: Box = Box::builder()
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&button_1);
    gtk_box.append(&button_2);

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(400)
        .default_height(400)
        .child(&gtk_box)
        .build();

    window.present()
}
