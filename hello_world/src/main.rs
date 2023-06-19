use std::{rc::Rc, cell::Cell};

use gtk::{glib::{self, clone}, prelude::*, Application, ApplicationWindow, Button, Orientation, Box};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";
const APP_TITLE: &str = "Title";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button_increase: Button = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease: Button = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease => move |button_increase| {
        number.set(number.get() + 1);
        button_increase.set_label(&number.get().to_string());
        button_decrease.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak button_increase => move |button_decrease| {
        number.set(number.get() - 1);
        button_increase.set_label(&number.get().to_string());
        button_decrease.set_label(&number.get().to_string());
    }));

    let gtk_box: Box = gtk::Box::builder().orientation(Orientation::Vertical).build();

    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&gtk_box)
        .build();

    window.present()
}
