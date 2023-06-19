use gtk::prelude::*;

fn main() {
    let integer_value = 10.to_value();
    let integer = integer_value.get::<i32>().expect("The value needs to be of type i32.");
    assert_eq!(integer, 10);

    let string_value = "Hello!".to_value();
    let string = string_value.get::<String>().expect("The value needs to be of type of String.");
    assert_eq!(string, "Hello!".to_string());

    let string_some_value = "Hello!".to_value();
    let string_some = string_some_value.get::<Option<String>>().expect("The value needs to be of type `Option<String>.`");
    assert_eq!(string_some, Some("Hello!".to_string()));

    let string_none_value = None::<String>.to_value();
    let string_none = string_none_value.get::<Option<String>>().expect("The value needs to be of type `Option<String>`.");
    assert_eq!(string_none, None);
}
