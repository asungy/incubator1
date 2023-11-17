use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive(ast: DeriveInput) -> TokenStream {
    eprintln!("{:#?}", ast);
    let name = &ast.ident;
    let gen = quote! {
        impl<T: Numeric> Tuple2<T> for #name<T> {
            fn ndim() -> usize {
                2
            }
        }
    };
    gen.into()
}
