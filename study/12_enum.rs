#[derive(Debug)]

enum Book {
    Papery(u32), 
    Electronic(String),
}

/*
    也可以为属性命名，可以用结构体语法:
enum Book 
{
    Papery { index: u32 },
    Electronic { url: String },
}
*/

fn main()
{
    enum Book_str 
    {
        Papery {index: u32},
        Electronic {url: String},
    }
    let book = Book_str::Papery{index: 1001};
    let ebook = Book_str::Electronic{url: String::from("url...")};


    /*
        --TODO: match语法
            枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。基于这个原理，往往枚举类
          最终都会被分支结构处理(许多语言中的switch)。switch语法很经典，但在Rust中并不支持，很多语言摒弃
          switch的原因都是因为switch容易存在因忘记添加break而产生的串接运行问题，Java和C#这类语言通过安
          全检查杜绝这种情况出现。

        match 块也可以当作函数表达式来对待，他也是可以有返回值的，但是所有返回值表达式的类型必须一样
        match 枚举类实例 
        {
            分类1 => 返回值表达式,
            分类2 => 返回值表达式
            ...
        }

          match除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、字符和字符串切片引用(&str)类型的数据进
        行分支选择。其中，浮点数类型被分支选择虽然合法，但不推荐这样使用
     */
    match book
    {
        Book_str::Papery{ index } => {
            println!("Papery book {}", index);
        },
        Book_str::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
    // println!("{:?}", book);

    let t = "abc";
    match t 
    {
        "abc" => println!("Yes"),
        _ => (),
    }

    /*
        --TODO: Option枚举类
            这个类用于填补Rust不支持null引用的空白。
      null 经常在开发者把一切都当作不是 null 的时候给予程序致命一击：毕竟只要出现一个这样的错误，
    程序的运行就要彻底终止。为了解决这个问题，很多语言默认不允许 null，但在语言层面支持 null 的出
    现（常在类型前面用 ? 符号修饰）。

      Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 
    引入了 Option 枚举类：
        enum Option<T>
        {
            Some(T),
            None,
        }
     */
    // 如果你想定义一个可以为空值的类，你可以这样:
    let opt = Option::Some("Hello");

    // 如果你想针对opt执行某些操作，你必须先判断它是否是Option::None
    match opt
    {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    /*
      如果你的变量刚开始是空值，你体谅一下编译器，它怎么知道值不为空的时候变量是什么类型的呢？
    所以初始值为空的 Option 必须明确类型：
     */
    let opt: Option<&str> = Option::None;
    match opt 
    {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    /*
    由于Option是Rust编译器默认引入的，在使用时可以省略Option::直接写None或者Some()
     */
    let m = Some(64);
    match m 
    {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }

    /*
        --TODO: if let 语法格式如下:
        if let 匹配值 = 源变量 {
            语句块
        }
     */

    let mbook = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = mbook 
    {
        println!("Papery {}", index);
    }
    else
    {
        println!("Not papery book");
    }
}