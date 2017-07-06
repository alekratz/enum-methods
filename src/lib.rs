/*!
# enum-methods

Enum getter/is\_XXX method generation.

# [crates.io](https://crates.io/crates/enum-methods)

# [docs.rs](https://docs.rs/enum-methods/0.0.2/enum_methods/)

# Usage

In your `Cargo.toml`, add this line under your `[dependencies]` section:

```toml,no_run
enum-methods = "0.0.2"
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
simply derive from the `EnumGetters`

```rust
#[macro_use]
extern crate enum_methods;

#[derive(EnumGetters, Debug)]
enum MyEnum {
    Foo(i64),
    Bar(char),
    Baz(String),
}

fn main() {
    let foo = MyEnum::Foo(42);
    assert_eq!(foo.foo(), 42);  // success!
}
```

# Requirements and gotchas

Right now, `enum-methods` has only two derivable options:
* `EnumGetters`
* `EnumIsA`

`EnumGetters` has a couple of limitations. First, any enum variant which
has exactly 1 member will have a getter generated for it. All other variants
are ignored. Generated methods simply use the lower-case version of their
variant name. **These names are not converated to snake_case.**
[see #1](https://github.com/alekratz/enum-methods/issues/1). Additionally,
enums which derive from `EnumGetters` must also derive from `Debug` - this
is for when a method is called for the wrong variant and needs to `panic!`.

`EnumIsA` is much simpler than the previous; it simply adds `is_XXX`
methods returning a boolean for whether the variant matches or not. Similar
to `EnumGetters`, the name is converted to lowercase and does **not** 
convert to snake\_case.

# License

This software is released under the Apache license 2.0. See the LICENSE file
for more details.
*/

#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::*;

macro_rules! copyable {
    ($name:expr) => {
        Ty::Path(None, Path { global: false, segments: vec![PathSegment { ident: Ident::new($name), parameters: PathParameters::none() }] })
    };
}

lazy_static! {
    static ref COPYABLE: Vec<Ty> = vec![
        copyable!("i8"),
        copyable!("i16"),
        copyable!("i32"),
        copyable!("i64"),
        copyable!("u8"),
        copyable!("u16"),
        copyable!("u32"),
        copyable!("u64"),
        copyable!("isize"),
        copyable!("usize"),
        copyable!("char"),
        copyable!("bool"),
        copyable!("f32"),
        copyable!("f64"),
        // TODO : string slices, function pointers
    ];
}

// TODO : map types for what a reference should return in its getter
// e.g. String -> &str in the getter

#[proc_macro_derive(EnumGetters)]
pub fn enum_getters(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();
    let mut getters = impl_enum_getters(&ast);
    getters.append(&mut impl_copyable_enum_getters(&ast));
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

fn impl_enum_getters(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    macro_rules! getter_filter {
        () => {
            variants.iter()
                .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
                .filter(|v| v.data.fields().len() == 1)
                .filter(|v| !COPYABLE.contains(&v.data.fields()[0].ty))
        };
    }
        

    let variant_names = getter_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = getter_filter!()
        .map(|v| v.ident.to_string().to_lowercase().into())
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

fn impl_copyable_enum_getters(ast: &DeriveInput) -> quote::Tokens {
    let ref name = ast.ident;

    let variants =
        if let Body::Enum(ref e) = ast.body { e }
        else { unreachable!() };

    macro_rules! getter_filter {
        () => {
            variants.iter()
                .filter(|v| if let VariantData::Tuple(_) = v.data { true } else { false })
                .filter(|v| v.data.fields().len() == 1)
                .filter(|v| COPYABLE.contains(&v.data.fields()[0].ty))
        };
    }
        

    let variant_names = getter_filter!()
        .map(|v| v.ident.clone())
        .collect::<Vec<Ident>>();

    let function_names = getter_filter!()
        .map(|v| v.ident.to_string().to_lowercase().into())
        .collect::<Vec<Ident>>();

    let function_name_strs = getter_filter!()
        .map(|v| v.ident.to_string().to_lowercase())
        .collect::<Vec<String>>();

    let variant_types = getter_filter!() 
        .map(|v| &v.data.fields()[0].ty)
        .map(|ty| ty.clone())
        .collect::<Vec<Ty>>();

    let getter_names = vec!(name.clone(); variant_types.len());
    
    quote! {
        #[allow(dead_code)]
        impl #name {
            #(
                fn #function_names(&self) -> #variant_types {
                    if let &#getter_names::#variant_names(v) = self {
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
        .map(|v| format!("is_{}", v.ident.to_string().to_lowercase()).into())
        .collect::<Vec<Ident>>();

    let variant_counts = is_a_filter!()
        .map(|v| vec!(Ident::new("_"); v.data.fields().len()))
        .collect::<Vec<_>>();

    let getter_names = vec!(name.clone(); variant_names.len());
    
    quote! {
        #[allow(dead_code)]
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
        .map(|v| format!("is_{}", v.ident.to_string().to_lowercase()).into())
        .collect::<Vec<Ident>>();

    let getter_names = vec!(name.clone(); variant_names.len());
    
    quote! {
        #[allow(dead_code)]
        impl #name {
            #(fn #function_names(&self) -> bool {
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
        .map(|v| format!("is_{}", v.ident.to_string().to_lowercase()).into())
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
            #(fn #function_names(&self) -> bool {
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
