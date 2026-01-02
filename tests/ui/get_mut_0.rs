use rust_macro::GetMut;

#[derive(GetMut)]
struct Foo {
    x: i32,
    #[get_mut(skip)]
    y: String,
}

fn main() {
    let mut f = Foo { x: 1, y: String::new() };
    *f.get_x_mut() = 10;
}