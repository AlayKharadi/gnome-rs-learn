use std::{path::PathBuf, fs::create_dir_all};

use gtk::glib::user_data_dir;

use crate::APP_ID;

pub fn data_path() -> PathBuf {
    let mut path: PathBuf = user_data_dir();
    path.push(APP_ID);
    create_dir_all(&path).expect("Could not create directory.");
    path.push("data.json");
    path
}
