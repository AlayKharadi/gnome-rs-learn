use gtk::{
    gio::SimpleAction,
    glib::{self, clone, ExitCode},
    prelude::{ActionMapExt, ApplicationExt, ApplicationExtManual, StaticVariantType, ToVariant, ActionExt},
    traits::{BoxExt, ButtonExt, GtkApplicationExt, GtkWindowExt, WidgetExt},
    Align, Application, ApplicationWindow, Box, Button, Label, Orientation,
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
    let original_state: i32 = 0;
    let label: Label = Label::builder()
        .label(format!("Counter: {original_state}"))
        .build();

    let button: Button = Button::builder().label("Press me.").build();
    button.connect_clicked(|button: &Button| {
        let parameter: i32 = 1;
        button
            .activate_action("win.count", Some(&parameter.to_variant()))
            .expect("The action does not exist.");
    });

    let gtk_box: Box = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&label);

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .width_request(360)
        .child(&gtk_box)
        .build();

    // Add action "close" to `window` taking no parameter
    let action_count: SimpleAction = SimpleAction::new_stateful(
        "count",
        Some(&i32::static_variant_type()),
        original_state.to_variant(),
    );

    action_count.connect_activate(clone!(@weak label => move |action, parameter| {
        let mut state = action.state().expect("Could not get state.").get::<i32>().expect("The variant need");
        
        let parameter = parameter.expect("Could not get parameter.").get::<i32>().expect("The variant needs to be of type `i32`.");

        state += parameter;

        action.set_state(state.to_variant());

        label.set_label(&format!("Counter: {state}"))
    }));
    window.add_action(&action_count);

    // Present window
    window.present();
}
