use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use syn::{parse_macro_input, LitStr};

use crate::utils::*;

// Implementation moved out of crate root; the root-level wrapper (in src/lib.rs)
// will call this function.
pub fn ini_file_impl(input: TokenStream) -> TokenStream {
    // 1. Parse the string literal path
    let path_lit = parse_macro_input!(input as LitStr);

    // 2. Get full file path
    let path = match get_file_pathbuf(&path_lit) {
        Ok(n) => n,
        Err(e) => return e,   
    };

    // 3. Read file (compile-time!)
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            let root_folder_info = format!("file = {}", path.display());
            let err_msg = format!("failed to read ini file: {e} and {root_folder_info}");
            return syn::Error::new_spanned(
                path_lit,
                err_msg,
            )
            .to_compile_error()
            .into();
        }
    };

    // 4. Parse INI (very simple parser)
    let mut inserts = Vec::new();

    let mut keys = Vec::new();

    for (line_no, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (k, v) = match line.split_once('=') {
            Some(x) => x,
            None => {
                return syn::Error::new_spanned(
                    &path_lit,
                    format!("invalid INI at line {}", line_no + 1),
                )
                .to_compile_error()
                .into();
            }
        };

        let key = k.trim().to_string();
        let value = v.trim().to_string();
        
        if keys.contains(&key) {
            return syn::Error::new_spanned(
                &path_lit,
                format!("duplicate key '{}' at line {}", key, line_no + 1),
            )
            .to_compile_error()
            .into();
        }
        else {
            keys.push(key.clone());
        }

        inserts.push(quote! {
            map.insert(#key.to_string(), #value.to_string());
        });
    }

    // 5. Generate Rust code
    quote! {{
        let mut map = ::std::collections::HashMap::new();
        #(#inserts)*
        map
    }}
    .into()
}
