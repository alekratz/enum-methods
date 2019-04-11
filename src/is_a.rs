use syn::*;
use crate::util::*;
use proc_macro::{Span, TokenStream};

/// Gives implementations of is_a_* functions for tuples.
pub(crate) fn impl_enum_is_a(ast: &ItemEnum) -> TokenStream {
    let name = &ast.ident;
    let variants = &ast.variants;

    macro_rules! is_a_filter {
        () => {
            variants.iter()
                .filter(|v| if let Fields::Unnamed(_) = v.fields { true } else { false })
        };
    }

    let variant_names = is_a_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = is_a_filter!()
        .map(|v| Ident::new(&format!("is_{}", to_snake_case(&v.ident.to_string())), Span::call_site().into()))
        .collect::<Vec<Ident>>();

    let getter_names = vec!(name.clone(); variant_names.len());

    let tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> bool {
                if let &#getter_names::#variant_names(..) = self {
                    true
                } else {
                    false
                }
            })*
        }
    };

    tokens.into()
}

pub(crate) fn impl_unit_enum_is_a(ast: &ItemEnum) -> TokenStream {
    let name = &ast.ident;
    let variants = &ast.variants;

    macro_rules! is_a_filter {
        () => {
            variants.iter()
                .filter(|v| if let Fields::Unit = v.fields { true } else { false })
        };
    }

    let variant_names = is_a_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = is_a_filter!()
        .map(|v| Ident::new(&format!("is_{}", to_snake_case(&v.ident.to_string())), Span::call_site().into()))
        .collect::<Vec<Ident>>();

    let getter_names = vec!(name.clone(); variant_names.len());

    let tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> bool {
                if let &#getter_names::#variant_names = self {
                    true
                } else {
                    false
                }
            })*
        }
    };

    tokens.into()
}

pub(crate) fn impl_struct_enum_is_a(ast: &ItemEnum) -> TokenStream {
    let name = &ast.ident;
    let variants = &ast.variants;

    macro_rules! is_a_filter {
        () => {
            variants.iter()
                .filter(|v| if let Fields::Named(_) = v.fields { true } else { false })
        };
    }

    let variant_names = is_a_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = is_a_filter!()
        .map(|v| Ident::new(&format!("is_{}", to_snake_case(&v.ident.to_string())), Span::call_site().into()))
        .collect::<Vec<Ident>>();

    let getter_names = vec!(name.clone(); variant_names.len());

    let tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> bool {
                if let &#getter_names::#variant_names { .. } = self {
                    true
                } else {
                    false
                }
            })*
        }
    };

    tokens.into()
}
