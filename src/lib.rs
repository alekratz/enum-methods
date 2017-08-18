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
* [docs.rs](https://docs.rs/enum-methods/0.0.7/enum_methods/)

# Usage

In your `Cargo.toml`, add this line under your `[dependencies]` section:

```toml,no_run
enum-methods = "0.0.7"
```

To use, simply derive and call methods (see the example below).

# Why?

Usually when you write an enum with one or zero values, you might want to
add a set of getters for them. As such:

```rust
#[derive(Debug)]
enum MyEnum {
    FooBarBaz(i64),
    BazBarFoo(String),
    // ... and others
}

impl MyEnum {
    pub fn foo_bar_baz(&self) -> i64 {
        if let &MyEnum::FooBarBaz(i) = self {
            i
        }
        else {
            panic!("called MyEnum::FooBarBaz() on {:?}", self)
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

#[derive(EnumIntoGetters, EnumAsGetters, EnumIsA, Debug)]
enum MyEnum {
    FooBarBaz(i64),
    BazBarFoo(String),
    // ... and others
}

fn main() {
    let my_foo = MyEnum::FooBarBaz(42);
    // EnumIsA - creates is_* methods for every member
    if my_foo.is_foo_bar_baz() {
        // EnumAsGetters - gets a reference to the enum, panicking if it is
        // not the specified variant
        assert_eq!(*my_foo.as_foo_bar_baz(), 42);
        // EnumIntoGetters - consumes the enum, yielding its owned value,
        // and panicking if it is not the specified variant
        assert_eq!(my_foo.into_foo_bar_baz(), 42);
    }
}
```

# Requirements and gotchas

Right now, `enum-methods` has four derivable options:

* `EnumAsGetters` for generating `as_*` methods, which return a reference.
* `EnumIntoGetters` for generating `into_*` methods, which consume the enum
   and returns the data held by the variant.
* `EnumToGetters` for generating `to_*` methods, which returns a clone of
   the data held by the variant.
* `EnumIsA` for generating `is_*` methods, which return a boolean indicating
   whether the enum matches that variant.

`EnumAsGetters`, `EnumIntoGetters`, and `EnumToGetters` have some limitations.

* Any enum variant which has exactly 1 member will have a getter generated for
  it. All other variants are ignored.
* Enums which derive from `EnumIntoGetters` must also derive from `Debug` - this
  is for when a method is called for the wrong variant and needs to `panic!`.

Furthermore, `EnumToGetters` is *only* for enums whose variants implement
`Clone`. There is not yet support for th

`EnumIsA` is much simpler than the previous; it simply adds `is_*`
methods returning a boolean for whether the variant matches or not.

**For all generated methods, all names are automatically converted to
snake_case**.

# License

This software is released under the Apache license 2.0. See the LICENSE file
for more details.

*/

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

mod getters;
mod is_a;
mod util;

use getters::*;
use is_a::*;
use proc_macro::TokenStream;
use syn::*;

// TODO : map types for what a reference should return in its getter
// e.g. String -> &str in the getter

#[proc_macro_derive(EnumAsGetters)]
#[doc(hidden)]
pub fn enum_as_getters(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();
    let getters = impl_enum_as_getters(&ast);
    //panic!("{:#?}", getters);
    getters.parse().unwrap()
}

#[proc_macro_derive(EnumIntoGetters)]
#[doc(hidden)]
pub fn enum_into_getters(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();
    let getters = impl_enum_into_getters(&ast);
    getters.parse().unwrap()
}

#[proc_macro_derive(EnumToGetters)]
#[doc(hidden)]
pub fn enum_to_getters(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();
    let getters = impl_enum_to_getters(&ast);
    getters.parse().unwrap()
}

#[proc_macro_derive(EnumIsA)]
#[doc(hidden)]
pub fn enum_is_a(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_derive_input(&s).unwrap();
    let mut gen = impl_enum_is_a(&ast);
    gen.append(&mut impl_struct_enum_is_a(&ast));
    gen.append(&mut impl_unit_enum_is_a(&ast));
    gen.parse().unwrap()
}

