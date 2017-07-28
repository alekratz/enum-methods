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

    let variant_types = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| &v.data.fields()[0].ty)
        .map(|ty| Ty::Rptr(None, Box::new(MutTy { ty: ty.clone(), mutability: Mutability::Immutable })))
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let mut tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
                    if let &#getter_names::#variant_names(ref v) = self {
                        v
                    }
                    else {
                        panic!(concat!("called as_", #function_name_strs, "() on {:?}"), self);
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

    let variant_types = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| Ty::Tup(v.data.fields().iter().map(|field| Ty::Rptr(None, Box::new(MutTy { ty: field.ty.clone(), mutability: Mutability::Immutable }))).collect::<Vec<Ty>>()))
        .collect::<Vec<Ty>>();

    let getter_names_multiple = vec!(name.clone(); variant_types.len());

    let tuple_args = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.data.fields().len()))
        .collect::<Vec<_>>();

    let tuple_args2 = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.data.fields().len()))
        .collect::<Vec<_>>();

    tokens.append(quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
                    if let &#getter_names_multiple::#variant_names(#(ref #tuple_args),*) = self {
                        (#(#tuple_args2), *)
                    }
                    else {
                        panic!(concat!("called as_", #function_name_strs, "() on {:?}"), self);
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
        };
    }

    let variant_names = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| format!("into_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| v.data.fields()[0].ty.clone())
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let mut tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(self) -> #variant_types {
                    if let #getter_names::#variant_names(v) = self {
                        v
                    }
                    else {
                        panic!(concat!("called into_", #function_name_strs, "() on {:?}"), self);
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
        .map(|v| format!("into_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| Ty::Tup(v.data.fields().iter().map(|field| field.ty.clone()).collect::<Vec<Ty>>()))
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let tuple_args = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.data.fields().len()))
        .collect::<Vec<_>>();

    let tuple_args2 = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.data.fields().len()))
        .collect::<Vec<_>>();

    tokens.append(quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(self) -> #variant_types {
                    if let #getter_names::#variant_names(#(#tuple_args),*) = self {
                        (#(#tuple_args2), *)
                    }
                    else {
                        panic!(concat!("called into_", #function_name_strs, "() on {:?}"), self);
                    }
                }
            )*
        }
    });

    tokens
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
        };
    }

    let variant_names = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| format!("to_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = getter_filter!()
        .filter(|v| v.data.fields().len() == 1)
        .map(|v| v.data.fields()[0].ty.clone())
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let mut tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
                    if let &#getter_names::#variant_names(ref v) = self {
                        v.clone()
                    }
                    else {
                        panic!(concat!("called to_", #function_name_strs, "() on {:?}"), self);
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
        .map(|v| format!("to_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| Ty::Tup(v.data.fields().iter().map(|field| field.ty.clone()).collect::<Vec<Ty>>()))
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let tuple_args = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.data.fields().len()))
        .collect::<Vec<_>>();

    let tuple_args2 = getter_filter!()
        .filter(|v| v.data.fields().len() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.data.fields().len()))
        .collect::<Vec<_>>();

    tokens.append(quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
                    if let &#getter_names::#variant_names(#(ref #tuple_args),*) = self {
                        (#(#tuple_args2.clone()), *)
                    }
                    else {
                        panic!(concat!("called to_", #function_name_strs, "() on {:?}"), self);
                    }
                }
            )*
        }
    });

    tokens
}
