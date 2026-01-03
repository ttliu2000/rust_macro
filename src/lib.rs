use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Check if the attribute list contains a specific (skip) attribute
pub (crate) fn has_skip_attr(attrs: &[syn::Attribute], att_name:&str, tag:&str) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident(att_name) {
            return false;
        }

        let mut found = false;

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(tag) {
                found = true;
            }
            Ok(())
        });

        found
    })
}

/// Attribute macro to print "Hello from macro!" at the start of a function
#[proc_macro_attribute]
pub fn hello(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr = {:?}", attr);
    let mut func = parse_macro_input!(item as syn::ItemFn);
    let stmt: syn::Stmt = syn::parse_quote! {
        println!("Hello from macro!");
    };

    func.block.stmts.insert(0, stmt);

    quote::quote!(#func).into()
}

mod ini_file_macro;

/// Attribute macro to include and parse an INI file at compile time
#[proc_macro]
pub fn ini_file(input: TokenStream) -> TokenStream {
    ini_file_macro::ini_file_impl(input)
}

mod is_variant;

/// Derive macro to generate is_xxx methods for enum variants
#[proc_macro_derive(GenIsEnumVariant, attributes(is_variant))]
pub fn is_enum_derive(input: TokenStream) -> TokenStream {
    is_variant::expand(input)
}

mod getter_setter;

#[proc_macro_derive(Accessors, attributes(getter, setter))]
pub fn accessors(input: TokenStream) -> TokenStream {
    getter_setter::expand(input)
}

mod get_mut;

#[proc_macro_derive(GetMut, attributes(get_mut))]
pub fn get_mut(input: TokenStream) -> TokenStream {
    get_mut::expand(input)
}