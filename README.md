# enum-methods

![](https://travis-ci.org/alekratz/enum-methods.svg?branch=master)

Enum getter/is\_XXX method generation.

## Please note that this crate is unstable and is subject to change frequently.

I will attempt to prevent *seriously* breaking changes after we hit 0.1.0.

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

`EnumGetters` has a couple of limitations.

* Any enum variant which has exactly 1 member will have a getter generated for
  it. All other variants are ignored.
* Enums which derive from `EnumGetters` must also derive from `Debug` - this
  is for when a method is called for the wrong variant and needs to `panic!`.

`EnumIsA` is much simpler than the previous; it simply adds `is_XXX`
methods returning a boolean for whether the variant matches or not.

**For both methods, all names are automatically converted to snake_case**.

# License

This software is released under the Apache license 2.0. See the LICENSE file
for more details.
