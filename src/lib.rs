#[macro_use]
extern crate matches;
#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::*;

lazy_static! {
    static ref COPYABLE: Vec<Ty> = vec![
        Ty::Path(None, Path { global: false, segments: vec![PathSegment { ident: Ident::new("i64"), parameters: PathParameters::none() }] })
    ];
}

#[proc_macro_derive(EnumMethods)]
pub fn enum_methods(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();
    let gen = impl_enum_methods(&ast);
    gen.parse().unwrap()
}

fn impl_enum_methods(ast: &DeriveInput) -> quote::Tokens {
    assert!(matches!(ast.body, Body::Enum(_)), "EnumMethods may only be used on enums");
    let mut tokens = impl_getter_methods(ast);
    tokens.append(&mut impl_is_methods(ast));

    tokens
}

fn impl_getter_methods(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    assert!(variants.iter().all(|v| v.data.fields().len() <= 1), "all EnumMethods enum variants must have 0 or 1 members");

    let variant_names = variants.iter()
        .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
        .filter(|v| !v.data.fields().is_empty())
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = variants.iter()
        .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
        .filter(|v| !v.data.fields().is_empty())
        .map(|v| v.ident.to_string().to_lowercase().into())
        .collect::<Vec<Ident>>();

    let function_name_strs = variants.iter()
        .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
        .filter(|v| !v.data.fields().is_empty())
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = variants.iter()
        .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
        .filter(|v| !v.data.fields().is_empty())
        .map(|v| &v.data.fields()[0].ty)
        .map(|ty| if COPYABLE.contains(ty) { ty.clone() } else { Ty::Rptr(None, Box::new(MutTy { ty: ty.clone(), mutability: Mutability::Immutable })) })
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());
    
    quote! {
        impl #name {
            #(
                fn #function_names(&self) -> #variant_types {
                    if let &#getter_names::#variant_names(ref v) = self {
                        v
                    }
                    else {
                        panic!(concat!("called ", #function_name_strs, "() on {:?}"), self);
                    }
                }
            )*
        }
    }
}

fn impl_is_methods(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    let variant_names = variants.iter()
        .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = variants.iter()
        .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
        .map(|v| format!("is_{}", v.ident.to_string().to_lowercase()).into())
        .collect::<Vec<Ident>>();

    let variant_counts = variants.iter()
        .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
        .map(|v| vec!(Ident::new("_"); v.data.fields().len()))
        .collect::<Vec<_>>();

    let getter_names = vec!(name.clone(); variant_names.len());
    
    quote! {
        impl #name {
            #(fn #function_names(&self) -> bool {
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
