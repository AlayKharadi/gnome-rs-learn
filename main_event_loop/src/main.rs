use gtk::{
    glib::{self, clone, MainContext, timeout_future_seconds},
    prelude::*,
    Application, ApplicationWindow, Button,
};

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

    
    button.connect_clicked(move |button| {
        let main_context: MainContext = MainContext::default();

        main_context.spawn_local(clone!(@weak button => async move {
            button.set_sensitive(false);
            timeout_future_seconds(5).await;
            button.set_sensitive(true);
        }));
    });

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&button)
        .build();

    window.present()
}
