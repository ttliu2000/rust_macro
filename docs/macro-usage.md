# rust_macro Usage Guide

This document describes all exported macros in the `rust_macro` crate and how to use them.

## Setup

Add the crate to your project and import the macros you need.

```toml
[dependencies]
rust_macro = { path = "../rust_macro" }
```

```rust
use rust_macro::*;
```

---

## 1) `#[derive(GenIsEnumVariant)]`

Generates `is_<variant>() -> bool` methods for enum variants.

### Example

```rust
use rust_macro::GenIsEnumVariant;

#[derive(GenIsEnumVariant)]
enum MyEnum {
    VariantA,
    VariantB,
    VariantC,
}

fn demo(e: &MyEnum) {
    assert_eq!(e.is_varianta(), matches!(e, MyEnum::VariantA));
    assert_eq!(e.is_variantb(), matches!(e, MyEnum::VariantB));
    assert_eq!(e.is_variantc(), matches!(e, MyEnum::VariantC));
}
```

### Skip one variant

Use `#[is_variant(skip)]` on a variant to avoid generating the checker for that variant.

```rust
#[derive(GenIsEnumVariant)]
enum MyEnum {
    VariantA,
    #[is_variant(skip)]
    VariantB,
}
// Generates only: is_varianta()
```

### Notes

- Works only on enums.
- Method names are lowercase variant names with `is_` prefix.

---

## 2) `#[derive(Accessors)]`

Generates getter/setter methods for **non-public struct fields**.

### Generated methods

For a field named `x: T`:

- Getter: `get_x(...)`
- Setter: `set_x(&mut self, value: T)`

Getter return behavior:

- Primitive scalar (`i32`, `u64`, `bool`, `char`, `f32`, ...): returns by value.
- Other types: returns `&T`.
- `Option<T>`: returns `Option<&T>`.

### Example

```rust
use rust_macro::Accessors;

#[derive(Accessors)]
struct Foo {
    x: i32,
    y: String,
    z: Option<String>,
}

fn demo(mut f: Foo) {
    let _x: i32 = f.get_x();
    let _y: &String = f.get_y();
    let _z: Option<&String> = f.get_z();

    f.set_x(10);
    f.set_y("hello".to_string());
    f.set_z(Some("v".to_string()));
}
```

### Skip attributes

- `#[getter(skip)]` disables getter generation for a field.
- `#[setter(skip)]` disables setter generation for a field.
- Both can be used together to disable both.

```rust
#[derive(Accessors)]
struct Foo {
    #[getter(skip)]
    x: i32,
    #[setter(skip)]
    y: String,
}
// Generates: set_x(...) and get_y(...)
```

### Notes

- Works only on structs.
- Public fields are ignored (no methods generated).

---

## 3) `#[derive(GetMut)]`

Generates mutable getters for **non-public struct fields**.

For a field named `x: T`, generated method is:

- `get_x_mut(&mut self)`

Return behavior:

- Normal type `T`: returns `&mut T`
- `Option<T>`: returns `Option<&mut T>`

### Example

```rust
use rust_macro::GetMut;

#[derive(GetMut)]
struct Foo {
    x: i32,
    y: Option<String>,
    #[get_mut(skip)]
    hidden: String,
}

fn demo(mut f: Foo) {
    *f.get_x_mut() = 42;

    if let Some(v) = f.get_y_mut() {
        v.push_str("!");
    }
}
```

### Skip attribute

- `#[get_mut(skip)]` disables `get_<field>_mut` generation for that field.

### Notes

- Works only on structs.
- Public fields are ignored.

---

## 4) `#[derive(EnumAccessors)]`

Generates accessors for enum **tuple variants** (variants with unnamed fields).

For variant `MyEnum::Pair(A, B)`, generated methods are:

- `get_pair(&self) -> Option<(&A, &B)>`
- `get_pair_mut(&mut self) -> Option<(&mut A, &mut B)>`

### Example

```rust
use rust_macro::EnumAccessors;

#[derive(EnumAccessors)]
enum Expr {
    Pair(i32, String),
    Unit,
    Named { x: i32 },
}

fn demo(mut e: Expr) {
    if let Some((a, b)) = e.get_pair() {
        let _ = (a, b);
    }

    if let Some((a, b)) = e.get_pair_mut() {
        *a += 1;
        b.push_str("!");
    }
}
```

### Notes

- Works only on enums.
- Generates methods only for tuple variants with at least one field.
- Unit and named-field variants are ignored.

---

## 5) `ini2hash!("path/to/file.ini")`

Function-like macro that reads an INI-like file at compile time and generates a `HashMap<String, String>` initialization block.

Supported line format:

- `key=value`
- Empty lines and lines starting with `#` are ignored.

### Example

```rust
use rust_macro::ini2hash;

fn main() {
    let settings = ini2hash!("tests/ui/ok.ini");
    assert_eq!(settings.get("key1"), Some(&"value1".to_string()));
}
```

### Compile-time checks

- Fails if file does not exist.
- Fails if a non-empty/non-comment line does not contain `=`.
- Fails on duplicate keys.

---

## 6) `rerun_if_changed!("path1", "path2", ...)`

Function-like macro intended for `build.rs`.

Generates:

```rust
println!("cargo:rerun-if-changed=<path>");
```

for each provided path.

### Example (`build.rs`)

```rust
use rust_macro::rerun_if_changed;

fn main() {
    rerun_if_changed!("proto/schema.proto", "config/build.toml");
}
```

### Notes

- Paths are resolved relative to `CARGO_MANIFEST_DIR`.
- Compilation fails if any listed path does not exist.

---

## 7) `#[hello]`

Attribute macro for functions. Inserts:

```rust
println!("Hello from macro!");
```

at the start of the function body.

### Example

```rust
use rust_macro::hello;

#[hello]
fn run() {
    println!("work");
}
```

When `run()` executes, it prints:

1. `Hello from macro!`
2. `work`

---

## Common compile errors

- `... can only be derived for structs`:
  You applied a struct-only derive (`Accessors`, `GetMut`) to a non-struct item.
- `... can only be applied to enums`:
  You applied an enum-only derive (`GenIsEnumVariant`, `EnumAccessors`) to a non-enum item.
- INI-related errors:
  File path, duplicate key, or invalid line format issues in `ini2hash!` input file.

---

## Practical tips

- Keep field/variant names stable when public API depends on generated method names.
- Use skip attributes to avoid generating methods you do not want to expose.
- For `ini2hash!`, keep files small and deterministic because parsing is compile-time.
- For Cargo build scripts, use `rerun_if_changed!` to avoid stale generated artifacts.
