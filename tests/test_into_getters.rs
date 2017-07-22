#[macro_use]
extern crate enum_methods;

#[test]
fn test_into_getters() {
    #[derive(EnumIntoGetters, Debug)]
    enum MyEnum {
        Foo(i64),
        Bar(bool),
        Baz(String),
    }

    let foo = MyEnum::Foo(42);
    let bar = MyEnum::Bar(false);
    let baz = MyEnum::Baz("hurry boy, it's waiting there for you".to_string());
    assert_eq!(foo.into_foo(), 42);
    assert_eq!(bar.into_bar(), false);
    assert_eq!(baz.into_baz(), "hurry boy, it's waiting there for you");
}

#[test]
fn test_into_getter_names() {
    #[derive(EnumIntoGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
    }

    let first = MyEnum::FooBar(true);
    let second = MyEnum::BarBaz("there's nothing that a hundred men or more could ever do".to_string());
    assert_eq!(first.into_foo_bar(), true);
    assert_eq!(second.into_bar_baz(), "there's nothing that a hundred men or more could ever do");
}

#[test]
fn test_getter_structs() {
    #[derive(EnumIntoGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
        SomeStruct { foo: i32 },  // should be skipped
    }

    impl MyEnum {
        pub fn into_some_struct(self) -> i32 {
            if let MyEnum::SomeStruct { foo } = self {
                foo
            }
            else {
                unreachable!()
            }
        }
    }

    let first = MyEnum::FooBar(true);
    let second = MyEnum::BarBaz("there's nothing that a hundred men or more could ever do".to_string());
    let third = MyEnum::SomeStruct { foo: 42 };
    assert_eq!(first.into_foo_bar(), true);
    assert_eq!(second.into_bar_baz(), "there's nothing that a hundred men or more could ever do");
    assert_eq!(third.into_some_struct(), 42);
}
