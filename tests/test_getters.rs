#[macro_use]
extern crate enum_methods;

#[test]
fn test_getters() {
    #[derive(EnumGetters, Debug)]
    enum MyEnum {
        Foo(i64),
        Bar(bool),
        Baz(String),
    }

    let foo = MyEnum::Foo(42);
    let bar = MyEnum::Bar(false);
    let baz = MyEnum::Baz("hurry boy, it's waiting there for you".to_string());
    assert_eq!(foo.foo(), 42);
    assert_eq!(bar.bar(), false);
    // note that this returns a &String by default
    assert_eq!(baz.baz(), "hurry boy, it's waiting there for you");
}

#[test]
fn test_getter_names() {
    #[derive(EnumGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
    }

    let first = MyEnum::FooBar(true);
    let second = MyEnum::BarBaz("there's nothing that a hundred men or more could ever do".to_string());
    assert_eq!(first.foobar(), true);
    assert_eq!(second.barbaz(), "there's nothing that a hundred men or more could ever do");
}

#[test]
fn test_getter_structs() {
    #[derive(EnumGetters, Debug)]
    enum MyEnum {
        FooBar(bool),
        BarBaz(String),
        SomeStruct { foo: i32 },  // should be skipped
    }

    impl MyEnum {
        pub fn somestruct(&self) -> i32 {
            if let &MyEnum::SomeStruct { foo } = self {
                foo
            }
            else {
                panic!("called MyEnum::somestruct() on {:?}", self);
            }
        }
    }

    let first = MyEnum::FooBar(true);
    let second = MyEnum::BarBaz("there's nothing that a hundred men or more could ever do".to_string());
    let third = MyEnum::SomeStruct { foo: 42 };
    assert_eq!(first.foobar(), true);
    assert_eq!(second.barbaz(), "there's nothing that a hundred men or more could ever do");
    assert_eq!(third.somestruct(), 42);
}
