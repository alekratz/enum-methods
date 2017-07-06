# enum-methods

Enum getter/is\_XXX method generation.

# [crates.io](https://crates.io/crates/enum-methods)

# [docs.rs](https://docs.rs/enum-methods/0.0.1/enum_methods/)

# Usage

In your `Cargo.toml`, add this line under your `[dependencies]` section:

```toml,no_run
enum-methods = "0.0.1"
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

```
#[derive(EnumGetters, Debug)]
enum MyEnum {
    Foo(i64),
    Bar(char),
    Baz(String),
}

fn main() {
    let foo = MyEnum::foo(42);
    assert_eq!(foo.foo(), 42);  // success!
}
```

# Requirements and gotchas

Right now, `enum-methods` has only two derivable options:
* `EnumGetters`
* `EnumIsA`

`EnumGetters` has a couple of limitations. First, each enum variant must
have exactly 0 or 1 members. Enum variants with 0 members do not get a
method generated for it. Generated methods simply use the lower-case
version of their variant name. **These names are not converated to
snake_case.** Additionally, enums which derive from `EnumGetters` must also
derive from `Debug` - this is for when a method is called for the wrong
variant and needs to `panic!`.

`EnumIsA` is much simpler than the previous; it simply adds `is_XXX`
methods returning a boolean for whether the variant matches or not. Similar
to `EnumGetters`, the name is converted to lowercase and does **not** 
convert to snake\_case.

# License

This software is released under the Apache license 2.0. See the LICENSE file
for more details.
