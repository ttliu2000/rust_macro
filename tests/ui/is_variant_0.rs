use rust_macro::GenIsEnumVariant;

#[derive(GenIsEnumVariant)]
pub enum MyEnum {
    VariantA,
    VariantB,
    VariantC,
}

fn main() {
    let e = MyEnum::VariantB;

    assert_eq!(e.is_varianta(), false);
    assert_eq!(e.is_variantb(), true);
    assert_eq!(e.is_variantc(), false);

    println!("All tests passed!");
}