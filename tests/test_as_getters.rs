#[macro_use]
extern crate enum_methods;

#[test]
fn test_as_getters() {
    #[derive(EnumAsGetters, Debug)]
    enum MyEnum {
        Foo(i64),
        Bar(bool),
        Baz(String),
        Tup(i32, String, Vec<bool>),
    }

    let foo = MyEnum::Foo(42);
    let bar = MyEnum::Bar(false);
    let baz = MyEnum::Baz("hurry boy, it's waiting there for you".to_string());
    let tup = MyEnum::Tup(42, String::from("Hello, Tuple, my old friend!"), vec![true, false, true]);
    assert_eq!(*foo.as_foo(), 42);
    assert_eq!(*bar.as_bar(), false);
    assert_eq!(baz.as_baz(), "hurry boy, it's waiting there for you");
    assert_eq!(tup.as_tup(), (&42, &String::from("Hello, Tuple, my old friend!"), &vec![true, false, true]));
}

#[test]
fn test_as_getter_names() {
    #[derive(EnumAsGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
    }

    let first = MyEnum::FooBar(true);
    let second = MyEnum::BarBaz(
        "there's nothing that a hundred men or more could ever do".to_string(),
    );
    assert_eq!(*first.as_foo_bar(), true);
    assert_eq!(
        second.as_bar_baz(),
        "there's nothing that a hundred men or more could ever do"
    );
}

#[test]
fn test_getter_structs() {
    #[derive(EnumAsGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
        SomeStruct { foo: i32 }, // should be skipped
    }

    impl MyEnum {
        pub fn as_some_struct(&self) -> &i32 {
            if let &MyEnum::SomeStruct { ref foo } = self {
                foo
            } else {
                unreachable!()
            }
        }
    }

    let first = MyEnum::FooBar(true);
    let second = MyEnum::BarBaz(
        "there's nothing that a hundred men or more could ever do".to_string(),
    );
    let third = MyEnum::SomeStruct { foo: 42 };
    assert_eq!(*first.as_foo_bar(), true);
    assert_eq!(
        second.as_bar_baz(),
        "there's nothing that a hundred men or more could ever do"
    );
    assert_eq!(*third.as_some_struct(), 42);
}
