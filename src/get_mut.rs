use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};
use syn::Visibility;

use crate::has_skip_attr;


pub (crate) fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => return quote! {
            compile_error!("GetMut can only be derived for structs");
        }.into(),
    };

    let methods = fields.iter().filter_map(|f| {
        if matches!(f.vis, Visibility::Public(_)) {
            return None;
        }

        if has_skip_attr(&f.attrs, "get_mut", "skip") {
            return None;
        }

        let field = f.ident.as_ref()?;
        let ty = &f.ty;

        let fn_name =
            syn::Ident::new(&format!("get_{}_mut", field), field.span());

        Some(quote! {
            pub fn #fn_name(&mut self) -> &mut #ty {
                &mut self.#field
            }
        })
    });

    quote! {
        impl #name {
            #(#methods)*
        }
    }
    .into()
}
