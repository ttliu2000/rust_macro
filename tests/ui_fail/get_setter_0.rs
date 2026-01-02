use rust_macro::Accessors;

#[derive(Accessors)]
struct Foo {
    x: i32,
    #[getter(skip)]
    y: String,
}

fn main() {
    let mut f = Foo { x: 1, y: String::new() };

    f.set_y(String::from("hello"));
    let _ = f.get_y();
}