use proc_macro::TokenStream;
use syn;

mod tuple2;

#[proc_macro_derive(Tuple2)]
pub fn derive_tuple2(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    tuple2::derive(ast)
}
