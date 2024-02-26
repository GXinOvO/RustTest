use derive_with::With;

#[derive(With, Default, Debug)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}

fn main()
{
    #[derive(Debug)]
    struct MyString(pub String);
    impl From<MyString> for String {
        fn from(value: MyString) -> Self {
            value.0
        }
    }

    let my_string = MyString("Hello, World!".to_string());
    let string: String = String::from(my_string);
    println!("my_string: {:#?}", string);

    let foo = Foo::default().with_a(1).with_b(MyString("1".to_string()));
    println!("foo: {:#?}", foo);
}