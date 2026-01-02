use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::Data;
use syn::DataEnum;
use quote::quote;

use crate::has_skip_attr;

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
        let method_name = syn::Ident::new(
            &format!("is_{}", vname.to_string().to_lowercase()),
            vname.span(),
        );

        let r = quote! {
            pub fn #method_name(&self) -> bool {
                matches!(self, #name::#vname)
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