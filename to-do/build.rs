use glib_build_tools::compile_resources;

fn main() {
    compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "todo.gresource",
    );
}
