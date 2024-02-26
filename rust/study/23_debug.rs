/*
--TODO: 调试(Debug)
    所有的类型，若想用std::fmt的格式化打印，都要求实现至少一个可打印的traits。仅有
  一些类型提供了自动实现，比如std库中的类型。所有其他类型都必须手动实现。

  fmt::Debug这个trait使这项工作变得相当简单。所有类型都能推导(derive，即自动创建)
fmt::Debug的实现。但是fmt::Display需要手动实现。
*/

// -> 这个结构体不能使用`fmt::Display`或`fmt::Debug`来进行打印
struct UnPrinttable(i32);

// `derive`属性会自动创建所需的实现，使这个`struct`能使用`fmt::Debug`打印。
#[derive(Debug)]
struct DebugPrintable(i32);

// -> 所有std类型都天生可以使用{:?}来打印:
// -> 将`Structure`放到结构体`Deep`中。然后使`Deep`也能够打印。
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a>
{
    name: &'a str,
    age: u8
}

fn main()
{
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor = "actor's");

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));


    // -> 但这牺牲了一些美感。Rust也通过{:#?}提供了`美化打印`的功能
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // -> 美化打印
    println!("{:#?}", peter);
}