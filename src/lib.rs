/*!
# enum-methods

[![Build Status](https://travis-ci.org/alekratz/enum-methods.svg?branch=master)](https://travis-ci.org/alekratz/enum-methods)
[![crates.io](https://img.shields.io/crates/v/enum-methods.svg)](https://crates.io/crates/enum-methods)

Enum getter/`is_*` method generation.

#### Please note that this crate is unstable and is subject to change frequently.
I will attempt to prevent *seriously* breaking changes after we hit 0.1.0.

# Links

* [Github](https://github.com/alekratz/enum-methods)
* [crates.io](https://crates.io/crates/enum-methods)
* [docs.rs](https://docs.rs/enum-methods/0.0.4/enum_methods/)

# Usage

In your `Cargo.toml`, add this line under your `[dependencies]` section:

```toml,no_run
enum-methods = "0.0.4"
```

To use, simply derive and call methods (see the example below).

# Why?

Usually when you write an enum with one or zero values, you might want to
add a set of getters for them. As such:

```rust
#[derive(Debug)]
enum MyEnum {
    Foo(i64),
    Bar(char),
    Baz(String),
}

impl MyEnum {
    pub fn foo(&self) -> i64 {
        if let &MyEnum::Foo(i) = self {
            i
        }
        else {
            panic!("called MyEnum::Foo() on {:?}", self)
        }
    }
    // et cetera
}

```

But this gets tedious, and adds a lot code for this simple functionality.
Enter `enum-methods`.

Instead of doing the above with the `if let ... else { panic!(...) }`, you
simply derive from the `EnumIntoGetters`

```rust
#[macro_use]
extern crate enum_methods;

#[derive(EnumIntoGetters, EnumAsGetters, Debug)]
enum MyEnum {
    Foo(i64),
    Bar(char),
    Baz(String),
}

fn main() {
    let my_foo = MyEnum::Foo(42);
    // gets as a reference
    assert_eq!(*my_foo.as_foo(), 42);
    // or consume the enum
    assert_eq!(my_foo.into_foo(), 42);
}
```

# Requirements and gotchas

Right now, `enum-methods` has only three derivable options:
* `EnumAsGetters`
* `EnumIntoGetters`
* `EnumIsA`

`EnumAsGetters` and `EnumIntoGetters` both have a couple of limitations.

* Any enum variant which has exactly 1 member will have a getter generated for
  it. All other variants are ignored.
* Enums which derive from `EnumIntoGetters` must also derive from `Debug` - this
  is for when a method is called for the wrong variant and needs to `panic!`.

`EnumIsA` is much simpler than the previous; it simply adds `is_XXX`
methods returning a boolean for whether the variant matches or not.

**For both methods, all names are automatically converted to snake_case**.

# License

This software is released under the Apache license 2.0. See the LICENSE file
for more details.

*/

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::*;

// TODO : map types for what a reference should return in its getter
// e.g. String -> &str in the getter

#[proc_macro_derive(EnumAsGetters)]
pub fn enum_as_getters(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();
    let getters = impl_enum_as_getters(&ast);
    getters.parse().unwrap()
}

#[proc_macro_derive(EnumIntoGetters)]
pub fn enum_into_getters(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();
    let getters = impl_enum_into_getters(&ast);
    getters.parse().unwrap()
}

#[proc_macro_derive(EnumIsA)]
pub fn enum_is_a(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();
    let mut gen = impl_enum_is_a(&ast);
    gen.append(&mut impl_struct_enum_is_a(&ast));
    gen.append(&mut impl_unit_enum_is_a(&ast));
    gen.parse().unwrap()
}

fn to_snake_case<S: AsRef<str>>(ident: &S) -> String {
    let mut snake_case = String::new();

    for (i, c) in ident.as_ref().chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            snake_case.push('_');
        }

        snake_case.push(c.to_lowercase().next().unwrap());
    }

    snake_case
}

fn impl_enum_as_getters(ast: &DeriveInput) -> quote::Tokens {
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
        .map(|v| format!("as_{}", to_snake_case(&v.ident)).into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = getter_filter!()
        .map(|v| &v.data.fields()[0].ty)
        .map(|ty| Ty::Rptr(None, Box::new(MutTy { ty: ty.clone(), mutability: Mutability::Immutable })))
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());

    quote! {
        #[allow(dead_code)]
        impl #name {
            #(pub fn #function_names(&self) -> #variant_types {
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

fn impl_enum_into_getters(ast: &DeriveInput) -> quote::Tokens {
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

fn impl_enum_is_a(ast: &DeriveInput) -> quote::Tokens {
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

fn impl_unit_enum_is_a(ast: &DeriveInput) -> quote::Tokens {
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

fn impl_struct_enum_is_a(ast: &DeriveInput) -> quote::Tokens {
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
