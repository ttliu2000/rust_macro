use rust_macro::ini_file;

fn main() {
    let _ = ini_file!("not_exists.ini");
}