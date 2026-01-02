use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::Data;
use syn::DataEnum;
use quote::quote;

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

/// Derive macro to generate is_xxx methods for enum variants
#[proc_macro_derive(GenIsEnumVariant)]
pub fn is_enum_derive(input: TokenStream) -> TokenStream {
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
    let methods = variants.iter().map(|v| {
        let vname = &v.ident;
        let method_name = syn::Ident::new(
            &format!("is_{}", vname.to_string().to_lowercase()),
            vname.span(),
        );
        quote! {
            pub fn #method_name(&self) -> bool {
                matches!(self, #name::#vname)
            }
        }
    });

    let expanded = quote! {
        impl #name {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}

mod getter_setter;

#[proc_macro_derive(Accessors)]
pub fn accessors(input: TokenStream) -> TokenStream {
    getter_setter::expand(input)
}