use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput,
    GenericParam,
    Ident,
    TypeParamBound,
};


fn get_generic(ast: &DeriveInput) -> Option<(Ident, Ident)> {
    let mut param_ident: Option<Ident> = None;
    let mut segment_ident: Option<Ident> = None;

    // Find `Ident`s for `<T = Numeric>`.
    for param in &ast.generics.params {
        if let GenericParam::Type(type_param) = param {
            for bound in &type_param.bounds {
                if let TypeParamBound::Trait(trait_bound) = bound {
                    for segment in &trait_bound.path.segments {
                        if segment.ident.to_string() == "Numeric" {
                            param_ident = Some(type_param.ident.clone());
                            segment_ident = Some(segment.ident.clone());
                            break;
                        }
                    }
                }
            }
        }
    }

    if param_ident.as_ref().and(segment_ident.as_ref()).is_some() {
        Some((param_ident.unwrap(), segment_ident.unwrap()))
    } else {
        None
    }
}

pub fn derive(ast: DeriveInput) -> TokenStream {
    eprintln!("{:#?}", ast);
    let name = &ast.ident;
    let (tname, ttrait) = get_generic(&ast).expect("No parameters were found!");

    let gen = quote! {
        impl<#tname: #ttrait> Tuple2<#tname> for #name<#tname> {
            fn ndim() -> usize {
                2
            }
        }
    };
    gen.into()
}
