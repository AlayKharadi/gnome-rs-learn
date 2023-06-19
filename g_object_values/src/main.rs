use gtk::prelude::*;

fn main() {
    let integer_variant = 10.to_variant();
    let integer = integer_variant.get::<i32>().expect("The variant needs to be of type i32");
    assert_eq!(integer, 10);

    let variant = vec!["Hello", "there!"].to_variant();
    assert_eq!(variant.n_children(), 2);
    let vec = &variant.get::<Vec<String>>().expect("The variant needs to be of type `String`.");
    assert_eq!(vec[0], "Hello");
}
