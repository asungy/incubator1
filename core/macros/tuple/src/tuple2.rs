use syn;
use quote::quote;
use proc_macro::TokenStream;

pub fn derive(ast: syn::DeriveInput) -> TokenStream {
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
