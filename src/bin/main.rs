use askama::Template;
use lungfish::generate;

#[derive(Template)]
#[template(path = "layout.html")]
pub struct HelloTemplate<'a> {
    test_var_1: &'a str,
}

fn main() {
    generate();

    let hello = HelloTemplate { test_var_1: "this is a temp var" };
    println!("{}", hello.render().unwrap());
}
