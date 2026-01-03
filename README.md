# rust_macro

Rust macro project to make rust programming easier. Being a compiler engineer, I like generate code rather than writing code. :)

The following are macro list. For detailed usage sample, please refer to tests/ui. This folder contains tests cases which shows how these attributes are used.

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

this macro generates getter and setter for struct's field if it's not public. It's more like C#'s getter and setter function. A skip attribute is also added to support skip code generation.

usage sample:

```rust

#[derive(Accessor)]
pub struct MyStruct {
   pub field : int,
   field2: int,
}

```

the getter and setter function will be generated for field2. The function name is get_field2() and set_field2.

## get_mut for struct

this macro generates get_xxx_mut. A skip attribute is also added to support skip code generation.

usage sample:

```rust

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

```

## ini file to hash

this macro generate hashmap code from ini file. This macro also checks the duplicated key. if there is a duplication, there will be a compile time error.

the usage sample is listed below.

```rust
use rust_macro::*;

fn main() {
    let _ = ini2hash!("ok.ini");
}
```
