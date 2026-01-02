#[derived(GenIsEnumVariant)]
pub enum MyEnum {
    VariantA,
    #[is_variant(skip)]
    VariantB,
    VariantC,
}

fn main() {
    let e = MyEnum::VariantB;

    e.is_variantb();
    //~^ ERROR method `is_variantb` not found for this enum
}