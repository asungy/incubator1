use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Tuple2 for #name {
            fn hello() -> String {
                format!("Hello, {}!", stringify!(#name))
            }
        }
    };
    gen.into()
}
