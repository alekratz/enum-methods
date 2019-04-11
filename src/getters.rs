use crate::util::*;
use syn::*;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren};
use proc_macro::{Span, TokenStream};

pub(crate) fn impl_enum_as_getters(ast: &ItemEnum) -> TokenStream {
    let name = &ast.ident;

    let variant_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| Ident::new(&format!("as_{}", to_snake_case(&v.ident.to_string())), Span::call_site().into()))
        .collect::<Vec<Ident>>();

    let function_name_strs = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| &v.fields.iter().next().expect("Unreachable").ty)
        .map(|ty| Type::Reference(TypeReference {
            and_token: Token![&]([Span::call_site().into()]),
            lifetime: None,
            mutability: None,
            elem: Box::new(ty.clone()),
        }))
        .collect::<Vec<Type>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let mut tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
                if let &#getter_names::#variant_names(ref v) = self {
                    v
                } else {
                    panic!(concat!("called as_", #function_name_strs, "() on {:?}"), self);
                }
            })*
        }
    };

    let variant_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| Ident::new(&format!("as_{}", to_snake_case(&v.ident.to_string())), Span::call_site().into()))
        .collect::<Vec<Ident>>();

    let function_name_strs = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| {
            let elems = v.fields.iter().map(|field| Type::Reference(TypeReference {
                and_token: Token![&]([Span::call_site().into()]),
                lifetime: None,
                mutability: None,
                elem: Box::new(field.ty.clone()),
            })).collect::<Punctuated<_, Comma>>();
            let tuple_ty = TypeTuple {
                paren_token: Paren { span: Span::call_site().into() },
                elems,
            };

            Type::Tuple(tuple_ty)
        })
        .collect::<Vec<Type>>();

    let getter_names_multiple = vec!(name.clone(); variant_types.len());

    let tuple_args = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.fields.iter().count()))
        .collect::<Vec<_>>();

    let tuple_args2 = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.fields.iter().count()))
        .collect::<Vec<_>>();

    tokens.extend(quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
                if let &#getter_names_multiple::#variant_names(#(ref #tuple_args),*) = self {
                    (#(#tuple_args2), *)
                } else {
                    panic!(concat!("called as_", #function_name_strs, "() on {:?}"), self);
                }
            })*
        }
    });

    tokens.into()
}

pub(crate) fn impl_enum_into_getters(ast: &ItemEnum) -> TokenStream {
    let name = &ast.ident;

    let variant_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| Ident::new(&format!("into_{}", to_snake_case(&v.ident.to_string())), Span::call_site().into()))
        .collect::<Vec<Ident>>();

    let function_name_strs = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| v.fields.iter().next().expect("Unreachable").ty.clone())
        .collect::<Vec<Type>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let mut tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(self) -> #variant_types {
                if let #getter_names::#variant_names(v) = self {
                    v
                } else {
                    panic!(concat!("called into_", #function_name_strs, "() on {:?}"), self);
                }
            })*
        }
    };

    let variant_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| Ident::new(&format!("into_{}", to_snake_case(&v.ident.to_string())), Span::call_site().into()))
        .collect::<Vec<Ident>>();

    let function_name_strs = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| {
            let elems = v.fields.iter().map(|field| field.ty.clone()).collect::<Punctuated<_, Comma>>();
            let tuple_ty = TypeTuple {
                paren_token: Paren { span: Span::call_site().into() },
                elems,
            };

            Type::Tuple(tuple_ty)
        })
        .collect::<Vec<Type>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let tuple_args = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.fields.iter().count()))
        .collect::<Vec<_>>();

    let tuple_args2 = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.fields.iter().count()))
        .collect::<Vec<_>>();

    tokens.extend(quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(self) -> #variant_types {
                if let #getter_names::#variant_names(#(#tuple_args),*) = self {
                    (#(#tuple_args2), *)
                } else {
                    panic!(concat!("called into_", #function_name_strs, "() on {:?}"), self);
                }
            })*
        }
    });

    tokens.into()
}

pub(crate) fn impl_enum_to_getters(ast: &ItemEnum) -> TokenStream {
    let name = &ast.ident;

    let variant_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| Ident::new(&format!("to_{}", to_snake_case(&v.ident.to_string())), Span::call_site().into()))
        .collect::<Vec<Ident>>();

    let function_name_strs = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() == 1)
        .map(|v| v.fields.iter().next().expect("Unreachable").ty.clone())
        .collect::<Vec<Type>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let mut tokens = quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
                if let &#getter_names::#variant_names(ref v) = self {
                    v.clone()
                } else {
                    panic!(concat!("called to_", #function_name_strs, "() on {:?}"), self);
                }
            })*
        }
    };

    let variant_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| Ident::new(&format!("to_{}", to_snake_case(&v.ident.to_string())), Span::call_site().into()))
        .collect::<Vec<Ident>>();

    let function_name_strs = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| {
            let elems = v.fields.iter().map(|field| field.ty.clone()).collect::<Punctuated<_, Comma>>();
            let tuple_ty = TypeTuple {
                paren_token: Paren { span: Span::call_site().into() },
                elems,
            };

            Type::Tuple(tuple_ty)
        })
        .collect::<Vec<Type>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    let tuple_args = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.fields.iter().count()))
        .collect::<Vec<_>>();

    let tuple_args2 = variant_filter!(ast.variants => Unnamed)
        .filter(|v| v.fields.iter().count() > 1)
        .map(|v| UniqueIdentifierIterator::new().take(v.fields.iter().count()))
        .collect::<Vec<_>>();

    tokens.extend(quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
                if let &#getter_names::#variant_names(#(ref #tuple_args),*) = self {
                    (#(#tuple_args2.clone()), *)
                } else {
                    panic!(concat!("called to_", #function_name_strs, "() on {:?}"), self);
                }
            })*
        }
    });

    tokens.into()
}
