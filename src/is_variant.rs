use proc_macro::TokenStream;
use syn::Fields;
use syn::Variant;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::Data;
use syn::DataEnum;
use quote::quote;

use crate::has_skip_attr;

fn pattern_for_variant(enum_name: &syn::Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let vname = &variant.ident;

    match &variant.fields {
        Fields::Unit => {
            quote! { #enum_name::#vname }
        }

        Fields::Unnamed(fields) => {
            let underscores = fields.unnamed.iter().map(|_| quote! { _ });
            quote! { #enum_name::#vname( #( #underscores ),* ) }
        }

        Fields::Named(_) => {
            quote! { #enum_name::#vname { .. } }
        }
    }
}

pub (crate) fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Make sure it is an enum
    let variants = if let Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        return syn::Error::new_spanned(name, "GenIsEnumVariant can only be applied to enums")
            .to_compile_error()
            .into();
    };

    // Generate is_xxx methods
    let methods = variants.iter().filter_map(|v| {
        if has_skip_attr(&v.attrs, "is_variant", "skip") {
            return None;
        } 

        let vname = &v.ident;
        let fn_name = syn::Ident::new(
    &format!("is_{}", vname.to_string().to_lowercase()),
            vname.span(),
        );

        let pattern = pattern_for_variant(&name, v);

        let r = quote! {
            pub fn #fn_name(&self) -> bool {
                matches!(self, #pattern)
            }
        };

        Some(r)
    });

    let expanded = quote! {
        impl #name {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}