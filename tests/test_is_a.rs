#[macro_use]
extern crate enum_methods;

#[test]
fn test_is_a() {
    #[derive(EnumIsA, Debug)]
    enum MyEnum {
        Foo,
        Bar(bool, i32),
        Baz(String),
    }

    let first = MyEnum::Foo;
    let second = MyEnum::Bar(false, -3);
    let third = MyEnum::Baz("it's gonna take some time to do the things we never had".to_string());

    assert!(first.is_foo());
    assert!(second.is_bar());
    assert!(third.is_baz());
}

#[test]
fn test_is_a_names() {
    #[derive(EnumIsA, Debug)]
    enum MyEnum {
        FooBar,
        BarBaz(bool, i32),
        Baz(String),
    }

    let first = MyEnum::FooBar;
    let second = MyEnum::BarBaz(false, -3);
    let third = MyEnum::Baz("it's gonna take some time to do the things we never had".to_string());

    assert!(first.is_foobar());
    assert!(second.is_barbaz());
    assert!(third.is_baz());
}
