use syn::*;
use quote;
use util::*;

/// Gives implementations of is_a_* functions for tuples.
pub(crate) fn impl_enum_is_a(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    macro_rules! is_a_filter {
        () => {
            variants.iter()
                .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
        };
    }

    let variant_names = is_a_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = is_a_filter!()
        .map(|v| format!("is_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let variant_counts = is_a_filter!()
        .map(|v| vec!(Ident::new("_"); v.data.fields().len()))
        .collect::<Vec<_>>();

    let getter_names = vec!(name.clone(); variant_names.len());

    quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> bool {
                if let &#getter_names::#variant_names(#(#variant_counts),*) = self {
                    true
                }
                else {
                    false
                }
            })*
        }
    }
}

pub(crate) fn impl_unit_enum_is_a(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    macro_rules! is_a_filter {
        () => {
            variants.iter()
                .filter(|v| if let VariantData::Unit = v.data { true } else { false })
        };
    }

    let variant_names = is_a_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = is_a_filter!()
        .map(|v| format!("is_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let getter_names = vec!(name.clone(); variant_names.len());

    quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> bool {
                if let &#getter_names::#variant_names = self {
                    true
                }
                else {
                    false
                }
            })*
        }
    }
}

pub(crate) fn impl_struct_enum_is_a(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    macro_rules! is_a_filter {
        () => {
            variants.iter()
                .filter(|v| if let VariantData::Struct(_) = v.data { true } else { false })
        };
    }

    let variant_names = is_a_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = is_a_filter!()
        .map(|v| format!("is_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let variant_field_names = is_a_filter!()
        .map(|v| v.data.fields().iter().map(|ref f| f.ident.as_ref().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let variant_counts = is_a_filter!()
        .map(|v| vec!(Ident::new("_"); v.data.fields().len()))
        .collect::<Vec<_>>();

    let getter_names = vec!(name.clone(); variant_names.len());

    quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> bool {
                if let &#getter_names::#variant_names { #(#variant_field_names: #variant_counts),* } = self {
                    true
                }
                else {
                    false
                }
            })*
        }
    }
}
