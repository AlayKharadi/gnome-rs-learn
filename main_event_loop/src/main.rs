use gtk::{
    glib::{self, clone, MainContext, Priority, Sender},
    prelude::*,
    Application, ApplicationWindow, Button,
};
use std::{thread, time::Duration};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";
const APP_TITLE: &str = "Title";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button: Button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let (sender, receiver) = MainContext::channel(Priority::default());

    button.connect_clicked(move |_| {
        let sender: Sender<bool> = sender.clone();

        thread::spawn(move || {
            sender.send(false).expect("Could not send through channel.");
            std::thread::sleep(Duration::from_secs(10));
            sender.send(true).expect("Could not send through channel.");
        });
    });

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(
        None,
        clone!(@weak button => @default-return Continue(false),
                    move |enable_button| {
                        button.set_sensitive(enable_button);
                        Continue(true)
                    }
        ),
    );

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&button)
        .build();

    window.present()
}
