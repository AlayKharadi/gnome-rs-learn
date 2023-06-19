mod custom_button;

use custom_button::CustomButton;
use gtk::{
    glib::{self, closure_local},
    prelude::*,
    Application, ApplicationWindow,
};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";
const APP_TITLE: &str = "Title";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button: CustomButton = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    button.connect_closure(
        "max-number-reached",
        false,
        closure_local!(move |_button: CustomButton, number: i32| {
            println!("The maximum number {} has been reached", number)
        }),
    );

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&button)
        .build();

    window.present()
}
