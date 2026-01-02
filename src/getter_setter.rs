use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

/// Derive macro to generate getter and setter methods for struct fields, excluding public fields
pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => {
            return syn::Error::new_spanned(
                name,
                "Accessors can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    let methods = fields.iter().filter_map(|f| {
        if matches!(f.vis, syn::Visibility::Public(_)) {
            return None;
        }

        let field = f.ident.as_ref()?;
        let getter_name = syn::Ident::new(&format!("get_{}", field), field.span());
        let ty = &f.ty;

        let getter = quote! {
            pub fn #getter_name(&self) -> &#ty {
                &self.#field
            }
        };

        let setter_name =
            syn::Ident::new(&format!("set_{}", field), field.span());

        let setter = quote! {
            pub fn #setter_name(&mut self, value: #ty) {
                self.#field = value;
            }
        };

        Some(quote! { #getter #setter })
    });

    quote! {
        impl #name {
            #(#methods)*
        }
    }
    .into()
}
