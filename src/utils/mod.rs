use std::path::PathBuf;

use proc_macro::TokenStream;
use syn::LitStr;

pub fn get_file_pathbuf(path_lit: &LitStr) -> Result<PathBuf, TokenStream> {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set");

    let mut path = PathBuf::from(manifest_dir.clone());
    let rel_path = path_lit.value();
    path.push(rel_path);
    
    // check if file exists
    if path.exists() {
        Ok(path)
    }
    else {
        let root_folder_info = format!("the root dir = {manifest_dir}, and path to file = {}", path.display());
        let err_msg = format!("The specified file is not exists. {root_folder_info}");
        let token = syn::Error::new_spanned(
            path_lit,
            err_msg,
            )
            .to_compile_error()
            .into();

        Err(token)
    }
}

pub fn to_snake_case(ident: &syn::Ident) -> syn::Ident {
    let s = ident.to_string();
    let mut out = String::new();

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                out.push('_');
            }
            for c in ch.to_lowercase() {
                out.push(c);
            }
        } else {
            out.push(ch);
        }
    }

    syn::Ident::new(&out, ident.span())
}