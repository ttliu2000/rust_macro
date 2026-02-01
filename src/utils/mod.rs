use std::path::PathBuf;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{GenericArgument, Ident, LitStr, PathArguments, Type, TypeReference, punctuated::Punctuated, token::Comma};
use quote::quote;

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

fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            type_path.qself.is_none() &&
            type_path.path.segments.last().is_some_and(|seg| {
                seg.ident == "Option" &&
                matches!(seg.arguments, PathArguments::AngleBracketed(_))
            })
        }
        _ => false,
    }
}

pub fn option_t_to_option_ref_t(ty: &Type) -> Option<Type> {
    if !is_option(ty) {
        return None;
    }
    
    let Type::Path(tp) = ty else { return None };

    let last = tp.path.segments.last()?;
    if last.ident != "Option" {
        return None;
    }

    let PathArguments::AngleBracketed(args) = &last.arguments else {
        return None;
    };

    let GenericArgument::Type(inner_ty) = args.args.first()? else {
        return None;
    };

    // Build &T
    let ref_inner = Type::Reference(TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(inner_ty.clone()),
    });

    // Rebuild Option<&T>
    let mut new_tp = tp.clone();
    let last_mut = new_tp.path.segments.last_mut().unwrap();

    last_mut.arguments = PathArguments::AngleBracketed(
        syn::AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args: {
                let mut p = Punctuated::<GenericArgument, Comma>::new();
                p.push(GenericArgument::Type(ref_inner));
                p
            },
            gt_token: Default::default(),
        }
    );

    Some(Type::Path(new_tp))
}

fn accessor_tokens(s: &str) -> proc_macro2::TokenStream {
    s.parse().expect("invalid accessor syntax")
}

fn convert_type(ty: &Type) -> (Option<Type>, Option<proc_macro2::TokenStream>) {
    if is_option(ty) {
        (option_t_to_option_ref_t(ty), Some(accessor_tokens("as_ref()")))
    } else {
        (None, None)
    }
}

pub fn create_getters(fields:&Vec<(Ident, Type)>) -> proc_macro2::TokenStream {
    let getters: Vec<_> = fields.iter().map(|(ident, ty)| {
        let getter_name = Ident::new(&format!("get_{}", ident), Span::call_site());
        match convert_type(ty) {
            (Some(ty0), Some(postfix)) => {
                quote! {
                    pub fn #getter_name(&self) -> #ty0 {
                        self.#ident.#postfix
                    }
                }
            }
            (Some(ty0), None) => {
                quote! {
                    pub fn #getter_name(&self) -> &#ty0 {
                        &self.#ident
                    }
                }
            }
            (None, None) => {
                quote! {
                    pub fn #getter_name(&self) -> &#ty {
                        &self.#ident
                    }
                }
            }
            _ => {
                quote! {
                    compile_error!("unexpected conversion result from convert_type");
                }
            }
        }
    }).collect();

    quote! {
        #(#getters)*
    }
}

pub fn create_mut_getters(fields:&Vec<(Ident, Type)>) -> proc_macro2::TokenStream {
    let getters: Vec<_> = fields.iter().map(|(ident, ty)| {
        let getter_name = Ident::new(&format!("get_{}_mut", ident), Span::call_site());
        match convert_type(ty) {
            (Some(ty0), Some(postfix)) => {
                quote! {
                    pub fn #getter_name(&mut self) -> #ty0 {
                        self.#ident.#postfix
                    }
                }
            }
            (Some(ty0), None) => {
                quote! {
                    pub fn #getter_name(&mut self) -> &mut #ty0 {
                        self.#ident
                    }
                }
            }
            (None, _) => {
                quote! {
                    pub fn #getter_name(&mut self) -> &mut #ty {
                        self.#ident
                    }
                }
            }
            _ => {
                quote! {
                    compile_error!("unexpected conversion result from convert_type");
                }
            }
        }
    }).collect();

    quote! {
        #(#getters)*
    }
}
