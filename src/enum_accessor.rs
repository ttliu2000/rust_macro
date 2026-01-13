use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};


pub fn enum_accessors(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let Data::Enum(data_enum) = input.data else {
        return syn::Error::new_spanned(
            enum_name,
            "EnumAccessors can only be used on enums",
        )
        .to_compile_error()
        .into();
    };

    let mut methods = Vec::new();

    for variant in data_enum.variants {
        let vname = variant.ident;

        match variant.fields {
            // Tuple variants: itemA(T0, T1, ...)
            Fields::Unnamed(fields) if !fields.unnamed.is_empty() => {
                let tys: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
                let bindings: Vec<_> = (0..tys.len())
                    .map(|i| format_ident!("v{}", i))
                    .collect();

                // ---------- immutable accessor
                let method = format_ident!("get_{}", vname);
                let ret = quote! { (#( &#tys ),*) };

                methods.push(quote! {
                    pub fn #method(&self) -> Option<#ret> {
                        match self {
                            #enum_name::#vname( #( #bindings ),* ) => {
                                Some(( #( #bindings ),* ))
                            }
                            _ => None,
                        }
                    }
                });

                let method = format_ident!("get_{}_mut", vname);
                let ret = quote! { (#( &mut #tys ),*) };

                methods.push(quote! {
                    pub fn #method(&mut self) -> Option<#ret> {
                        match self {
                            #enum_name::#vname( #( #bindings ),* ) => {
                                Some(( #( #bindings ),* ))
                            }
                            _ => None,
                        }
                    }
                });
            }

            // Ignore other forms (named fields, and unit variants)
            _ => {}
        }
    }

    quote! {
        impl #enum_name {
            #( #methods )*
        }
    }
    .into()
}
