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
