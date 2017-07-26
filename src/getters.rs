use util::*;
use syn::*;
use quote;

pub(crate) fn impl_enum_as_getters(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    macro_rules! getter_filter {
        () => {
            variants.iter()
                .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
        };
    }


    let variant_names = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| format!("as_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types_single = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| &v.data.fields()[0].ty)
        .map(|ty| Ty::Rptr(None, Box::new(MutTy { ty: ty.clone(), mutability: Mutability::Immutable })))
        .collect::<Vec<Ty>>();

    let getter_names_single = vec!(name.clone(); variant_types_single.len());

    let mut tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types_single {
                    if let &#getter_names_single::#variant_names(ref v) = self {
                        v
                    }
                    else {
                        panic!(concat!("called ", #function_name_strs, "() on {:?}"), self);
                    }
                }
            )*
        }
    };

    let variant_names = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| format!("as_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types_multiple = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| Ty::Tup(v.data.fields().iter().map(|field| Ty::Rptr(None, Box::new(MutTy { ty: field.ty.clone(), mutability: Mutability::Immutable }))).collect::<Vec<Ty>>()))
        .collect::<Vec<Ty>>();

    let getter_names_multiple = vec!(name.clone(); variant_types_multiple.len());

    tokens.append(quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types_multiple {
                    if let &#getter_names_multiple::#variant_names(ref v) = self {
                        v
                    }
                    else {
                        panic!(concat!("called ", #function_name_strs, "() on {:?}"), self);
                    }
                }
            )*
        }
    });

    tokens
}

pub(crate) fn impl_enum_into_getters(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    macro_rules! getter_filter {
        () => {
            variants.iter()
                .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
                .filter(|v| v.data.fields().len() == 1)
        };
    }


    let variant_names = getter_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = getter_filter!()
        .map(|v| format!("into_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = getter_filter!()
        .map(|v| v.data.fields()[0].ty.clone())
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(self) -> #variant_types {
                    if let #getter_names::#variant_names(v) = self {
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

pub(crate) fn impl_enum_to_getters(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    macro_rules! getter_filter {
        () => {
            variants.iter()
                .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
                .filter(|v| v.data.fields().len() == 1)
        };
    }


    let variant_names = getter_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = getter_filter!()
        .map(|v| format!("to_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = getter_filter!()
        .map(|v| v.data.fields()[0].ty.clone())
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
                    if let &#getter_names::#variant_names(ref v) = self {
                        v.clone()
                    }
                    else {
                        panic!(concat!("called ", #function_name_strs, "() on {:?}"), self);
                    }
                }
            )*
        }
    }
}
