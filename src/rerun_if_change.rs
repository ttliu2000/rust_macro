use std::path::Path;

use proc_macro::TokenStream;
use syn::{parse_macro_input, punctuated::Punctuated, LitStr, Token};
use quote::quote;

pub fn rerun_if_changed(input: TokenStream) -> TokenStream {
    let paths =
        parse_macro_input!(input with Punctuated::<LitStr, Token![,]>::parse_terminated);

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir);

    let mut stmts = Vec::new();

    for path_lit in paths {
        let path = path_lit.value();
        let full_path = manifest_dir.join(&path);

        if !full_path.exists() {
            return syn::Error::new(
                path_lit.span(),
                format!("file does not exist: {}", full_path.display()),
            )
            .to_compile_error()
            .into();
        }

        stmts.push(quote! {
            println!("cargo:rerun-if-changed={}", #path);
        });
    }

    quote! { #(#stmts)* }.into()
}
