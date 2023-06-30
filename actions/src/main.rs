use gtk::{
    gio::SimpleAction,
    glib::{self, ExitCode, clone},
    prelude::{ActionMapExt, ApplicationExt, ApplicationExtManual},
    traits::{GtkWindowExt, GtkApplicationExt},
    Application, ApplicationWindow,
};

const APP_ID: &str = "org.gtk_rs.Actions";
const APP_TITLE: &str = "My GTK App";

fn main() -> ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);

    app.run()
}

fn build_ui(app: &Application) {
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .width_request(360)
        .build();

    // Add action "close" to `window` taking no parameter
    let action_close: SimpleAction = SimpleAction::new("close", None);

    action_close.connect_activate(clone!(@weak window => move |_, _| {
        window.close();
    }));

    window.add_action(&action_close);

    // Present window
    window.present();
}
