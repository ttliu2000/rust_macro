# rust_macro

Rust macro project to make rust programming easier. Being a compiler engineer, I like generate code rather than writing code. :)

The following are macro list

## hello marco

   this is the sample generated from ChatGPT with a little modification

## is_xxx macro for enum

   this is the macro generate is_xxx for an enum.

   usage sample:

```rust

#[derive(GenIsEnumVariant)]
pub enum MyEnum {
    Variant1,
    Variant2,
}
```

there will be two functions is_variant1 and is_variant2 added to MyEnum

## getter and setter for struct

this macro generate getter and setter for struct's field if it's not public. It's more like C#'s getter and setter function.

usage sample:

```rust

#[derive(Accessor)]
pub struct MyStruct {
   pub field : int,
   field2: int,
}

```

the getter and setter function will be generated for field2. The function name is get_field2() and set_field2.
